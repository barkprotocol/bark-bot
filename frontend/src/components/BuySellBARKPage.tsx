import { useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { buyBARK, sellBARK } from '@/lib/solanaUtils';

const BuySellBARKPage = () => {
  const [amount, setAmount] = useState(0);
  const { connected, publicKey } = useWallet();

  const handleBuy = async () => {
    if (!connected || !publicKey) return alert('Please connect your wallet.');
    try {
      await buyBARK(publicKey, amount);
      alert('BARK tokens purchased successfully!');
    } catch (error) {
      console.error('Buy failed:', error);
      alert('Failed to buy BARK.');
    }
  };

  const handleSell = async () => {
    if (!connected || !publicKey) return alert('Please connect your wallet.');
    try {
      await sellBARK(publicKey, amount);
      alert('BARK tokens sold successfully!');
    } catch (error) {
      console.error('Sell failed:', error);
      alert('Failed to sell BARK.');
    }
  };

  return (
    <div className="container">
      <h1>Buy or Sell BARK Tokens</h1>
      <input
        type="number"
        value={amount}
        onChange={(e) => setAmount(Number(e.target.value))}
        placeholder="Amount"
      />
      <div>
        <button onClick={handleBuy} className="button">
          Buy BARK
        </button>
        <button onClick={handleSell} className="button">
          Sell BARK
        </button>
      </div>
    </div>
  );
};

export default BuySellBARKPage;
