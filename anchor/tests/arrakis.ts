import { autoDiscover, createClient } from "@solana/client";

const client = createClient({
  cluster: "devnet",
  walletConnectors: autoDiscover(),
});

// Connect a wallet
const connectors = client.connectors.all;
await client.actions.connectWallet(connectors[0].id);

// Fetch balance
const wallet = client.store.getState().wallet;
if (wallet.status === "connected") {
  const balance = await client.actions.fetchBalance(wallet.session.account.address);
  console.log(`Balance: ${balance.toString()} lamports`);
}

// Send SOL
const signature = await client.solTransfer.sendTransfer({
  amount: 100_000_000n, // 0.1 SOL
  authority: wallet.session,
  destination: "Fg6PaFpoGXkYsidMpWFKfwtz6DhFVyG4dL1x8kj7ZJup",
});