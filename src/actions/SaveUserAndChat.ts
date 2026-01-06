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
import Chat from "contexts/Chat";
import Context from "contexts/Context";
import Log from "helpers/Log";
import User from "contexts/User";
import { createAndGetChat, getChatById } from "services/Chats";
import { createAndGetUser } from "services/Users";
import { chats, users, PrismaClient } from "@prisma/client";

export default class SaveUserAndChat extends Action {

    /**
     * The constructor.
     *
     * @author Marcos Leandro
     * @since  2023-06-02
     *
     * @param context
     */
    public constructor(context: Context) {
        super(context, "sync");
    }

    /**
     * Runs the action.
     *
     * @author Marcos Leandro
     * @since  2023-06-06
     */
    public async run(): Promise<void> {

        try {

            const contextUser = this.getContextUser();
            const contextChat = this.context.getChat();

            if (!contextUser || !contextChat) {
                return Promise.resolve();
            }

            const user = await this.getUser(contextUser, true);
            const chat = await this.getChat(contextChat);
            const chatWithConfigs = await getChatById(chat.id);

            if (!user || !chatWithConfigs) {
                return Promise.reject(new Error("User or chat not found."));
            }

            const prisma = new PrismaClient();
            await prisma.rel_users_chats.upsert({
                where: {
                    user_id_chat_id: {
                        user_id: user.id,
                        chat_id: chatWithConfigs.id
                    }
                },
                create: {
                    user_id: user.id,
                    chat_id: chatWithConfigs.id,
                    joined: true,
                    checked: !(chatWithConfigs.chat_configs?.captcha ?? false),
                    date: Math.floor(Date.now() / 1000),
                    last_seen: Math.floor(Date.now() / 1000)
                },
                update: {
                    joined: true,
                    last_seen: Math.floor(Date.now() / 1000)
                }

            }).catch((err: Error) => {
                Log.save(err.message, err.stack);

            }).finally(async () => {
                await prisma.$disconnect();
            });

        } catch (err: any) {
            Log.save("SaveUserAndChat :: " + err.message, err.stack);
        }
    }

    /**
     * Returns the context user.
     *
     * @author Marcos Leandro
     * @since  2025-03-08
     *
     * @throws Error User not found.
     *
     * @return Context user, if applicable.
     */
    private getContextUser(): User|undefined {
        const contextUser = this.context.getNewChatMember() ?? this.context.getLeftChatMember() ?? this.context.getUser();
        if (!contextUser) {
            return;
        }

        return contextUser;
    }

    /**
     * Returns the user.
     *
     * @author Marcos Leandro
     * @since  2025-03-08
     *
     * @param contextUser
     *
     * @throws Error User not found.
     *
     * @return User.
     */
    private async getUser(contextUser: User, retry: boolean): Promise<users> {

        if (!contextUser) {
            throw new Error("User not found in the context.");
        }

        let user;

        try {
            user = await createAndGetUser(contextUser);

        } catch (err: any) {
            await new Promise(resolve => setTimeout(resolve, 500));
            retry && (user = await this.getUser(contextUser, false));
        }

        if (!user) {
            throw new Error("User not found in the context. " + JSON.stringify(this.context.getPayload()));
        }

        return user;
    }

    /**
     * Returns the context chat.
     *
     * @author Marcos Leandro
     * @since  2025-03-08
     *
     * @param contextChat
     *
     * @throws Error Chat not found.
     *
     * @return Context chat.
     */
    private async getChat(contextChat?: Chat): Promise<chats> {

        if (!contextChat) {
            throw new Error("Chat not found in the context.");
        }

        const chat = await createAndGetChat(contextChat);
        if (!chat) {
            throw new Error("Chat not found in the context. " + JSON.stringify(this.context.getPayload()));
        }

        return chat;
    }
}
