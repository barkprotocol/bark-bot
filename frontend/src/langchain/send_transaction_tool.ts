import { Tool } from '@/src/langchain/tools';
import { SolanaAgentKit } from '@/src/agent';
import { PublicKey } from '@solana/web3.js';

// SendTransactionTool: A tool to send Solana transactions
export class SendTransactionTool extends Tool {
  name = "send_transaction_tool";
  description = "A tool to send Solana transactions.";

  constructor(private solanaKit: SolanaAgentKit) {
    super();
  }

  // Override the _call method to handle sending transactions
  protected async _call(input: string): Promise<string> {
    try {
      // Expecting input format: "recipient_address amount"
      const [recipientAddress, amountStr] = input.split(" ").map((str) => str.trim());

      // Validate input format
      if (!recipientAddress || !amountStr) {
        throw new Error("Invalid input format. Expected 'recipient_address amount'.");
      }

      // Validate recipient address
      let recipientPubKey: PublicKey;
      try {
        recipientPubKey = new PublicKey(recipientAddress); // Throws if invalid
      } catch (error) {
        throw new Error("Invalid recipient address.");
      }

      // Validate amount
      const amount = parseFloat(amountStr);
      if (isNaN(amount) || amount <= 0) {
        throw new Error("Invalid amount. Must be a positive number.");
      }

      // Sending the transaction using SolanaAgentKit
      const result = await this.solanaKit.sendTransaction(recipientPubKey.toBase58(), amount);

      return JSON.stringify({
        status: "success",
        message: "Transaction sent successfully",
        data: result,
      });
    } catch (error: any) {
      return JSON.stringify({
        status: "error",
        message: error.message,
        code: error.code || "UNKNOWN_ERROR",
      });
    }
  }
}
