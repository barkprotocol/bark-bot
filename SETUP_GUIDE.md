# BARK Bot Setup Guide

Welcome to the BARK Bot Setup Guide! Follow the steps below to configure and run the BARK Bot and its accompanying API service.

## Prerequisites

Before you begin, ensure you have the following installed on your system:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [PostgreSQL](https://www.postgresql.org/download/) (for database management)
- [Telegram Bot API Token](https://core.telegram.org/bots#botfather) (create a bot via BotFather)
- [Dialect Blink Protocol Key](https://docs.dialect.to) (required for Blink integration)
- [Git](https://git-scm.com/)

---

## Step 1: Clone the Repository

```bash
git clone https://github.com/bark-protocol/bark-bot-platform.git
cd bark-bot-platform
```

---

## Step 2: Configure Environment Variables

Update the `config.toml` file in the project root with the necessary environment variables. Below is an example configuration:

```toml
[telegram]
bot_token = "your-telegram-bot-token"

[database]
url = "postgresql://user:password@localhost:5432/bark_bot"

[dialect]
blink_key = "your-blink-key"

[squads]
api_url = "https://squads-v3-api.example.com"
```

---

## Step 3: Setup the Database

Initialize the PostgreSQL database:

```bash
# Login to PostgreSQL
psql -U postgres

# Create the database
CREATE DATABASE bark_bot;

# Exit PostgreSQL
\q
```

Run migrations (if applicable):

```bash
cargo install sqlx-cli # Install SQLx CLI if not already installed
sqlx migrate run
```

---

## Step 4: Start the Services

Start the API service:

```bash
cd bark_api
cargo run
```

Start the Telegram bot:

```bash
cd bark_bot
cargo run
```

---

## Step 5: Test the Setup

1. Open Telegram and find your bot using the username created in BotFather.
2. Start a conversation and send a command like `/start` to confirm the bot is responsive.
3. Test a transaction flow by sending a blink to the bot and verifying the process.

---

## Troubleshooting

- **Database Connection Issues**: Verify the `url` in the `[database]` section of `config.toml` matches your PostgreSQL setup.
- **Missing Dependencies**: Ensure all required Rust crates and external tools are installed.
- **Bot Not Responding**: Double-check the Telegram Bot API token and ensure the bot is enabled in BotFather.

---

## Additional Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Telegram Bot API](https://core.telegram.org/bots/api)
- [Dialect Documentation](https://docs.dialect.to/)
- [Squads v3 API](https://squads.so/docs)
