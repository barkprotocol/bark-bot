import { WalletProvider, ConnectionProvider } from '@solana/wallet-adapter-react';
import { PhantomWalletAdapter, SolflareWalletAdapter } from '@solana/wallet-adapter-wallets';
import BuySellBARKPage from '../components/BuySellBARKPage';

const App = () => {
  const wallets = [new PhantomWalletAdapter(), new SolflareWalletAdapter()];

  return (
    <ConnectionProvider endpoint="https://api.devnet.solana.com">
      <WalletProvider wallets={wallets} autoConnect>
        <div className="App">
          <h1>Welcome to BARK Token Marketplace</h1>
          <BuySellBARKPage />
        </div>
      </WalletProvider>
    </ConnectionProvider>
  );
};

export default App;
