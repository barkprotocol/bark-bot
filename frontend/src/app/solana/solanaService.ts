import { SolanaAgentKit } from './solanaAgentKit';
import { BARK_TOKEN_MINT_ADDRESS } from '../barkToken/token';

// Initialize SolanaAgentKit
const solanaKit = new SolanaAgentKit();

export const getSolanaTokenBalance = async (mintAddress: string, userAddress: string) => {
  try {
    const balance = await solanaKit.getTokenBalance(mintAddress, userAddress);
    return balance;
  } catch (error) {
    console.error('Error fetching token balance:', error);
    return 0;
  }
};

export const sendSolanaTransaction = async (fromAddress: string, toAddress: string, amount: number) => {
  try {
    const txHash = await solanaKit.transferTokens(fromAddress, toAddress, amount);
    return `Transaction successful with hash: ${txHash}`;
  } catch (error) {
    console.error('Error sending transaction:', error);
    return 'Transaction failed';
  }
};
