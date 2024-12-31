import TelegramBot from 'node-telegram-bot-api';
import { handleMessage } from './handlers';
import { getTelegramBotToken } from '../../config/telegramConfig';

// Initialize the bot using your bot's API token
const bot = new TelegramBot(getTelegramBotToken(), { polling: true });

// Register event handlers
bot.onText(/\/start/, (msg) => handleMessage(msg));
bot.onText(/\/bark_token_balance/, (msg) => handleMessage(msg));
bot.onText(/\/claim_nft/, (msg) => handleMessage(msg));

export default bot;
