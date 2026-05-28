# Ada Lovelace Bot

> Yet another Telegram group management bot.

A feature-rich Telegram bot written in Rust, built to help administrators keep their groups clean, organized, and spam-free.

## Features

### AdaShield
Cross-group spam prevention system. When enabled, the bot automatically bans users who are flagged in the shared shield database — stopping known spammers and trolls before they can cause trouble in your group.

### Welcome Messages
Sends a customizable greeting when new members join the group. The message content is configurable per group and supports placeholders for member details.

### Captcha
Restricts newly joined members until they solve a captcha challenge. If the user does not respond within the configured time window (`captcha_ban_seconds`, default 300 s), they are automatically removed from the group.

### Ask to Ask Warning
Detects and warns users who send variations of "can I ask a question?" or "anyone here?" without actually stating their question — a common anti-pattern in technical communities. Encourages users to ask directly instead.

### Warnings System
Admins can issue warnings to members. After reaching the configured warning threshold (default: 3), the user is automatically banned. Warnings can include an optional reason and are tracked per group.

### Additional Capabilities
- **Goodbye messages** — notifies the group when a member leaves
- **Blocked terms** — configurable per-group word/phrase filter with selectable actions (`delete`, `mute`, `warn`, `ban`)
- **Macros** — custom command shortcuts with text responses
- **Federations** — ban a user across multiple linked groups at once
- **Name change warnings** — detects when a member changes their display name and warns them
- **Event message cleanup** — automatically deletes join/leave system messages to reduce noise

## Tech Stack

| Layer | Library |
|---|---|
| Async runtime | [Tokio](https://tokio.rs) |
| HTTP server | [Axum](https://github.com/tokio-rs/axum) |
| HTTP client | [Reqwest](https://github.com/seanmonstar/reqwest) |
| Database ORM | [SQLx](https://github.com/launchbadge/sqlx) + [sqlxx](https://crates.io/crates/sqlxx) |
| Database | MySQL / MariaDB |
| Serialization | [Serde](https://serde.rs) |

## Requirements

- Rust 1.85+ (edition 2024)
- MySQL 8+ or MariaDB 10.6+
- A Telegram Bot token (obtain one from [@BotFather](https://t.me/BotFather))

## Setup

### 1. Clone the repository

```bash
git clone https://github.com/Desenvolvimento-de-Software/mslovelace_bot
cd mslovelace_bot
```

### 2. Configure the environment

```bash
cp .env.example .env
```

Edit `.env` and fill in the required values:

| Variable | Description |
|---|---|
| `PORT` | Port the webhook server listens on |
| `AUTH` | Secret token appended to the webhook URL path |
| `DATABASE_URL` | MySQL connection string |
| `TELEGRAM_USER_ID` | Your personal Telegram user ID (bot owner) |
| `TELEGRAM_USERNAME` | The bot's username (without `@`) |
| `TELEGRAM_BOT_TOKEN` | Bot token from BotFather |
| `TELEGRAM_WEBHOOK_ENABLED` | `true` to use webhooks, `false` for long polling |
| `TELEGRAM_POLLING_TIMEOUT` | Long-polling timeout in seconds (ignored when using webhooks) |
| `DEBUG` | `true` to run both server and polling simultaneously |

### 3. Create the database schema

```bash
mysql -u <user> -p <database> < database/schema.sql
```

### 4. Build and run

**Development:**
```bash
cargo run
```

**Production:**
```bash
cargo build --release
./target/release/mslovelace_bot
```

## Receiving Updates

The bot supports two update modes, controlled by `TELEGRAM_WEBHOOK_ENABLED`:

- **Long polling** (`false`) — the bot polls Telegram's servers directly. No public URL required. Ideal for development.
- **Webhook** (`true`) — Telegram pushes updates to your server. Requires a publicly accessible HTTPS URL. Register it with:

```
https://api.telegram.org/bot<TOKEN>/setWebhook?url=https://<your-domain>/<AUTH>
```

Setting `DEBUG=true` runs both modes simultaneously, which is useful for local testing with a tunnel (e.g. [ngrok](https://ngrok.com)).

## License

GNU General Public License v3.0