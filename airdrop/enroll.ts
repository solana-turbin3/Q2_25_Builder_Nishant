import "dotenv/config";
import {
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
  Connection,
} from "@solana/web3.js";
import bs58 from "bs58";
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { IDL } from "./programs/turbin_preq.ts";

import type { Turbin3Prereq } from "./programs/turbin_preq.ts";

const turbinKey = process.env.TURBIN_KEY!;
const turbinKeypair = Keypair.fromSecretKey(bs58.decode(turbinKey));

const connection = new Connection("https://api.devnet.solana.com");
const github = Buffer.from("NishantCoder108", "utf8");

console.log("Github buffer:", github);

const provider = new AnchorProvider(connection, new Wallet(turbinKeypair), {
  commitment: "confirmed",
});

console.log({ provider });

//create program :

const program: Program<Turbin3Prereq> = new Program(IDL, provider);
console.log({ program });

const enrollTx = async () => {
  try {
    const txHash = await program.methods
      .submit(github)
      .accounts({
        signer: turbinKeypair.publicKey,
      })
      .signers([turbinKeypair])
      .rpc();

    console.log(
      `Success! Check out TX here : https://explorer.solana.com/tx/${txHash}?cluster=devnet`
    );
  } catch (error) {
    console.log("Something went wrong:", error);
  }
};

enrollTx();

// Submitted
// https://explorer.solana.com/tx/51uVHmbWxsbotvMqrgrXhP6J5rRyxdNk2K8aFA21FSVM9NUGfpxRowNZZsXuGEQJTavH7EVcaucWeMeYqfiheWVz?cluster=devnet
