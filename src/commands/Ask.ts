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

import Command from "./Command";
import CommandContext from "contexts/Command";
import Context from "contexts/Context";
import Lang from "helpers/Lang";
import { BotCommand } from "libraries/telegram/types/BotCommand";
import { getChatByTelegramId } from "services/Chats";

export default class Ask extends Command {

    /**
     * Commands list.
     *
     * @author Marcos Leandro
     * @since  2024-05-03
     *
     * @var {BotCommand[]}
     */
    public readonly commands: BotCommand[] = [
        { command: "ask", description: "Shows the ask to ask answering status. Manages it with [on | off]." }
    ];

    /**
     * The constructor.
     *
     * @author Marcos Leandro
     * @since  2022-09-12
     */
    public constructor() {
        super();
    }

    /**
     * Run the command.
     *
     * @author Marcos Leandro
     * @since  2023-06-12
     *
     * @param {CommandContext} command
     * @param {Context}        context
     *
     * @return {Promise<void>}
     */
    public async run(command: CommandContext, context: Context): Promise<void> {

        this.context = context;
        if (!await this.context.getUser()?.isAdmin()) {
            return Promise.resolve();
        }

        this.context.getMessage()?.delete();

        let chat;
        const chatId = this.context.getChat()?.getId();

        if (chatId) {
            chat = await getChatByTelegramId(chatId);
        }

        Lang.set(chat?.language ?? "en");

        const replyToMessage = this.context.getMessage()?.getReplyToMessage();
        if (replyToMessage) {
            await replyToMessage.reply(Lang.get("askToAskLink"));
            return Promise.resolve();
        }

        this.context.getChat()?.sendMessage(Lang.get("askToAskLink"));
    }
}
