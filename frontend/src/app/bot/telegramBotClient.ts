import TelegramBot from 'node-telegram-bot-api';

// This is where you can extend the functionality of the Telegram bot.
export const sendMessageToUser = async (chatId: number, text: string) => {
  const bot = new TelegramBot(process.env.TELEGRAM_BOT_TOKEN!, { polling: true });
  await bot.sendMessage(chatId, text);
};
