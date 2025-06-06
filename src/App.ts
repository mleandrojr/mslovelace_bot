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

import express from "express";
import TelegramBotApi from "./libraries/telegram/TelegramBotApi";
import SetMyCommands from "./libraries/telegram/resources/SetMyCommands";
import Command from "./commands/Command";
import Iinterval from "interfaces/Iinterval";
import Log from "./helpers/Log";
import { controllers } from "./configs/controllers";
import { commands as commandsConfig } from "./configs/commands";
import DeleteExpiredMessages from "intervals/DeleteExpiredMessages";
import KickUnverifiedUsers from "intervals/KickUnverifiedUsers";

export default class App {

    /**
     * Express application instance.
     *
     * @author Marcos Leandro
     * @since  1.0.0
     *
     * @var {express.Application}
     */
    private readonly expressApp: express.Application;

    /**
     * Application port.
     *
     * @author Marcos Leandro
     * @since  1.0.0
     *
     * @var {number}
     */
    private port: number;

    /**
     * Registered commands.
     *
     * @author Marcos Leandro
     * @since  2025-01-06
     *
     * @var {Command[]}
     */
    private readonly commands: Command[] = [];

    /**
     * Registered intervals.
     *
     * @author Marcos Leandro
     * @since  2025-02-25
     */
    private readonly intervals: Iinterval[] = [];

    /**
     * The constructor.
     *
     * @author Marcos Leandro
     * @since  1.0.0
     */
    constructor() {

        this.expressApp = express();
        this.port = (process.env.PORT ?? 3000) as number;

        TelegramBotApi.setToken(process.env.TELEGRAM_BOT_TOKEN ?? "");
    }

    /**
     * Initializes the application.
     *
     * @author Marcos Leandro
     * @since  2024-05-03
     */
    public async init() {
        await this.registerCommands();
        this.initializeMiddlewares();
        this.initializeControllers();
        this.initializeIntervals();
    }

    /**
     * Returns the controller's commands.
     *
     * @author Marcos Leandro
     * @since  2025-01-06
     *
     * @return {Command[]}
     */
    public getCommands(): Command[] {
        return this.commands;
    }

    /**
     * Starts to listen in the specified port.
     *
     * @author Marcos Leandro
     * @since  1.0.0
     *
     * @return {void}
     */
    public listen(): void {

        this.expressApp.listen(this.port, () => {
            Log.warn(`Listening on port ${this.port}`);

        }).on("error", (err) => {
            Log.error(`Port ${this.port} is already in use. Trying to use another port...`);
            this.port++;
            this.listen();
        });
    }

    /**
     * Initializes the controllers.
     *
     * @author Marcos Leandro
     * @since  2023-06-06
     */
    private initializeControllers(): void {
        controllers.forEach((controller) => {
            this.expressApp.use("/", (new controller(this)).getRoutes());
        });
    }

    /**
     * Initializes the middlewares.
     *
     * @author Marcos Leandro
     * @since  1.0.0
     *
     * @return {void}
     */
    private initializeMiddlewares(): void {
        this.expressApp.use(express.json());
        this.expressApp.use(express.urlencoded({ extended : true }));
    }

    /**
     * Initializes the intervals.
     *
     * @author Marcos Leandro
     * @since  2025-02-25
     */
    private initializeIntervals(): void {
        this.intervals.push(...[
            new DeleteExpiredMessages(),
            new KickUnverifiedUsers()
        ]);
    }

    /**
     * Registers the bot commands.
     *
     * @author Marcos Leandro
     * @since  2024-05-03
     */
    private async registerCommands(): Promise<void> {

        const availableCommands = [];
        for (const commandClass of commandsConfig) {
            const commandInstance = new commandClass();
            this.commands.push(commandInstance);
            availableCommands.push(...commandInstance.commands);
        }

        if (!availableCommands.length) {
            return;
        }

        Log.info("Registering " + availableCommands.length + " commands...");
        const request = new SetMyCommands();
        request.setCommands(availableCommands);

        const response = await request.post();
        if (response.ok) {
            Log.info("Commands successfully registered");
        }
    }
}
