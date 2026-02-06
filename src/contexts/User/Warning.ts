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

import User from "../User";
import Chat from "../Chat";
import Lang from "helpers/Lang";
import Message from "../Message";
import { addWarning } from "services/Warnings";
import { getChatByTelegramId } from "services/Chats";
import { getUserWarnings } from "services/Warnings";
import { InlineKeyboardButton } from "libraries/telegram/types/InlineKeyboardButton";

export default class Warning {

    /**
     * User context.
     *
     * @author Marcos Leandro
     * @since  2026-02-06
     */
    private readonly user: User;

    /**
     * Chat context.
     *
     * @author Marcos Leandro
     * @since  2026-02-06
     */
    private readonly chat: Chat;

    /**
     * The constructor.
     *
     * @author Marcos Leandro
     * @since  2026-02-06
     *
     * @param user
     */
    public constructor(user: User, chat: Chat) {
        this.user = user;
        this.chat = chat;
    }

    /**
     * Gets the user's warnings.
     *
     * @author Marcos Leandro
     * @since  2026-02-06
     */
    public async get(): Promise<void> {
        await this.sendMessage();
    }

    /**
     * Warns the user.
     *
     * @author Marcos Leandro
     * @since  2026-02-06
     *
     * @param reason
     * @param message
     */
    public async warn(reason?: string, message?: Message): Promise<void> {

        if (this.user.getId() === parseInt(process.env.TELEGRAM_USER_ID!)) {
            message ? await message.reply(Lang.get("selfWarnMessage")) : await this.chat.sendMessage(Lang.get("selfWarnMessage"));
            return Promise.resolve();
        }

        if (await this.user.isAdmin()) {
            message ? await message.reply(Lang.get("adminWarnMessage")) : await this.chat.sendMessage(Lang.get("adminWarnMessage"));
            return Promise.resolve();
        }

        const reasonMessage = reason?.length ? reason : Lang.get("reasonUnknown");
        await addWarning(this.user, this.chat, reasonMessage);
        await this.checkBan();
        await this.sendMessage();
    }

    /**
     * Bans the user if necessary.
     *
     * @author Marcos Leandro
     * @since  2026-02-06
     */
    private async checkBan(): Promise<void> {

        const chat = await getChatByTelegramId(this.chat.getId());
        const warnings = await getUserWarnings(this.user, this.chat);
        if (!chat || !warnings.length) {
            return Promise.resolve();
        }

        const warningLimit = chat.chat_configs?.warnings ?? 3;
        if (warnings.length < warningLimit) {
            return Promise.resolve();
        }

        await this.user.ban();
    }

    /**
     * Sends the warning messages.
     *
     * @author Marcos Leandro
     * @since  2026-02-06
     */
    private async sendMessage(): Promise<void> {

        const chat = await getChatByTelegramId(this.chat.getId());
        if (!chat) {
            return Promise.resolve();
        }

        const warnings = await getUserWarnings(this.user, this.chat);
        const warningLimit = chat.chat_configs?.warnings ?? 3;
        const message = await this.getWarningMessage(warnings, warningLimit);
        const options = this.getMessageOptions(warnings);

        await this.chat.sendMessage(message, options);
    }

    /**
     * Returns the warning message.
     *
     * @author Marcos Leandro
     * @since  2026-02-06
     *
     * @param warnings
     * @param warningsLimit
     *
     * @return Warning message.
     */
    protected async getWarningMessage(warnings: Record<string, any>[], warningsLimit: number): Promise<string> {

        const username = this.user.getFirstName() ?? this.user.getUsername();

        let langIndex = warnings.length === 1 ? "warningSigleMessage" : "warningPluralMessage";
        langIndex = warnings.length >= warningsLimit ? "warningBanMessage" : langIndex;
        langIndex = warnings.length === 0 ? "warningNoneMessage" : langIndex;

        let message = Lang.get(langIndex)
            .replace("{userid}", this.user.getId().toString())
            .replace("{username}", username!)
            .replace("{warnings}", warnings.length.toString() + "/" + warningsLimit.toString());

        for (let i = 0, length = warnings.length; i < length; i++) {
            message += ` • ${warnings[i].reason}\n`;
        }

        return Promise.resolve(message);
    }

    /**
     * Returns the message options.
     *
     * @author Marcos Leandro
     * @since  2026-02-06
     *
     * @param warnings
     *
     * @return {Record<string, any>}
     */
    private getMessageOptions(warnings: Record<string, any>[]): Record<string, any> {

        const options: Record<string, any> = {
            parse_mode: "HTML"
        };

        if (!warnings.length) {
            return options;
        }

        /* Get the last warning */
        const lastWarning = warnings[warnings.length - 1];
        const lastWarningRemovalButton: InlineKeyboardButton = {
            text: Lang.get("lastWarningRemovalButton"),
            callback_data: JSON.stringify({
                c: "warning",
                d: `${this.user.getId()},${this.chat.getId()},${lastWarning.id}`
            })
        };

        const allWarningsRemovalButton: InlineKeyboardButton = {
            text: Lang.get("warningsRemovalButton"),
            callback_data: JSON.stringify({
                c: "warning",
                d: `${this.user.getId()},${this.chat.getId()}`
            })
        };

        options.replyMarkup = {
            inline_keyboard: [[lastWarningRemovalButton], [allWarningsRemovalButton]]
        };

        return options;
    }
}
