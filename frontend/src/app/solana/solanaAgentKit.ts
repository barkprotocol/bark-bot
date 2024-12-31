import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';

export class SolanaAgentKit {
  private connection: Connection;

  constructor() {
    this.connection = new Connection(process.env.SOLANA_RPC_URL ?? 'https://api.devnet.solana.com');
  }

  // Fetch token balance
  async getTokenBalance(mintAddress: string, userAddress: string) {
    const publicKey = new PublicKey(userAddress);
    const mint = new PublicKey(mintAddress);
    const tokenAccount = await this.connection.getParsedTokenAccountsByOwner(publicKey, { mint });

    if (tokenAccount.value.length === 0) return 0;
    const balance = tokenAccount.value[0].account.data.parsed.info.tokenAmount.uiAmount;
    return balance;
  }

  // Send tokens (example)
  async transferTokens(fromAddress: string, toAddress: string, amount: number) {
    // Placeholder: logic for creating and sending Solana transaction
    const fromPublicKey = new PublicKey(fromAddress);
    const toPublicKey = new PublicKey(toAddress);

    const transaction = new Transaction();
    // Add instruction to transaction (e.g., transfer tokens)
    
    // Send transaction using Solana's sendTransaction method

    return 'example-tx-hash';
  }
}
