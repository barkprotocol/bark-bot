import './bot/bot';
import './barkToken/tokenService';
import { SolanaAgentKit } from '@/src/agent';
import { SendTransactionTool } from './langchain/send_transaction_tool';

// Initialize the SolanaAgentKit instance
const solanaKit = new SolanaAgentKit({
  rpcUrl: process.env.RPC_URL,
  privateKey: process.env.SOLANA_PRIVATE_KEY,
});

// Ensure that environment variables are set correctly
if (!process.env.RPC_URL || !process.env.SOLANA_PRIVATE_KEY) {
  console.error('Missing environment variables: RPC_URL or SOLANA_PRIVATE_KEY');
  process.exit(1); // Exit the process if required variables are missing
}

// Create the SendTransactionTool
const sendTransactionTool = new SendTransactionTool(solanaKit);

// Example input: "recipient_address amount"
const input = "BARKkeAwhTuFzcLHX4DjotRsmjXQ1MshGrZbn1CUQqMo";

// Call the tool with the input
sendTransactionTool._call(input).then(response => {
  console.log("Transaction Response:", response);
}).catch(error => {
  console.error("Error sending transaction:", error);
});

console.log('Bark Bot and Token service are up and running!');
