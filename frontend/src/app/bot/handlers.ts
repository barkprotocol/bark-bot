import { sendMessageToUser } from './telegramBotClient';
import { checkBarkTokenBalance } from '../barkToken/tokenService';
import { claimNFT } from '../solana/solanaService';

export const handleMessage = async (msg: TelegramBot.Message) => {
  const chatId = msg.chat.id;
  const text = msg.text ?? '';

  if (text === '/start') {
    await sendMessageToUser(chatId, 'Welcome to the Bark Bot!');
  } else if (text === '/bark_token_balance') {
    const balance = await checkBarkTokenBalance(msg.from?.id ?? 0);
    await sendMessageToUser(chatId, `Your Bark Token balance is: ${balance}`);
  } else if (text === '/claim_nft') {
    const result = await claimNFT(msg.from?.id ?? 0);
    await sendMessageToUser(chatId, result);
  } else {
    await sendMessageToUser(chatId, 'Unknown command');
  }
};
