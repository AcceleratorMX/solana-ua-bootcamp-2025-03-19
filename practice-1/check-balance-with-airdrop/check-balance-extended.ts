import { Connection, LAMPORTS_PER_SOL, PublicKey, clusterApiUrl } from "@solana/web3.js";
import { airdropIfRequired } from "@solana-developers/helpers";

const connection = new Connection(clusterApiUrl("devnet"));
console.log("Connection is successful!");

const publicKey = new PublicKey("ALEx8xkoLJ2UMH1xo3UJcB5oB4QLRrGVVdKGmDm9j34G");

let balanceInLamports: number, balanceInSOL: number = 0;

let requestsCount = 10;
for (requestsCount; requestsCount > 0; requestsCount--) {
  balanceInLamports = await connection.getBalance(publicKey);
  balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL;

  if (balanceInSOL > 0 && requestsCount < 10) {
    console.log("Airdop is successful!");
    break;
  } else if (balanceInSOL > 0) break;

  console.log("Waiting for airdrop...");

  await airdropIfRequired(
    connection,
    publicKey,
    1 * LAMPORTS_PER_SOL,
    0.5 * LAMPORTS_PER_SOL
  );
}

if (requestsCount === 0) console.error("Airdrop failed!");
console.log(`Your wallet address: ${publicKey}\nYour balance: ${balanceInSOL} SOL`);