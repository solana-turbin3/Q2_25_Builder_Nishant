use bs58;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
};
use std::io::{self, BufRead};

const RPC_URL: &str = "https://api.devnet.solana.com";
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
    fn airdrop() {
        let keypair = read_keypair_file("wallet_key.json").expect("Couldn't find wallet file");

        //connected to the devnet rpc client
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000) {
            Ok(s) => {
                println!("Success! Check out TX here:");

                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }

            Err(e) => {
                println!("Something went wrong : {}", e.to_string())
            }
        }
    }

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
