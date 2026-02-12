/**
 * Ada Lovelace Telegram Bot
 *
 * This file is part of Ada Lovelace Telegram Bot.
 * You are free to modify and share this project or its files.
 *
 * @package  mslovelace_bot
 * @author   Marcos Leandro <mleandrojr@yggdrasill.com.br>
 * @license  GPLv3 <http://www.gnu.org/licenses/gpl-3.0.en.html>
 */

import Action from "./Action";
import Context from "contexts/Context";
import Lang from "helpers/Lang";
import Log from "helpers/Log";
import { ChatWithConfigs } from "types/ChatWithConfigs";
import { BlockedTerm } from "types/BlockedTerm";
import { getBlockedTermsByChatId, getChatByTelegramId } from "../services/Chats";

export default class BlockedTerms extends Action {

    /**
     * Indicates whether the action reached a terminal status.
     *
     * @author Marcos Leandro
     * @since  2026-02-06
     */
    private isTerminalStatus = false;

    /**
     * The constructor.
     *
     * @author Marcos Leandro
     * @since  2026-02-06
     *
     * @param context
     */
    public constructor(context: Context) {
        super(context, "sync");
    }

    /**
     * Action routines.
     *
     * @author Marcos Leandro
     * @since  2026-02-06
     */
    public async run(): Promise<void> {

        if (await this.context.getUser()?.isAdmin()) {
            return;
        }

        const chatId = this.context.getChat()?.getId();
        if (!chatId) {
            throw new Error("Chat not found.");
        }

        const chat = await getChatByTelegramId(chatId);
        if (!chat) {
            throw new Error(`Chat ${chatId} not found.`);
        }

        Lang.set(chat.language || "en");

        try {
            const text = this.context.getMessage()?.getText() ?? "";
            const blocked = await getBlockedTermsByChatId(chat.id);
            for (const term of blocked) {
                await this.checkBlockedTerm(chat, term, text);
                if (this.isTerminalStatus) {
                    break;
                }
            }

        } catch (err: Error) {
            Log.save(err.toString());
        }
    }

    /**
     * Checks a blocked term in the given text.
     *
     * @author Marcos Leandro
     * @since  2026-02-06
     *
     * @param term
     * @param text
     */
    private async checkBlockedTerm(chat: ChatWithConfigs, term: BlockedTerm, text: string): Promise<void> {

        if (!text?.toLowerCase().includes(term.term.toLowerCase())) {
            return;
        }

        await this.context?.getMessage()?.delete().then(async () => {

            if (term.action === "mute") {
                await this.context!.getUser()?.restrict();
            }

            if (term.action === "ban") {
                await this.context!.getUser()!.ban();
                this.isTerminalStatus = true;
                return;
            }

            if (term.action === "warn") {
                const reasonMessage = Lang.get("warnBlockedTerm");
                await this.context!.getUser()?.warn(reasonMessage, undefined);
                this.isTerminalStatus = true;
            }
        });
    }
}
