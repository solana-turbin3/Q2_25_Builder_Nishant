import { Keypair } from "@solana/web3.js";
import bs58 from "bs58";
const keypair = Keypair.generate();

console.log("Created Keypair Address:", keypair.publicKey.toBase58());
console.log("Keypair Secret Key:", bs58.encode(keypair.secretKey));

//Address : 3QBEspdTy287wywo6SmRLvW3GGEbCbZ2tCH1LmwGYA7h
