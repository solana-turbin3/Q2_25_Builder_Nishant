import "dotenv/config";
import {
  Connection,
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import bs58 from "bs58";

const privateKey = process.env.LOCAL_WALLET_ADDRESS_KEY;

const localAddress = Keypair.fromSecretKey(bs58.decode(privateKey!));

console.log(localAddress.publicKey.toBase58());

const connection = new Connection("https://api.devnet.solana.com");

const airdropToWallet = async () => {
  try {
    const signature = await connection.requestAirdrop(
      localAddress.publicKey,
      LAMPORTS_PER_SOL * 5
    );
    // console.log("Airdrop Sign :", signature)
    console.log(
      `Success! Check out TX here : https://explorer.solana.com/tx/${signature}?cluster=devnet`
    );
  } catch (error) {
    console.log("Somethign went  wrong :", error);
  }
};

airdropToWallet();
