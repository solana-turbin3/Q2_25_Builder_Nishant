// idlgen!(
//     {
//         "version": "0.1.0",
//         "name": "turbine_prereq",
//         "instructions": [
//             {
//                 "name": "clean",
//                 "accounts": [
//                     { "name": "signer", "isMut": true, "isSigner": true },
//                     { "name": "prereq", "isMut": true, "isSigner": false }
//                 ],
//                 "args": []
//             },
//             {
//                 "name": "submit",
//                 "accounts": [
//                     { "name": "signer", "isMut": true, "isSigner": true },
//                     {
//                         "name": "prereq",
//                         "isMut": true,
//                         "isSigner": false,
//                         "pda": {
//                             "seeds": [
//                                 { "kind": "const", "value": "preQ225" },
//                                 { "kind": "account", "path": "signer" }
//                             ]
//                         }
//                     },
//                     { "name": "system_program", "isMut": false, "isSigner": false }
//                 ],
//                 "args": [{ "name": "github_username", "type": "bytes" }]
//             },
//             {
//                 "name": "update",
//                 "accounts": [
//                     { "name": "signer", "isMut": true, "isSigner": true },
//                     { "name": "prereq", "isMut": true, "isSigner": false },
//                     { "name": "system_program", "isMut": false, "isSigner": false }
//                 ],
//                 "args": [{ "name": "github", "type": "bytes" }]
//             }
//         ],
//         "accounts": [
//             {
//                 "name": "Q2Prereq2024",
//                 "type": {
//                     "kind": "struct",
//                     "fields": [
//                         { "name": "github", "type": "bytes" },
//                         { "name": "key", "type": "pubkey" }
//                     ]
//                 }
//             },
//             {
//                 "name": "Q2Prereq2025",
//                 "type": {
//                     "kind": "struct",
//                     "fields": [
//                         { "name": "github", "type": "bytes" },
//                         { "name": "key", "type": "pubkey" }
//                     ]
//                 }
//             }
//         ],
//         "metadata": {
//             "address": "Trb3aEx85DW1cEEvoqEaBkMn1tfmNEEEPaKzLSu4YAv",
//             "spec": "0.1.0",
//             "description": "Created with Anchor"
//         },
//         "errors": [
//             {
//                 "code": 6000,
//                 "name": "InvalidGithubAccount",
//                 "msg": "Invalid Github account"
//             }
//         ]
//     }
// );

use solana_idlgen::idlgen;
idlgen!({
  "version": "0.1.0",
  "name": "turbine_prereq",
  "instructions": [
    {
      "name": "complete",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "prereq",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "github",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "update",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "prereq",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "github",
          "type": "bytes"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "SolanaCohort5Account",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "github",
            "type": "bytes"
          },
          {
            "name": "key",
            "type": "publicKey"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidGithubAccount",
      "msg": "Invalid Github account"
    }
  ],
  "metadata": {
    "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa"
  }
});
