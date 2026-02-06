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
import Context from "contexts/Context";
import CommandContext from "contexts/Command";
import Lang from "helpers/Lang";
import User from "contexts/User";
import { BotCommand } from "libraries/telegram/types/BotCommand";
import { getChatByTelegramId } from "services/Chats";

export default class Warn extends Command {

    /**
     * Commands list.
     *
     * @author Marcos Leandro
     * @since  2024-05-03
     *
     * @var {BotCommand[]}
     */
    public readonly commands: BotCommand[] = [
        { command: "warn", description: "Gives the user a warning." },
        { command: "delwarn", description: "Gives the user a warning and deletes their's message." }
    ];

    /**
     * Command context.
     *
     * @author Marcos Leandro
     * @since  2023-06-14
     *
     * @var {CommandContext}
     */
    private command?: CommandContext;

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
     * Executes the command.
     *
     * @author Marcos Leandro
     * @since  2023-06-07
     *
     * @param {CommandContext} command
     * @param {Context}        context
     */
    public async run(command: CommandContext, context: Context): Promise<void> {

        this.context = context;

        if (!this.context) {
            return Promise.resolve();
        }

        if (!await this.context.getUser()?.isAdmin()) {
            return Promise.resolve();
        }

        if (this.context.getChat()?.getType() === "private") {
            return Promise.resolve();
        }

        this.command = command;
        const chatContext = this.context.getChat();

        const chat = await getChatByTelegramId(chatContext!.getId());
        if (!chat) {
            return Promise.resolve();
        }

        const params = this.command.getParams();
        if (!params?.length) {
            return Promise.resolve();
        }

        Lang.set(chat.language || "en");

        const replyToMessage = this.context.getMessage()?.getReplyToMessage();
        if (replyToMessage && command.getCommand() === "delwarn") {
            replyToMessage.delete();
        }

        const users: User[] = [];
        if (replyToMessage) {
            const user = replyToMessage.getUser();
            user && (users.push(user));
        }

        const mentions = await this.context.getMessage()?.getMentions() || [];
        for (const mention of mentions) {
            users.push(mention);
            params.shift();
        }

        for (const user of users) {
            await user.warn(params.join(" "));
        }
    }
}
