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

import Federation from "./Federation";
import { BotCommand } from "libraries/telegram/types/BotCommand";
import Lang from "helpers/Lang";
import Log from "helpers/Log";
import { getFederationById, getFederationByHash } from "services/Federations";
import { PrismaClient } from "@prisma/client";

export default class Group extends Federation {

    /**
     * Commands list.
     *
     * @author Marcos Leandro
     * @since  2024-05-03
     *
     * @var {BotCommand[]}
     */
    public readonly commands: BotCommand[] = [
        { command: "fshow", description: "Shows the federation information." },
        { command: "fjoin", description: "Joins a federation." },
        { command: "fleave", description: "Leaves a federation." }
    ];

    /**
     * The constructor.
     *
     * @author Marcos Leandro
     * @since  2023-07-04
     */
    public constructor() {
        super();
    }

    /**
     * Shows the group federation.
     *
     * @author Marcos Leandro
     * @since  2024-09-09
     *
     * @return {Promise<void>}
     */
    private async show(): Promise<void> {

        if (this.context?.getChat()?.getType() === "private") {
            this.context?.getMessage()?.reply(Lang.get("federationCommandOnlyGroupError"));
            return;
        }

        if (!this.chat?.federation_id) {
            this.context?.getMessage()?.reply(Lang.get("federationLeaveNoFederationError"));
            return;
        }

        const federation = await getFederationById(this.chat.federation_id);
        if (!federation) {
            this.context?.getMessage()?.reply(Lang.get("federationLeaveNoFederationError"));
            return;
        }

        const message = Lang.get("federationDetails")
            .replace("{federation}", federation.description ?? "")
            .replace("{hash}", federation.hash);

        this.context?.getMessage()?.reply(message, { parse_mode : "HTML" });
    }

    /**
     * Joins a federation.
     *
     * @author Marcos Leandro
     * @since  2023-07-04
     */
    private async join(): Promise<void> {

        if (this.context?.getChat()?.getType() === "private") {
            this.context?.getMessage()?.reply(Lang.get("federationCommandOnlyGroupError"));
            return;
        }

        if (!await this.context?.getUser()?.isAdmin()) {
            this.context?.getMessage()?.reply(Lang.get("federationJoinOnlyAdminError"));
            return;
        }

        const params = this.command?.getParams() || [];
        if (!params.length) {
            this.context?.getMessage()?.reply(Lang.get("federationJoinNoHashError"));
            return;
        }

        const hash = params[0].trim();
        const federation = await getFederationByHash(hash);

        if (!federation) {
            this.context?.getMessage()?.reply(Lang.get("federationInvalidHashError"));
            return;
        }

        if (this.chat?.federation_id) {
            this.context?.getMessage()?.reply(Lang.get("federationJoinHasFederationError"));
            return;
        }

        if (this.chat?.federation_id === federation.id) {
            this.context?.getMessage()?.reply(Lang.get("federationJoinAlreadyJoinedError"));
            return;
        }

        try {

            const prisma = new PrismaClient();
            await prisma.chats.update({
                where: { chat_id: this.context!.getChat()!.getId() },
                data: { federation_id: federation.id }

            }).catch((err) => {
                Log.error(err.message, err.stack);

            }).finally(async () => {
                await prisma.$disconnect();
            });;

            const message = Lang.get("federationJoinSuccess")
                .replace("{federation}", federation.description ?? "");

            this.context?.getMessage()?.reply(message);

        } catch (err: any) {
            this.context?.getMessage()?.reply(Lang.get("federationJoinError"));
            Log.error(err.toString());
            return;
        }
    }

    /**
     * Leaves a federation.
     *
     * @author Marcos Leandro
     * @since  2023-07-04
     */
    private async leave(): Promise<void> {

        if (this.context?.getChat()?.getType() === "private") {
            this.context?.getMessage()?.reply(Lang.get("federationCommandOnlyGroupError"));
            return;
        }

        if (!await this.context?.getUser()?.isAdmin()) {
            this.context?.getMessage()?.reply(Lang.get("federationJoinOnlyAdminError"));
            return;
        }

        if (!this.chat?.federation_id) {
            this.context?.getMessage()?.reply(Lang.get("federationLeaveNoFederationError"));
            return;
        }

        try {

            const prisma = new PrismaClient();
            await prisma.chats.update({
                where: { chat_id: this.context!.getChat()!.getId() },
                data: { federation_id: null }

            }).catch((err) => {
                Log.error(err.message, err.stack);

            }).finally(async () => {
                await prisma.$disconnect();
            });

            this.context?.getMessage()?.reply(Lang.get("federationLeaveSuccess"));

        } catch (err: any) {
            this.context?.getMessage()?.reply(Lang.get("federationLeaveError"));
            Log.error(err.toString());
            return;
        }
    }
}
