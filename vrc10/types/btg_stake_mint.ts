/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/btg_stake_mint.json`.
 */
export type BtgStakeMint = {
  "address": "vrca8nDGAZW9R23jAerRJUqsd7vz1q3sqkKMxQsisxk",
  "metadata": {
    "name": "btgStakeMint",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addToWhitelist",
      "discriminator": [
        157,
        211,
        52,
        54,
        144,
        81,
        5,
        55
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint"
        },
        {
          "name": "oracleAccount",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  97,
                  99,
                  108,
                  101,
                  95,
                  97,
                  99,
                  99,
                  111,
                  117,
                  110,
                  116
                ]
              }
            ],
            "program": {
              "kind": "account",
              "path": "oracleProgram"
            }
          }
        },
        {
          "name": "oracleProgram",
          "address": "oracynDbHWzsfV7GaQveWhqrAsB1LNq6pr4ftRYME4c"
        }
      ],
      "args": [
        {
          "name": "symbol",
          "type": "string"
        },
        {
          "name": "outputRate",
          "type": "f64"
        }
      ]
    },
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "mintToOwner",
      "discriminator": [
        152,
        88,
        83,
        7,
        21,
        123,
        129,
        108
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "userTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "authority"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                146,
                254,
                170,
                189,
                14,
                251,
                208,
                173,
                193,
                77,
                189,
                183,
                143,
                165,
                133,
                83,
                112,
                168,
                140,
                159,
                214,
                120,
                107,
                2,
                254,
                70,
                202,
                55,
                107,
                110,
                241,
                192
              ]
            }
          }
        },
        {
          "name": "associatedTokenProgram",
          "address": "AtokenhZ6AE34VMYRv1AqSv8q8QZJxxEaY1zKiXKwSWT"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "redeemToken",
      "discriminator": [
        190,
        85,
        90,
        176,
        192,
        218,
        41,
        214
      ],
      "accounts": [
        {
          "name": "stakingVault",
          "writable": true
        },
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "userTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "user"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                146,
                254,
                170,
                189,
                14,
                251,
                208,
                173,
                193,
                77,
                189,
                183,
                143,
                165,
                133,
                83,
                112,
                168,
                140,
                159,
                214,
                120,
                107,
                2,
                254,
                70,
                202,
                55,
                107,
                110,
                241,
                192
              ]
            }
          }
        },
        {
          "name": "tokenProgram"
        }
      ],
      "args": []
    },
    {
      "name": "removeFromWhitelist",
      "discriminator": [
        7,
        144,
        216,
        239,
        243,
        236,
        193,
        235
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint"
        },
        {
          "name": "oracleAccount",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  97,
                  99,
                  108,
                  101,
                  95,
                  97,
                  99,
                  99,
                  111,
                  117,
                  110,
                  116
                ]
              }
            ],
            "program": {
              "kind": "account",
              "path": "oracleProgram"
            }
          }
        },
        {
          "name": "oracleProgram",
          "address": "oracynDbHWzsfV7GaQveWhqrAsB1LNq6pr4ftRYME4c"
        }
      ],
      "args": []
    },
    {
      "name": "stakeBtg",
      "discriminator": [
        154,
        220,
        56,
        94,
        110,
        227,
        249,
        26
      ],
      "accounts": [
        {
          "name": "stakingVault",
          "writable": true,
          "signer": true
        },
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "oracleAccount",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  97,
                  99,
                  108,
                  101,
                  95,
                  97,
                  99,
                  99,
                  111,
                  117,
                  110,
                  116
                ]
              }
            ],
            "program": {
              "kind": "account",
              "path": "oracleProgram"
            }
          }
        },
        {
          "name": "oracleProgram",
          "address": "oracynDbHWzsfV7GaQveWhqrAsB1LNq6pr4ftRYME4c"
        },
        {
          "name": "userTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "user"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                146,
                254,
                170,
                189,
                14,
                251,
                208,
                173,
                193,
                77,
                189,
                183,
                143,
                165,
                133,
                83,
                112,
                168,
                140,
                159,
                214,
                120,
                107,
                2,
                254,
                70,
                202,
                55,
                107,
                110,
                241,
                192
              ]
            }
          }
        },
        {
          "name": "associatedTokenProgram",
          "address": "AtokenhZ6AE34VMYRv1AqSv8q8QZJxxEaY1zKiXKwSWT"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "oracleAccount",
      "discriminator": [
        189,
        23,
        49,
        197,
        79,
        96,
        245,
        91
      ]
    },
    {
      "name": "stakeConfig",
      "discriminator": [
        238,
        151,
        43,
        3,
        11,
        151,
        63,
        176
      ]
    },
    {
      "name": "stakingVault",
      "discriminator": [
        68,
        141,
        118,
        28,
        87,
        84,
        213,
        233
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "unauthorized",
      "msg": "unauthorized"
    },
    {
      "code": 6001,
      "name": "invalidToken",
      "msg": "Invalid token - not in whitelist"
    },
    {
      "code": 6002,
      "name": "tokenAlreadyExists",
      "msg": "The token already exists in the list."
    },
    {
      "code": 6003,
      "name": "invalidMintAddress",
      "msg": "The provided mint address is invalid."
    },
    {
      "code": 6004,
      "name": "amountTooSmall",
      "msg": "The staking amount is too small."
    },
    {
      "code": 6005,
      "name": "insufficientFunds",
      "msg": "The user does not have enough funds."
    },
    {
      "code": 6006,
      "name": "invalidMintAuthority",
      "msg": "The mint authority is not controlled by the program."
    }
  ],
  "types": [
    {
      "name": "oracleAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "prices",
            "type": {
              "vec": {
                "defined": {
                  "name": "priceEntry"
                }
              }
            }
          },
          {
            "name": "priceAuthorities",
            "type": {
              "vec": "pubkey"
            }
          }
        ]
      }
    },
    {
      "name": "priceEntry",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "source",
            "type": "string"
          },
          {
            "name": "currencyType",
            "type": "u8"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "price",
            "type": "f64"
          },
          {
            "name": "updatedAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "stakeConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "tokens",
            "type": {
              "vec": {
                "defined": {
                  "name": "tokenInfo"
                }
              }
            }
          }
        ]
      }
    },
    {
      "name": "stakingVault",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "btgAmount",
            "type": "u64"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "tokenPrice",
            "type": "f64"
          },
          {
            "name": "btgPrice",
            "type": "f64"
          },
          {
            "name": "outputRate",
            "type": "f64"
          },
          {
            "name": "outputTokenAmount",
            "type": "u64"
          },
          {
            "name": "time",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "tokenInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "outputRate",
            "type": "f64"
          }
        ]
      }
    }
  ]
};
