import { Tool } from "langchain/tools";
import { SolanaAgentKit } from "../agent";

export class CustomTool extends Tool {
  name = "custom_tool";
  description = "A tool to execute custom logic using Solana Agent Kit.";

  constructor(private solanaKit: SolanaAgentKit) {
    super();
  }

  // Method that will be called when this tool is used
  protected async _call(input: string): Promise<string> {
    try {
      // Example: Check if the input is a Solana wallet address, and fetch balance
      const balanceResult = await this.solanaKit.getTokenBalance(input);
      return JSON.stringify({
        status: "success",
        message: "Custom tool executed successfully",
        data: balanceResult,
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
