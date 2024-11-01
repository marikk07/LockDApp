export type EscrowProgram = {
  "version": "0.1.0",
  "name": "escrow_program",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "condition",
          "type": "string"
        }
      ]
    },
    {
      "name": "release",
      "accounts": [
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "escrow",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "conditionHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "isReleased",
            "type": "bool"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "IncorrectCondition",
      "msg": "Incorrect condition provided."
    },
    {
      "code": 6001,
      "name": "FundsAlreadyReleased",
      "msg": "Funds have already been released."
    }
  ]
};

export const IDL: EscrowProgram = {
  "version": "0.1.0",
  "name": "escrow_program",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "condition",
          "type": "string"
        }
      ]
    },
    {
      "name": "release",
      "accounts": [
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "escrow",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "conditionHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "isReleased",
            "type": "bool"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "IncorrectCondition",
      "msg": "Incorrect condition provided."
    },
    {
      "code": 6001,
      "name": "FundsAlreadyReleased",
      "msg": "Funds have already been released."
    }
  ]
};
