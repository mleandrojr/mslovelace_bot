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
import User from "contexts/User";
import { PrismaClient, shield } from "@prisma/client";

const prisma = new PrismaClient();

/**
 * Returns a user from the AdaShield by ID.
 *
 * @author Marcos Leandro
 * @since  2025-03-07
 *
 * @param userContext
 *
 * @return {Promise<shield|null>}
 */
export const getUserByTelegramId = async (telegramUserId: number): Promise<shield|null> => {
    const user = await prisma.shield.findUnique({
        where: { user_id: telegramUserId }

    }).then(async (response) => {
        return response;

    }).catch((e: Error) => {
        throw e;

    }).finally(async () => {
        await prisma.$disconnect();
    });

    return user;
};

/**
 * Returns an user from the AdaShield by username.
 *
 * @author Marcos Leandro
 * @since  2025-12-02
 *
 * @param userContext
 *
 * @return {Promise<shield|null>}
 */
export const getUserByUsername = async (username: string): Promise<shield|null> => {
    const user = await prisma.shield.findUnique({
        where: { username: username }

    }).then(async (response) => {
        return response;

    }).catch((e: Error) => {
        throw e;

    }).finally(async () => {
        await prisma.$disconnect();
    });

    return user;
};

/**
 * Adds an user to the AdaShield.
 *
 * @author Marcos Leandro
 * @since  2025-03-07
 *
 * @param userContext
 * @param reason
 */
export const addUserToShield = async (userContext: User, reason: string): Promise<void> => {
    await prisma.shield.upsert({
        create: {
            user_id: userContext.getId(),
            reason: reason,
            date: Math.floor(Date.now() / 1000)
        },

        update: {
            reason: reason,
            date: Math.floor(Date.now() / 1000)
        },

        where: {
            user_id: userContext.getId()
        }

    }).then(async (response) => {
        return response;

    }).catch((e: Error) => {
        throw e;

    }).finally(async () => {
        await prisma.$disconnect();
    });
};

/**
 * Enables the AdaShield.
 *
 * @author Marcos Leandro
 * @since  2025-03-07
 *
 * @param chatContext
 *
 * @return {Promise<boolean>}
 */
export const enableAdaShield = async (chatContext: Chat): Promise<boolean> => {
    await updateAdaShield(chatContext, true);
    return Promise.resolve(true);
};

/**
 * Disables the AdaShield.
 *
 * @author Marcos Leandro
 * @since  2025-03-07
 *
 * @param chatContext
 *
 * @return {Promise<boolean>}
 */
export const disableAdaShield = async (chatContext: Chat): Promise<boolean> => {
    await updateAdaShield(chatContext, false);
    return Promise.resolve(true);
};

/**
 * Updates the AdaShield status.
 *
 * @author Marcos Leandro
 * @since  2025-03-07
 *
 * @param chatContext
 * @param status
 *
 * @return {Promise<boolean>}
 */
const updateAdaShield = async (chatContext: Chat, status: boolean): Promise<boolean> => {

    const chat = await prisma.chats.findUnique({
        where: { chat_id: chatContext.getId() }
    });

    if (!chat) {
        return Promise.resolve(false);
    }

    await prisma.chat_configs.upsert({
        where: {
            chat_id: Number(chat.id)
        },
        update: {
            adashield: status
        },
        create: {
            chat_id: chat.id,
            adashield: status
        }

    }).then(async (response) => {
        return response;

    }).catch(async (e: Error) => {
        throw e;

    }).finally(async () => {
        await prisma.$disconnect();
    });

    return Promise.resolve(true);
};
