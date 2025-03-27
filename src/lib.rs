use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
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
}
