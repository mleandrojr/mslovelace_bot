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

import Chat from "contexts/Chat";
import Command from "../Command";
import User from "contexts/User";
import Lang from "helpers/Lang";
import { InlineKeyboardButton } from "libraries/telegram/types/InlineKeyboardButton";
import { InlineKeyboardMarkup } from "libraries/telegram/types/InlineKeyboardMarkup";
import { getChatByTelegramId } from "services/Chats";
import { getUserWarnings } from "services/Warnings";

export default class Base extends Command {

    /**
     * The constructor.
     *
     * @author Marcos Leandro
     * @since  2024-04-22
     */
    public constructor() {
        super();
    }

    /**
     * Returns the warning message.
     *
     * @author Marcos Leandro
     * @since  2024-04-22
     *
     * @param contextUser
     * @param warnings
     * @param warningsLimit
     *
     * @return Warning message.
     */
    protected async getWarningMessage(contextUser: User, warnings: Record<string, any>[], warningsLimit: number): Promise<string> {

        const username = contextUser.getFirstName() ?? contextUser.getUsername();

        let langIndex = warnings.length === 1 ? "warningSigleMessage" : "warningPluralMessage";
        langIndex = warnings.length >= warningsLimit ? "warningBanMessage" : langIndex;
        langIndex = warnings.length === 0 ? "warningNoneMessage" : langIndex;

        let message = Lang.get(langIndex)
            .replace("{userid}", contextUser.getId().toString())
            .replace("{username}", username!)
            .replace("{warnings}", warnings.length.toString() + "/" + warningsLimit.toString());

        for (let i = 0, length = warnings.length; i < length; i++) {
            message += ` • ${warnings[i].reason}\n`;
        }

        return Promise.resolve(message);
    }

    /**
     * Sends the warning messages.
     *
     * @author Marcos Leandro
     * @since  2024-04-22
     *
     * @param usersContext
     * @param chatContext
     *
     * @return
     */
    protected async sendWarningMessages(usersContext: User[], chatContext: Chat): Promise<void> {

        const chat = await getChatByTelegramId(chatContext.getId());
        if (!chat) {
            return Promise.resolve();
        }

        Lang.set(chat.language || "en");

        const warnings = await getUserWarnings(usersContext[0], chatContext);
        const warningLimit = chat.chat_configs?.warnings ?? 3;
        const messages = [];

        for (let i = 0, length = usersContext.length; i < length; i++) {
            const contextUser = usersContext[i];
            messages.push(await this.getWarningMessage(contextUser, warnings, warningLimit));
        }

        if (!messages.length) {
            return Promise.resolve();
        }

        const message = messages.join("\n-----\n");
        const options = this.getMessageOptions(usersContext, warnings);

        await this.context?.getChat()?.sendMessage(message, options);
    }

    /**
     * Returns the message options.
     *
     * @author Marcos Leandro
     * @since  2024-04-22
     *
     * @param users
     * @param warnings
     *
     * @return {Record<string, any>}
     */
    private getMessageOptions(users: User[], warnings: Record<string, any>[]): Record<string, any> {

        const options: Record<string, any> = {
            parse_mode: "HTML"
        };

        if (users.length !== 1) {
            return options;
        }

        if (!warnings.length) {
            return options;
        }

        /* Get the last warning */
        const lastWarning = warnings[warnings.length - 1];
        const lastWarningRemovalButton: InlineKeyboardButton = {
            text: Lang.get("lastWarningRemovalButton"),
            callback_data: JSON.stringify({
                c: "warning",
                d: `${users[0].getId()},${this.context?.getChat()?.getId()},${lastWarning.id}`
            })
        };

        const allWarningsRemovalButton: InlineKeyboardButton = {
            text: Lang.get("warningsRemovalButton"),
            callback_data: JSON.stringify({
                c: "warning",
                d: `${users[0].getId()},${this.context?.getChat()?.getId()}`
            })
        };

        options.replyMarkup = {
            inline_keyboard: [[lastWarningRemovalButton], [allWarningsRemovalButton]]
        };

        return options;
    }
}
