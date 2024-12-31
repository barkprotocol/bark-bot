import { Tool } from '@/src/langchain/tools';
import { SolanaAgentKit } from '@/src/agent';

// Custom Tool for Solana logic
export class CustomTool extends Tool {
  name = "custom_tool";
  description = "A tool to execute custom logic using SolanaAgentKit.";

  constructor(private solanaKit: SolanaAgentKit) {
    super();
  }

  // Override _call method for custom logic
  protected async _call(input: string): Promise<string> {
    try {
      // Validate the input (e.g., check if input is a valid Solana address)
      const walletAddress = input.trim();
      if (!walletAddress || !this.isValidSolanaAddress(walletAddress)) {
        throw new Error("Invalid Solana wallet address.");
      }

      // Fetch the token balance for the wallet
      const balanceResult = await this.solanaKit.getTokenBalance(walletAddress);
      
      return JSON.stringify({
        status: "success",
        message: "Token balance fetched successfully",
        data: { balance: balanceResult }, // Adjust format if needed
      });
    } catch (error: any) {
      return JSON.stringify({
        status: "error",
        message: error.message,
        code: error.code || "UNKNOWN_ERROR",
      });
    }
  }

  // Helper function to validate Solana address
  private isValidSolanaAddress(address: string): boolean {
    try {
      new PublicKey(address); // Will throw if invalid
      return true;
    } catch (error) {
      return false;
    }
  }
}

// Tool for sending Solana transactions
export class SendTransactionTool extends Tool {
  name = "send_transaction_tool";
  description = "A tool to send Solana transactions.";

  constructor(private solanaKit: SolanaAgentKit) {
    super();
  }

  // Override _call method to handle transaction logic
  protected async _call(input: string): Promise<string> {
    try {
      // Validate input format and amount
      const [recipientAddress, amountStr] = input.split(" ").map((str) => str.trim());
      if (!recipientAddress || !amountStr) {
        throw new Error("Invalid input format. Expected 'recipient_address amount'.");
      }

      // Validate recipient address and amount
      if (!this.isValidSolanaAddress(recipientAddress)) {
        throw new Error("Invalid recipient Solana address.");
      }

      const amount = parseFloat(amountStr);
      if (isNaN(amount) || amount <= 0) {
        throw new Error("Invalid amount. Must be a positive number.");
      }

      // Send the transaction using SolanaKit
      const result = await this.solanaKit.sendTransaction(recipientAddress, amount);
      
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

  // Helper function to validate Solana address
  private isValidSolanaAddress(address: string): boolean {
    try {
      new PublicKey(address); // Will throw if invalid
      return true;
    } catch (error) {
      return false;
    }
  }
}

// Helper function to create tools
export function createSolanaTools(solanaKit: SolanaAgentKit) {
  return [
    new CustomTool(solanaKit),
    new SendTransactionTool(solanaKit),
  ];
}
