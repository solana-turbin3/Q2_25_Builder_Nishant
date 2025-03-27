use bs58;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::io::{self, BufRead};
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    #[test]
    fn keygen() {
        let kp = Keypair::new();

        println!("Generated Solana Wallet : {}", kp.pubkey().to_string());
        println!("Save your wallet into json file");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {}

    #[test]
    fn transfer_sol() {}

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");

        let stdin = io::stdin();

        let base58 = stdin.lock().lines().next().unwrap().unwrap();

        println!("Your wallet file is:");

        let wallet = bs58::decode(base58).into_vec().unwrap();

        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");

        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches("[")
            .trim_end_matches("]")
            .split(",")
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        println!("You private key :");

        let base58 = bs58::encode(wallet).into_string();

        println!("{:?}", base58)
    }
}
