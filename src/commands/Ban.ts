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
import Log from "helpers/Log";
import Message from "contexts/Message";
import UserContext from "contexts/User";
import { BotCommand } from "libraries/telegram/types/BotCommand";
import { getChatByTelegramId } from "services/Chats";
import { getUserByTelegramId, ban } from "services/Users";
import { User as UserType } from "../libraries/telegram/types/User";

export default class Ban extends Command {

    /**
     * Commands list.
     *
     * @author Marcos Leandro
     * @since  2024-05-03
     *
     * @var {BotCommand[]}
     */
    public readonly commands: BotCommand[] = [
        { command: "ban", description: "Bans an user from group." },
        { command: "sban", description: "Silently bans an user from group and deletes their message." },
        { command: "dban", description: "Bans an user from group and deletes their message." },
        { command: "sdban", description: "Silently bans an user from group and deletes their message." }
    ];

    /**
     * Command context.
     *
     * @author Marcos Leandro
     * @since  2023-07-04
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

        this.command = command;
        this.context = context;

        if (!await this.context.getUser()?.isAdmin()) {
            return Promise.resolve();
        }

        this.context.getMessage()?.delete();

        let params = command.getParams() || [];

        const replyToMessage = this.context.getMessage()?.getReplyToMessage();
        if (replyToMessage && ["dban", "sdban"].includes(command.getCommand())) {
            replyToMessage.delete();
        }

        if (replyToMessage) {
            this.banByReply(replyToMessage, params.join(" ").trim());
            return Promise.resolve();
        }

        const mentions = await this.context.getMessage()!.getMentions();
        params = params.filter((param) => !param.startsWith("@"));

        mentions.forEach(async (mention) => {
            await this.banByMention(mention, params.join(" ").trim());
        });

        const userId = parseInt(params[0]);
        if (userId === Number(params[0])) {
            params.shift();
            this.banByUserId(userId, params.join(" ").trim());
        }
    }

    /**
     * Bans an user by message reply.
     *
     * @author Marcos Leandro
     * @since  2023-06-07
     *
     * @returns void
     */
    private async banByReply(replyToMessage: Message, reason: string): Promise<void> {

        if (!await replyToMessage.getUser()?.ban()) {
            const message = Lang.get("banErrorMessage");
            return this.context?.getChat()?.sendMessage(message, { parse_mode : "HTML" });
        }

        const user = replyToMessage.getUser();
        user && (this.saveBan(user, reason));
    }

    /**
     * Bans an user by mention reply.
     *
     * @author Marcos Leandro
     * @since  2023-06-07
     *
     * @returns void
     */
    private async banByMention(mention: UserContext, reason: string): Promise<void> {

        const ban = await mention.ban();
        if (!ban) {
            const message = Lang.get("banErrorMessage");
            return this.context?.getChat()?.sendMessage(message, { parse_mode : "HTML" });
        }

        await this.saveBan(mention, reason);
    }

    /**
     * Bans the user by Telegram ID.
     *
     * @author Marcos Leandro
     * @since  2023-07-04
     *
     * @param userId
     * @param reason
     */
    private async banByUserId(userId: number, reason: string): Promise<void> {

        const user = await getUserByTelegramId(userId);
        if (!user) {
            return Promise.resolve();
        }

        const userType: UserType = {
            id: userId,
            is_bot: user.is_bot,
            first_name: user?.first_name ?? "",
            last_name: user?.last_name ?? "",
            username: user?.username ?? userId.toString()
        };

        const chat = this.context?.getChat();
        if (!chat) {
            return Promise.resolve();
        }

        const contextUser = new UserContext(userType, chat);
        if (await contextUser.ban()) {
            this.saveBan(contextUser, reason);
        }
    }

    /**
     * Saves the ban.
     *
     * @author Marcos Leandro
     * @since  2023-07-04
     *
     * @param {UserContext} contextUser User object.
     */
    private async saveBan(contextUser: UserContext, reason: string): Promise<void> {

        const chatId = this.context?.getChat()?.getId();
        if (!chatId) {
            return Promise.resolve();
        }

        const chat = await getChatByTelegramId(chatId);
        const user = await getUserByTelegramId(contextUser.getId());

        if (!user || !chat) {
            return Promise.resolve();
        }

        Lang.set(chat.language || "en");

        try {

            await ban(user.id, chat.id, chat.federation_id ?? null, reason);
            if (["sban", "sdban"].includes(<string> this.command?.getCommand())) {
                return;
            }

            const message = Lang.get("bannedMessage")
                .replace("{userid}", contextUser.getId().toString())
                .replace("{username}", contextUser.getFirstName() ?? contextUser.getUsername() ?? contextUser.getId().toString())
                .replace("{reason}", reason.length ? reason : Lang.get("reasonUnknown"));

            this.context?.getChat()?.sendMessage(message, { parse_mode : "HTML" });

        } catch (err: any) {
            Log.save(err.message, err.stack);
        }
    }
}
