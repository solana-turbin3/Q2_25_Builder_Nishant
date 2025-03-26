import "dotenv/config";
import {
  Transaction,
  sendAndConfirmTransaction,
  Connection,
  SystemProgram,
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import bs58 from "bs58";

const localPrivateKey = process.env.LOCAL_WALLET_ADDRESS_KEY;
const from = Keypair.fromSecretKey(bs58.decode(localPrivateKey!));

const to = new PublicKey("HiMmuCbieNgDNFd9GbcbVSHYPGPuEgZWwQxJULaJVoVs");

const connection = new Connection("https://api.devnet.solana.com");

const transfer = async () => {
  try {
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: LAMPORTS_PER_SOL * 0.1,
      })
    );

    const { blockhash } = await connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = from.publicKey;

    const sign = await sendAndConfirmTransaction(connection, transaction, [
      from,
    ]);

    console.log(
      `Success! Check out TX here : https://explorer.solana.com/tx/${sign}?cluster=devnet`
    );
  } catch (error) {
    console.log("Something went wrong :", error);
  }
};

transfer();
