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
    const balance = await connection.getBalance(from.publicKey);

    const txFee = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance,
      })
    );

    const { blockhash } = await connection.getLatestBlockhash();
    txFee.recentBlockhash = blockhash;
    txFee.feePayer = from.publicKey;

    const feeForTx =
      (await connection.getFeeForMessage(txFee.compileMessage(), "confirmed"))
        .value || 0;

    txFee.instructions.pop();

    txFee.add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance - feeForTx,
      })
    );

    const sign = await sendAndConfirmTransaction(connection, txFee, [from]);

    console.log(
      `Success! Check out TX here : https://explorer.solana.com/tx/${sign}?cluster=devnet`
    );
  } catch (error) {
    console.log("Something went wrong :", error);
  }
};

transfer();

const balance = async () => {
  const balance = await connection.getBalance(from.publicKey);
  console.log(`Balance : ${balance / LAMPORTS_PER_SOL} SOL`);
};

// balance();
