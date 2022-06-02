export type Tpd = {
  "version": "0.1.0",
  "name": "tpd",
  "instructions": [
    {
      "name": "initializeDeal",
      "accounts": [
        {
          "name": "applicationState",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userOfferer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amountTokens",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "state",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "userOfferer",
            "type": "publicKey"
          },
          {
            "name": "amountTokens",
            "type": "u64"
          }
        ]
      }
    }
  ]
};

export const IDL: Tpd = {
  "version": "0.1.0",
  "name": "tpd",
  "instructions": [
    {
      "name": "initializeDeal",
      "accounts": [
        {
          "name": "applicationState",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userOfferer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amountTokens",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "state",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "userOfferer",
            "type": "publicKey"
          },
          {
            "name": "amountTokens",
            "type": "u64"
          }
        ]
      }
    }
  ]
};
