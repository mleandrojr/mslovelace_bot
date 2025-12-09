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
import Check from "libraries/combot/resources/Check";
import Context from "contexts/Context";
import Lang from "helpers/Lang";
import User from "contexts/User";
import Log from "helpers/Log";
import { addUserToShield, getUserByTelegramId, getUserByUsername } from "services/AdaShield";
import { getChatById } from "services/Chats";
import { PrismaClient } from "@prisma/client";

export default class AdaShield extends Action {

    /**
     * Ban message.
     *
     * @author Marcos Leandro
     * @since  2023-06-06
     */
    private banMessage: string = "adaShieldMessage";

    /**
     * The constructor.
     *
     * @author Marcos Leandro
     * @since  2023-06-06
     *
     * @param context
     */
    public constructor(context: Context) {
        super(context, "sync");
    }

    /**
     * Run the action.
     *
     * @author Marcos Leandro
     * @since  2023-06-06
     *
     * @returns
     */
    public async run(): Promise<void> {

        const chatMember = this.context.getNewChatMember() ?? this.context.getLeftChatMember() ?? this.context.getUser();
        if (!chatMember) {
            return Promise.resolve();
        }

        const user =
            await getUserByTelegramId(chatMember.getId()) ??
            await getUserByUsername(chatMember.getUsername() ?? "");

        if (!user || await this.cas(chatMember)) {
            return Promise.resolve();
        }

        const newChat = this.context.getChat();
        const chat = await getChatById(newChat!.getId());

        chatMember.ban().catch(() => this.banMessage += "2");

        const prisma = new PrismaClient();
        await prisma.rel_users_chats.upsert({
            where: {
                user_id_chat_id: {
                    user_id: user!.id,
                    chat_id: chat!.id
                }
            },
            create: {
                user_id: user!.id,
                chat_id: chat!.id,
                joined: false,
                checked: false,
                date: Math.floor(Date.now() / 1000),
                last_seen: Math.floor(Date.now() / 1000)
            },
            update: {
                joined: false,
                checked: false,
                date: Math.floor(Date.now() / 1000),
                last_seen: Math.floor(Date.now() / 1000)
            }

        }).catch((err: Error) => {
            Log.save(err.message, err.stack);

        }).finally(async () => {
            await prisma.$disconnect();
        });

        try {

            const userId = chatMember.getId();
            const username = (chatMember.getFirstName() ?? chatMember.getUsername());
            const lang = Lang.get(this.banMessage)
                .replace(/{userid}/g, userId.toString())
                .replace(/{username}/g, username ?? "");

            this.context.getChat()?.sendMessage(lang, {
                parse_mode: "HTML",
                link_preview_options: {
                    is_disabled: true
                }
            }).catch(error => {
                throw new Error(error);
            })

        } catch (err) {
            Log.save((err as Error).message, (err as Error).stack);
        }
    }

    /**
     * Executes the Combot Anti-SPAM (CAS) to see if the user is a registered spammer.
     *
     * @author Marcos Leandro
     * @since  2022-09-09
     *
     * @param userId
     *
     * @return
     */
    private async cas(user: User): Promise<boolean> {

        const casCheck = new Check(user.getId());
        const response = await casCheck.get();
        const json = await response.json();

        const result = (!!json?.ok) || false;
        if (!result) {
            return false;
        }

        await addUserToShield(user, "CAS");

        this.banMessage = "casMessage";
        return true;
    }
}
