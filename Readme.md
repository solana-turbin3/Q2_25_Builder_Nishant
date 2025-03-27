#### Wallet Address : 2122teTsNBdojUrWAMsRcqLKG9tt8Ah5EFhRE6ti6nXc

### For running tests

```bash
cargo test
```

### For running tests with logs

```bash
cargo test wallet_to_base58 -- --nocapture
```

### For Idlgen 
- Add `solana-idlgen` to `Cargo.toml` file
- Add `address` inside metadata for transaction smoothly

```bash
idlgen!(
    {
        "address": "Trb3aEx85DW1cEEvoqEaBkMn1tfmNEEEPaKzLSu4YAv",
        "metadata": {
          "name": "turbine_prereq",
          "version": "0.1.0",
          "spec": "0.1.0",
          "description": "Created with Anchor"
          "address": "Trb3aEx85DW1cEEvoqEaBkMn1tfmNEEEPaKzLSu4YAv"
    }
);  
```

Submitted Prereq : https://explorer.solana.com/tx/22gBKPfws6pimzZWAWZ623NFqtkZPLZ7a4X1yAKC7wBvkLvMBLYq7Ln2v1QtdQ9pdDtWWh2bkRFDyAnf9qPZyxk9/?cluster=devnet