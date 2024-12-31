export const BARK_TOKEN_MINT_ADDRESS = "2NTvEssJ2i998V2cMGT4Fy3JhyFnAzHFonDo9dbAkVrg";

// Define the interface for token interactions
export interface TokenTransaction {
  amount: number;
  fromAddress: string;
  toAddress: string;
}
