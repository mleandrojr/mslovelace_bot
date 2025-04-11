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
import { getUserByTelegramId } from "services/Users";

export default class getUserLink extends Command {

    /**
     * Commands list.
     *
     * @author Marcos Leandro
     * @since  2025-04-11
     *
     * @var {BotCommand[]}
     */
    public readonly commands: BotCommand[] = [
        { command: "getuserlink", description: "Sends the user deep link for the given user ID." }
    ];

    /**
     * The constructor.
     *
     * @author Marcos Leandro
     * @since  2025-04-11
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
        const params = command.getParams() || [];
        if (params.length === 0) {
            return;
        }

        await this.getUserInfo(parseInt(params[0]));
    }

    /**
     * Gets the user info and sends the message.
     *
     * @author Marcos Leandro
     * @since  2025-04-11
     *
     * @param userId
     */
    private async getUserInfo(userId: number): Promise<void> {

        const chat = await getChatByTelegramId(this.context!.getChat()!.getId());
        const user = await getUserByTelegramId(userId);

        Lang.set(chat?.language || "en");

        const message = Lang.get("userLink")
            .replace("{userid}", userId.toString())
            .replace("{username}", user?.first_name ?? user?.username ?? userId.toString());

        this.context!.getChat()!.sendMessage(message, { parse_mode: "HTML" });
    }
}
