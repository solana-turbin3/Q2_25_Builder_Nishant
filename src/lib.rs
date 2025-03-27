// use crate::programs::Turbin3_prereq::{SubmitArgs, TurbinePrereqProgram, UpdateArgs};
use crate::programs::Turbin3_prereq::{CompleteArgs, TurbinePrereqProgram, UpdateArgs};
use bs58;

use solana_client::rpc_client::RpcClient;
use solana_program::{account_info, hash::hash, pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::{
    message::Message,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};
use solana_sdk::{signer::keypair, system_program};
use std::io::{self, BufRead};
use std::str::FromStr;

mod programs;

const RPC_URL: &str = "https://api.devnet.solana.com";
const GITHUB_USERNAME: &str = "NishantCoder108";

fn submit() {
    let keypair = read_keypair_file("wallet_turbine_key.json").expect("Couldn't find wallet file");

    let client = RpcClient::new(RPC_URL);

    // let (prereq, _bump) = Pubkey::find_program_address(
    //     &[b"preQ225", keypair.pubkey().as_ref()],
    //     &TurbinePrereqProgram::id(),
    // );

    let prereq = TurbinePrereqProgram::derive_program_address(&[
        b"prereq",
        keypair.pubkey().to_bytes().as_ref(),
    ]);

    let args = CompleteArgs {
        github: GITHUB_USERNAME.as_bytes().to_vec(),
    };

    // let updateArgs = UpdateArgs {
    //     github: b"NishantCoder108".to_vec(),
    // };
    // Get recent blockhash
    let blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    // let transaction = TurbinePrereqProgram::submit(
    //     &[&keypair.pubkey(), &prereq, &system_program::id()],
    //     &args,
    //     Some(&keypair.pubkey()),
    //     &[&keypair],
    //     blockhash,
    // );

    // let transaction = TurbinePrereqProgram::update(
    //     &[&keypair.pubkey(), &prereq, &system_program::id()],
    //     &updateArgs,
    //     Some(&keypair.pubkey()),
    //     &[&keypair],
    //     blockhash,
    // );

    // let transaction = TurbinePrereqProgram::clean(
    //     &[&keypair.pubkey(), &prereq],
    //     keypair.pubkey(),
    //     Some(&keypair.pubkey()),
    //     &[&keypair],
    //     blockhash,
    // );

    let transaction = TurbinePrereqProgram::complete(
        &[&keypair.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&keypair.pubkey()),
        &[&keypair],
        blockhash,
    );
    let signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    )
}

/// Main entry function
pub fn main() {
    submit();
}

#[cfg(test)]
mod tests {

    use super::*;

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
    fn transfer_sol() {
        let keypair = read_keypair_file("wallet_key.json").expect("Couldn't find wallet file");

        // With the imported Keypair, we can sign a new message.
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my solana Keypair!";

        let sig = keypair.sign_message(message_bytes);

        // After that we can verify the singature, using the default implementation
        match sig.verify(&pubkey.to_bytes(), message_bytes) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }

        let to_pubkey = Pubkey::from_str("HiMmuCbieNgDNFd9GbcbVSHYPGPuEgZWwQxJULaJVoVs").unwrap();

        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Get balance of dev wallet
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        println!("Balance: {}", balance);

        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Create a test transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

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
