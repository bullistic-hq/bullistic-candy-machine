export type BullisticCandyMachine = {
  accounts: [
    {
      docs: ["Candy machine state and config data."];
      name: "candyMachine";
      type: {
        fields: [
          { name: "bullisticAuthority"; type: "publicKey" },
          { name: "creatorAuthority"; type: "publicKey" },
          { name: "treasuryWallet"; type: "publicKey" },
          { name: "treasuryMint"; type: { option: "publicKey" } },
          { name: "itemsRedeemed"; type: "u64" },
          { name: "data"; type: { defined: "CandyMachineData" } }
        ];
        kind: "struct";
      };
    },
    {
      name: "buyerInfoAccount";
      type: {
        fields: [
          {
            docs: ["Number bought during the Merkle allowlist phase."];
            name: "numberBoughtMerkleAllowlistPhase";
            type: "u16";
          },
          {
            docs: ["Number bought during the public phase."];
            name: "numberBoughtPublicPhase";
            type: "u16";
          }
        ];
        kind: "struct";
      };
    },
    {
      docs: ["Collection PDA account"];
      name: "collectionPda";
      type: {
        fields: [
          { name: "mint"; type: "publicKey" },
          { name: "candyMachine"; type: "publicKey" }
        ];
        kind: "struct";
      };
    },
    {
      docs: ["Collection PDA account"];
      name: "freezePda";
      type: {
        fields: [
          { name: "candyMachine"; type: "publicKey" },
          { name: "allowThaw"; type: "bool" },
          { name: "frozenCount"; type: "u64" },
          { name: "mintStart"; type: { option: "i64" } },
          { name: "freezeTime"; type: "i64" },
          { name: "freezeFee"; type: "u64" }
        ];
        kind: "struct";
      };
    }
  ];
  errors: [
    {
      code: 8000;
      msg: "Account does not have correct owner!";
      name: "IncorrectOwner";
    },
    { code: 8001; msg: "Account is not initialized!"; name: "Uninitialized" },
    { code: 8002; msg: "Mint Mismatch!"; name: "MintMismatch" },
    {
      code: 8003;
      msg: "Index greater than length!";
      name: "IndexGreaterThanLength";
    },
    {
      code: 8004;
      msg: "Numerical overflow error!";
      name: "NumericalOverflowError";
    },
    {
      code: 8005;
      msg: "Can only provide up to 4 creators to candy machine (because candy machine is one)!";
      name: "TooManyCreators";
    },
    {
      code: 8006;
      msg: "Uuid must be exactly of 6 length.";
      name: "UuidMustBeExactly6Length";
    },
    {
      code: 8007;
      msg: "Not enough tokens to pay for this minting.";
      name: "NotEnoughTokens";
    },
    {
      code: 8008;
      msg: "Not enough SOL to pay for this minting.";
      name: "NotEnoughSOL";
    },
    { code: 8009; msg: "Token transfer failed."; name: "TokenTransferFailed" },
    { code: 8010; msg: "Candy machine is empty!"; name: "CandyMachineEmpty" },
    {
      code: 8011;
      msg: "Candy machine public sale is not live!";
      name: "CandyMachinePublicSaleNotLive";
    },
    {
      code: 8012;
      msg: "Configs that are using hidden uris do not have config lines, they have a single hash representing hashed order.";
      name: "HiddenSettingsConfigsDoNotHaveConfigLines";
    },
    {
      code: 8013;
      msg: "Cannot change number of lines unless is a hidden config.";
      name: "CannotChangeNumberOfLines";
    },
    { code: 8014; msg: "Public key mismatch."; name: "PublicKeyMismatch" },
    {
      code: 8015;
      msg: "No SPL allowlist token present.";
      name: "NoSplAllowlistToken";
    },
    { code: 8016; msg: "Token burn failed."; name: "TokenBurnFailed" },
    {
      code: 8017;
      msg: "Unable to find an unused config line near your random number index.";
      name: "CannotFindUsableConfigLine";
    },
    { code: 8018; msg: "Invalid string."; name: "InvalidString" },
    {
      code: 8019;
      msg: "Suspicious transaction detected.";
      name: "SuspiciousTransaction";
    },
    {
      code: 8020;
      msg: "Cannot Switch to Hidden Settings after items available is greater than 0.";
      name: "CannotSwitchToHiddenSettings";
    },
    {
      code: 8021;
      msg: "Incorrect SlotHashes PubKey.";
      name: "IncorrectSlotHashesPubkey";
    },
    {
      code: 8022;
      msg: "Incorrect collection NFT authority.";
      name: "IncorrectCollectionAuthority";
    },
    {
      code: 8023;
      msg: "Collection PDA address is invalid.";
      name: "MismatchedCollectionPda";
    },
    {
      code: 8024;
      msg: "Provided mint account doesn't match collection PDA mint.";
      name: "MismatchedCollectionMint";
    },
    {
      code: 8025;
      msg: "Slot hashes Sysvar is empty.";
      name: "SlotHashesEmpty";
    },
    {
      code: 8026;
      msg: "The metadata account has data in it, and this must be empty to mint a new NFT.";
      name: "MetadataAccountMustBeEmpty";
    },
    {
      code: 8027;
      msg: "Missing set collection during mint IX for Candy Machine with collection set.";
      name: "MissingSetCollectionDuringMint";
    },
    {
      code: 8028;
      msg: "Can't change collection settings after items have begun to be minted.";
      name: "NoChangingCollectionDuringMint";
    },
    {
      code: 8029;
      msg: "Can't change freeze settings after items have begun to be minted. You can only disable.";
      name: "NoChangingFreezeDuringMint";
    },
    {
      code: 8030;
      msg: "Can't change authority while freeze is enabled. Disable freeze first.";
      name: "NoChangingAuthorityWithFreeze";
    },
    {
      code: 8031;
      msg: "Can't change token while freeze is enabled. Disable freeze first.";
      name: "NoChangingTokenWithFreeze";
    },
    {
      code: 8032;
      msg: "Cannot thaw NFT unless all NFTs are minted or Candy Machine authority enables thawing.";
      name: "InvalidThawNft";
    },
    {
      code: 8033;
      msg: "The number of remaining accounts passed in doesn't match the Candy Machine settings.";
      name: "IncorrectRemainingAccountsLen";
    },
    {
      code: 8034;
      msg: "FreezePda ATA needs to be passed in if token mint is enabled.";
      name: "MissingFreezeAta";
    },
    {
      code: 8035;
      msg: "Incorrect freeze ATA address.";
      name: "IncorrectFreezeAta";
    },
    {
      code: 8036;
      msg: "FreezePda account doesn't belong to this Candy Machine.";
      name: "FreezePdaMismatch";
    },
    {
      code: 8037;
      msg: "Freeze time can't be longer than MAX_FREEZE_TIME.";
      name: "EnteredFreezeIsMoreThanMaxFreeze";
    },
    {
      code: 8038;
      msg: "Can't withdraw Candy Machine while freeze is active. Disable freeze first.";
      name: "NoWithdrawWithFreeze";
    },
    {
      code: 8039;
      msg: "Can't withdraw Candy Machine while frozen funds need to be redeemed. Unlock funds first.";
      name: "NoWithdrawWithFrozenFunds";
    },
    {
      code: 8040;
      msg: "Missing required remaining accounts for remove_freeze with token mint.";
      name: "MissingRemoveFreezeTokenAccounts";
    },
    {
      code: 8041;
      msg: "Can't withdraw SPL Token from freeze PDA into itself.";
      name: "InvalidFreezeWithdrawTokenAddress";
    },
    {
      code: 8042;
      msg: "Can't unlock funds while NFTs are still frozen. Run thaw on all NFTs first.";
      name: "NoUnlockWithNFTsStillFrozen";
    },
    {
      code: 8043;
      msg: "Invalid bot signer authority.";
      name: "InvalidBotSignerAuthority";
    },
    {
      code: 8044;
      msg: "The wallet has already minted the maximum number allowed.";
      name: "BuyLimitPerAddressExceeded";
    },
    {
      code: 8045;
      msg: "The provided merkle allowlist proof is invalid.";
      name: "InvalidAllowlistProof";
    },
    {
      code: 8046;
      msg: "All available allowlist mints have already been claimed by this address.";
      name: "AllowlistMintsAlreadyUsed";
    },
    {
      code: 8047;
      msg: "Maximum roots list count exceeded.";
      name: "MaximumRootCountExceeded";
    },
    {
      code: 8048;
      msg: "Can only provide up to 5 omni mint wallets.";
      name: "TooManyOmniMintWallets";
    },
    {
      code: 8049;
      msg: "Candy machine allowlist sale is not live.";
      name: "CandyMachineAllowlistSaleNotLive";
    },
    {
      code: 8050;
      msg: "Candy machine public sale has ended.";
      name: "CandyMachinePublicSaleEnded";
    },
    {
      code: 8051;
      msg: "Invalid candy machine mint phase times provided.";
      name: "CandyMachineInvalidMintPhases";
    },
    { code: 8052; msg: "Bot tax collected."; name: "BotTaxCollected" },
    {
      code: 8053;
      msg: "Invalid mint price provided.";
      name: "InvalidMintPrice";
    },
    {
      code: 8054;
      msg: "Invalid allowlist settings. Can only enable a single allowlist feature at a time.";
      name: "InvalidAllowlistSettings";
    }
  ];
  instructions: [
    {
      accounts: [
        { isMut: true; isSigner: false; name: "candyMachine" },
        { isMut: false; isSigner: true; name: "bullisticAuthority" }
      ];
      args: [
        { name: "index"; type: "u32" },
        { name: "configLines"; type: { vec: { defined: "ConfigLine" } } }
      ];
      name: "addConfigLines";
    },
    {
      accounts: [
        { isMut: false; isSigner: true; name: "bullisticAuthority" },
        { isMut: true; isSigner: false; name: "candyMachine" }
      ];
      args: [{ name: "rootsToAppend"; type: { vec: { array: ["u8", 32] } } }];
      name: "appendMerkleAllowlistRoots";
    },
    {
      accounts: [
        { isMut: false; isSigner: true; name: "bullisticAuthority" },
        { isMut: true; isSigner: false; name: "candyMachine" }
      ];
      args: [];
      name: "clearMerkleAllowlistRoots";
    },
    {
      accounts: [
        { isMut: true; isSigner: false; name: "candyMachine" },
        { isMut: false; isSigner: false; name: "treasuryWallet" },
        { isMut: false; isSigner: false; name: "bullisticAuthority" },
        { isMut: false; isSigner: false; name: "creatorAuthority" },
        { isMut: false; isSigner: true; name: "payer" },
        { isMut: false; isSigner: false; name: "systemProgram" },
        { isMut: false; isSigner: false; name: "rent" }
      ];
      args: [{ name: "data"; type: { defined: "CandyMachineData" } }];
      name: "initializeCandyMachine";
    },
    {
      accounts: [
        { isMut: true; isSigner: false; name: "candyMachine" },
        { isMut: false; isSigner: false; name: "candyMachineCreator" },
        { isMut: false; isSigner: true; name: "buyer" },
        { isMut: true; isSigner: false; name: "treasuryWallet" },
        { isMut: true; isSigner: false; name: "metadata" },
        { isMut: true; isSigner: true; name: "mint" },
        { isMut: false; isSigner: false; name: "creatorAuthority" },
        { isMut: true; isSigner: false; name: "masterEdition" },
        { isMut: false; isSigner: false; name: "tokenMetadataProgram" },
        { isMut: false; isSigner: false; name: "tokenProgram" },
        { isMut: false; isSigner: false; name: "systemProgram" },
        { isMut: false; isSigner: false; name: "rent" },
        { isMut: false; isSigner: false; name: "recentSlothashes" },
        { isMut: false; isSigner: false; name: "instructionSysvarAccount" },
        { isMut: false; isSigner: false; name: "botSignerAuthority" },
        { isMut: true; isSigner: false; name: "buyerInfoAccount" },
        { isMut: true; isSigner: false; name: "buyerTokenAccount" },
        { isMut: false; isSigner: false; name: "ataProgram" }
      ];
      args: [
        { name: "creatorBump"; type: "u8" },
        { name: "buyerInfoAccountBump"; type: "u8" },
        {
          name: "buyerMerkleAllowlistProofData";
          type: { option: { defined: "BuyerMerkleAllowlistProofData" } };
        },
        { name: "expectedPrice"; type: "u64" }
      ];
      name: "mintNft";
    },
    {
      accounts: [
        { isMut: true; isSigner: false; name: "candyMachine" },
        { isMut: false; isSigner: true; name: "bullisticAuthority" },
        { isMut: true; isSigner: false; name: "collectionPda" },
        { isMut: false; isSigner: false; name: "metadata" },
        { isMut: false; isSigner: false; name: "mint" },
        { isMut: true; isSigner: false; name: "collectionAuthorityRecord" },
        { isMut: false; isSigner: false; name: "tokenMetadataProgram" }
      ];
      args: [];
      name: "removeCollection";
    },
    {
      accounts: [
        { isMut: true; isSigner: false; name: "candyMachine" },
        { isMut: true; isSigner: true; name: "bullisticAuthority" },
        { isMut: true; isSigner: false; name: "freezePda" }
      ];
      args: [];
      name: "removeFreeze";
    },
    {
      accounts: [
        { isMut: true; isSigner: false; name: "candyMachine" },
        { isMut: false; isSigner: true; name: "bullisticAuthority" },
        { isMut: true; isSigner: false; name: "creatorAuthority" },
        { isMut: true; isSigner: false; name: "collectionPda" },
        { isMut: false; isSigner: true; name: "payer" },
        { isMut: false; isSigner: false; name: "systemProgram" },
        { isMut: false; isSigner: false; name: "rent" },
        { isMut: true; isSigner: false; name: "metadata" },
        { isMut: false; isSigner: false; name: "mint" },
        { isMut: false; isSigner: false; name: "edition" },
        { isMut: true; isSigner: false; name: "collectionAuthorityRecord" },
        { isMut: false; isSigner: false; name: "tokenMetadataProgram" }
      ];
      args: [];
      name: "setCollection";
    },
    {
      accounts: [
        { isMut: false; isSigner: false; name: "candyMachine" },
        { isMut: false; isSigner: false; name: "metadata" },
        { isMut: false; isSigner: true; name: "buyer" },
        { isMut: true; isSigner: false; name: "collectionPda" },
        { isMut: false; isSigner: false; name: "tokenMetadataProgram" },
        { isMut: false; isSigner: false; name: "instructionSysvarAccount" },
        { isMut: false; isSigner: false; name: "collectionMint" },
        { isMut: false; isSigner: false; name: "collectionMetadata" },
        { isMut: false; isSigner: false; name: "collectionMasterEdition" },
        { isMut: false; isSigner: false; name: "creatorAuthority" },
        { isMut: false; isSigner: false; name: "collectionAuthorityRecord" }
      ];
      args: [];
      name: "setCollectionDuringMint";
    },
    {
      accounts: [
        { isMut: true; isSigner: false; name: "candyMachine" },
        { isMut: true; isSigner: true; name: "bullisticAuthority" },
        { isMut: true; isSigner: false; name: "freezePda" },
        { isMut: false; isSigner: false; name: "systemProgram" }
      ];
      args: [{ name: "freezeTime"; type: "i64" }];
      name: "setFreeze";
    },
    {
      accounts: [
        { isMut: true; isSigner: false; name: "freezePda" },
        { isMut: true; isSigner: false; name: "candyMachine" },
        { isMut: true; isSigner: false; name: "tokenAccount" },
        { isMut: false; isSigner: false; name: "owner" },
        { isMut: false; isSigner: false; name: "mint" },
        { isMut: false; isSigner: false; name: "edition" },
        { isMut: true; isSigner: true; name: "payer" },
        { isMut: false; isSigner: false; name: "tokenProgram" },
        { isMut: false; isSigner: false; name: "tokenMetadataProgram" },
        { isMut: false; isSigner: false; name: "systemProgram" }
      ];
      args: [];
      name: "thawNft";
    },
    {
      accounts: [
        { isMut: true; isSigner: false; name: "candyMachine" },
        { isMut: true; isSigner: true; name: "bullisticAuthority" },
        { isMut: true; isSigner: false; name: "freezePda" },
        { isMut: false; isSigner: false; name: "systemProgram" }
      ];
      args: [];
      name: "unlockFunds";
    },
    {
      accounts: [
        { isMut: true; isSigner: false; name: "candyMachine" },
        { isMut: false; isSigner: true; name: "bullisticAuthority" },
        { isMut: false; isSigner: false; name: "treasuryWallet" }
      ];
      args: [{ name: "newAuthority"; type: { option: "publicKey" } }];
      name: "updateAuthority";
    },
    {
      accounts: [
        { isMut: true; isSigner: false; name: "candyMachine" },
        { isMut: false; isSigner: true; name: "bullisticAuthority" },
        { isMut: false; isSigner: false; name: "treasuryWallet" }
      ];
      args: [{ name: "data"; type: { defined: "CandyMachineData" } }];
      name: "updateCandyMachine";
    },
    {
      accounts: [
        { isMut: true; isSigner: false; name: "candyMachine" },
        { isMut: true; isSigner: true; name: "bullisticAuthority" }
      ];
      args: [];
      name: "withdrawFunds";
    }
  ];
  instructionsMap: {
    addConfigLines: ["candyMachine", "bullisticAuthority"];
    appendMerkleAllowlistRoots: ["bullisticAuthority", "candyMachine"];
    clearMerkleAllowlistRoots: ["bullisticAuthority", "candyMachine"];
    initializeCandyMachine: [
      "candyMachine",
      "treasuryWallet",
      "bullisticAuthority",
      "creatorAuthority",
      "payer",
      "systemProgram",
      "rent"
    ];
    mintNft: [
      "candyMachine",
      "candyMachineCreator",
      "buyer",
      "treasuryWallet",
      "metadata",
      "mint",
      "creatorAuthority",
      "masterEdition",
      "tokenMetadataProgram",
      "tokenProgram",
      "systemProgram",
      "rent",
      "recentSlothashes",
      "instructionSysvarAccount",
      "botSignerAuthority",
      "buyerInfoAccount",
      "buyerTokenAccount",
      "ataProgram"
    ];
    removeCollection: [
      "candyMachine",
      "bullisticAuthority",
      "collectionPda",
      "metadata",
      "mint",
      "collectionAuthorityRecord",
      "tokenMetadataProgram"
    ];
    removeFreeze: ["candyMachine", "bullisticAuthority", "freezePda"];
    setCollection: [
      "candyMachine",
      "bullisticAuthority",
      "creatorAuthority",
      "collectionPda",
      "payer",
      "systemProgram",
      "rent",
      "metadata",
      "mint",
      "edition",
      "collectionAuthorityRecord",
      "tokenMetadataProgram"
    ];
    setCollectionDuringMint: [
      "candyMachine",
      "metadata",
      "buyer",
      "collectionPda",
      "tokenMetadataProgram",
      "instructionSysvarAccount",
      "collectionMint",
      "collectionMetadata",
      "collectionMasterEdition",
      "creatorAuthority",
      "collectionAuthorityRecord"
    ];
    setFreeze: [
      "candyMachine",
      "bullisticAuthority",
      "freezePda",
      "systemProgram"
    ];
    thawNft: [
      "freezePda",
      "candyMachine",
      "tokenAccount",
      "owner",
      "mint",
      "edition",
      "payer",
      "tokenProgram",
      "tokenMetadataProgram",
      "systemProgram"
    ];
    unlockFunds: [
      "candyMachine",
      "bullisticAuthority",
      "freezePda",
      "systemProgram"
    ];
    updateAuthority: ["candyMachine", "bullisticAuthority", "treasuryWallet"];
    updateCandyMachine: ["candyMachine", "bullisticAuthority", "treasuryWallet"];
    withdrawFunds: ["candyMachine", "bullisticAuthority"];
  };
  name: "bullistic_candy_machine";
  types: [
    {
      docs: ["Candy machine settings data."];
      name: "CandyMachineData";
      type: {
        fields: [
          { name: "uuid"; type: "string" },
          { name: "price"; type: "u64" },
          { name: "premintPrice"; type: { option: "u64" } },
          { name: "allowlistPrice"; type: { option: "u64" } },
          {
            docs: ["The symbol for the asset"];
            name: "symbol";
            type: "string";
          },
          {
            docs: [
              "Royalty basis points that goes to creators in secondary sales (0-10000)"
            ];
            name: "sellerFeeBasisPoints";
            type: "u16";
          },
          { name: "maxSupply"; type: "u64" },
          { name: "itemsAvailable"; type: "u64" },
          { name: "isMutable"; type: "bool" },
          { name: "allowlistSaleStartTime"; type: { option: "i64" } },
          { name: "publicSaleStartTime"; type: "i64" },
          { name: "publicSaleEndTime"; type: "i64" },
          { name: "creators"; type: { vec: { defined: "Creator" } } },
          { name: "omniMintWallets"; type: { vec: "publicKey" } },
          {
            name: "hiddenSettings";
            type: { option: { defined: "HiddenSettings" } };
          },
          { name: "botProtectionEnabled"; type: "bool" },
          { name: "limitPerAddress"; type: "u16" },
          { name: "sequentialMintOrderEnabled"; type: "bool" },
          {
            name: "merkleAllowlistRootList";
            type: { vec: { array: ["u8", 32] } };
          },
          {
            name: "splTokenAllowlistSettings";
            type: { option: { defined: "SplTokenAllowlistSettings" } };
          }
        ];
        kind: "struct";
      };
    },
    {
      docs: ["Individual config line for storing NFT data pre-mint."];
      name: "ConfigLine";
      type: {
        fields: [
          { name: "name"; type: "string" },
          {
            docs: ["URI pointing to JSON representing the asset"];
            name: "uri";
            type: "string";
          }
        ];
        kind: "struct";
      };
    },
    {
      name: "Creator";
      type: {
        fields: [
          { name: "address"; type: "publicKey" },
          { name: "verified"; type: "bool" },
          { name: "share"; type: "u8" }
        ];
        kind: "struct";
      };
    },
    {
      docs: ["Hidden Settings for large mints used with offline data."];
      name: "HiddenSettings";
      type: {
        fields: [
          { name: "name"; type: "string" },
          { name: "uri"; type: "string" },
          { name: "hash"; type: { array: ["u8", 32] } }
        ];
        kind: "struct";
      };
    },
    {
      name: "BuyerMerkleAllowlistProofData";
      type: {
        fields: [
          { name: "amount"; type: "u16" },
          { name: "proof"; type: { vec: { array: ["u8", 32] } } },
          { name: "rootIndexForProof"; type: "u16" }
        ];
        kind: "struct";
      };
    },
    {
      name: "SplTokenAllowlistSettings";
      type: {
        fields: [
          { name: "mode"; type: { defined: "SplTokenAllowlistMode" } },
          { name: "mint"; type: "publicKey" }
        ];
        kind: "struct";
      };
    },
    {
      name: "MintPhase";
      type: {
        kind: "enum";
        variants: [
          { name: "Premint" },
          { name: "Allowlist" },
          { name: "Public" },
          { name: "Expired" }
        ];
      };
    },
    {
      name: "SplTokenAllowlistMode";
      type: {
        kind: "enum";
        variants: [{ name: "BurnEveryTime" }, { name: "NeverBurn" }];
      };
    }
  ];
  version: "0.0.1";
};
export const IDL: BullisticCandyMachine = {
  accounts: [
    {
      docs: ["Candy machine state and config data."],
      name: "candyMachine",
      type: {
        fields: [
          { name: "bullisticAuthority", type: "publicKey" },
          { name: "creatorAuthority", type: "publicKey" },
          { name: "treasuryWallet", type: "publicKey" },
          { name: "treasuryMint", type: { option: "publicKey" } },
          { name: "itemsRedeemed", type: "u64" },
          { name: "data", type: { defined: "CandyMachineData" } },
        ],
        kind: "struct",
      },
    },
    {
      name: "buyerInfoAccount",
      type: {
        fields: [
          {
            docs: ["Number bought during the Merkle allowlist phase."],
            name: "numberBoughtMerkleAllowlistPhase",
            type: "u16",
          },
          {
            docs: ["Number bought during the public phase."],
            name: "numberBoughtPublicPhase",
            type: "u16",
          },
        ],
        kind: "struct",
      },
    },
    {
      docs: ["Collection PDA account"],
      name: "collectionPda",
      type: {
        fields: [
          { name: "mint", type: "publicKey" },
          { name: "candyMachine", type: "publicKey" },
        ],
        kind: "struct",
      },
    },
    {
      docs: ["Collection PDA account"],
      name: "freezePda",
      type: {
        fields: [
          { name: "candyMachine", type: "publicKey" },
          { name: "allowThaw", type: "bool" },
          { name: "frozenCount", type: "u64" },
          { name: "mintStart", type: { option: "i64" } },
          { name: "freezeTime", type: "i64" },
          { name: "freezeFee", type: "u64" },
        ],
        kind: "struct",
      },
    },
  ],
  errors: [
    {
      code: 8000,
      msg: "Account does not have correct owner!",
      name: "IncorrectOwner",
    },
    { code: 8001, msg: "Account is not initialized!", name: "Uninitialized" },
    { code: 8002, msg: "Mint Mismatch!", name: "MintMismatch" },
    {
      code: 8003,
      msg: "Index greater than length!",
      name: "IndexGreaterThanLength",
    },
    {
      code: 8004,
      msg: "Numerical overflow error!",
      name: "NumericalOverflowError",
    },
    {
      code: 8005,
      msg: "Can only provide up to 4 creators to candy machine (because candy machine is one)!",
      name: "TooManyCreators",
    },
    {
      code: 8006,
      msg: "Uuid must be exactly of 6 length.",
      name: "UuidMustBeExactly6Length",
    },
    {
      code: 8007,
      msg: "Not enough tokens to pay for this minting.",
      name: "NotEnoughTokens",
    },
    {
      code: 8008,
      msg: "Not enough SOL to pay for this minting.",
      name: "NotEnoughSOL",
    },
    { code: 8009, msg: "Token transfer failed.", name: "TokenTransferFailed" },
    { code: 8010, msg: "Candy machine is empty!", name: "CandyMachineEmpty" },
    {
      code: 8011,
      msg: "Candy machine public sale is not live!",
      name: "CandyMachinePublicSaleNotLive",
    },
    {
      code: 8012,
      msg: "Configs that are using hidden uris do not have config lines, they have a single hash representing hashed order.",
      name: "HiddenSettingsConfigsDoNotHaveConfigLines",
    },
    {
      code: 8013,
      msg: "Cannot change number of lines unless is a hidden config.",
      name: "CannotChangeNumberOfLines",
    },
    { code: 8014, msg: "Public key mismatch.", name: "PublicKeyMismatch" },
    {
      code: 8015,
      msg: "No SPL allowlist token present.",
      name: "NoSplAllowlistToken",
    },
    { code: 8016, msg: "Token burn failed.", name: "TokenBurnFailed" },
    {
      code: 8017,
      msg: "Unable to find an unused config line near your random number index.",
      name: "CannotFindUsableConfigLine",
    },
    { code: 8018, msg: "Invalid string.", name: "InvalidString" },
    {
      code: 8019,
      msg: "Suspicious transaction detected.",
      name: "SuspiciousTransaction",
    },
    {
      code: 8020,
      msg: "Cannot Switch to Hidden Settings after items available is greater than 0.",
      name: "CannotSwitchToHiddenSettings",
    },
    {
      code: 8021,
      msg: "Incorrect SlotHashes PubKey.",
      name: "IncorrectSlotHashesPubkey",
    },
    {
      code: 8022,
      msg: "Incorrect collection NFT authority.",
      name: "IncorrectCollectionAuthority",
    },
    {
      code: 8023,
      msg: "Collection PDA address is invalid.",
      name: "MismatchedCollectionPda",
    },
    {
      code: 8024,
      msg: "Provided mint account doesn't match collection PDA mint.",
      name: "MismatchedCollectionMint",
    },
    {
      code: 8025,
      msg: "Slot hashes Sysvar is empty.",
      name: "SlotHashesEmpty",
    },
    {
      code: 8026,
      msg: "The metadata account has data in it, and this must be empty to mint a new NFT.",
      name: "MetadataAccountMustBeEmpty",
    },
    {
      code: 8027,
      msg: "Missing set collection during mint IX for Candy Machine with collection set.",
      name: "MissingSetCollectionDuringMint",
    },
    {
      code: 8028,
      msg: "Can't change collection settings after items have begun to be minted.",
      name: "NoChangingCollectionDuringMint",
    },
    {
      code: 8029,
      msg: "Can't change freeze settings after items have begun to be minted. You can only disable.",
      name: "NoChangingFreezeDuringMint",
    },
    {
      code: 8030,
      msg: "Can't change authority while freeze is enabled. Disable freeze first.",
      name: "NoChangingAuthorityWithFreeze",
    },
    {
      code: 8031,
      msg: "Can't change token while freeze is enabled. Disable freeze first.",
      name: "NoChangingTokenWithFreeze",
    },
    {
      code: 8032,
      msg: "Cannot thaw NFT unless all NFTs are minted or Candy Machine authority enables thawing.",
      name: "InvalidThawNft",
    },
    {
      code: 8033,
      msg: "The number of remaining accounts passed in doesn't match the Candy Machine settings.",
      name: "IncorrectRemainingAccountsLen",
    },
    {
      code: 8034,
      msg: "FreezePda ATA needs to be passed in if token mint is enabled.",
      name: "MissingFreezeAta",
    },
    {
      code: 8035,
      msg: "Incorrect freeze ATA address.",
      name: "IncorrectFreezeAta",
    },
    {
      code: 8036,
      msg: "FreezePda account doesn't belong to this Candy Machine.",
      name: "FreezePdaMismatch",
    },
    {
      code: 8037,
      msg: "Freeze time can't be longer than MAX_FREEZE_TIME.",
      name: "EnteredFreezeIsMoreThanMaxFreeze",
    },
    {
      code: 8038,
      msg: "Can't withdraw Candy Machine while freeze is active. Disable freeze first.",
      name: "NoWithdrawWithFreeze",
    },
    {
      code: 8039,
      msg: "Can't withdraw Candy Machine while frozen funds need to be redeemed. Unlock funds first.",
      name: "NoWithdrawWithFrozenFunds",
    },
    {
      code: 8040,
      msg: "Missing required remaining accounts for remove_freeze with token mint.",
      name: "MissingRemoveFreezeTokenAccounts",
    },
    {
      code: 8041,
      msg: "Can't withdraw SPL Token from freeze PDA into itself.",
      name: "InvalidFreezeWithdrawTokenAddress",
    },
    {
      code: 8042,
      msg: "Can't unlock funds while NFTs are still frozen. Run thaw on all NFTs first.",
      name: "NoUnlockWithNFTsStillFrozen",
    },
    {
      code: 8043,
      msg: "Invalid bot signer authority.",
      name: "InvalidBotSignerAuthority",
    },
    {
      code: 8044,
      msg: "The wallet has already minted the maximum number allowed.",
      name: "BuyLimitPerAddressExceeded",
    },
    {
      code: 8045,
      msg: "The provided merkle allowlist proof is invalid.",
      name: "InvalidAllowlistProof",
    },
    {
      code: 8046,
      msg: "All available allowlist mints have already been claimed by this address.",
      name: "AllowlistMintsAlreadyUsed",
    },
    {
      code: 8047,
      msg: "Maximum roots list count exceeded.",
      name: "MaximumRootCountExceeded",
    },
    {
      code: 8048,
      msg: "Can only provide up to 5 omni mint wallets.",
      name: "TooManyOmniMintWallets",
    },
    {
      code: 8049,
      msg: "Candy machine allowlist sale is not live.",
      name: "CandyMachineAllowlistSaleNotLive",
    },
    {
      code: 8050,
      msg: "Candy machine public sale has ended.",
      name: "CandyMachinePublicSaleEnded",
    },
    {
      code: 8051,
      msg: "Invalid candy machine mint phase times provided.",
      name: "CandyMachineInvalidMintPhases",
    },
    { code: 8052, msg: "Bot tax collected.", name: "BotTaxCollected" },
    {
      code: 8053,
      msg: "Invalid mint price provided.",
      name: "InvalidMintPrice",
    },
    {
      code: 8054,
      msg: "Invalid allowlist settings. Can only enable a single allowlist feature at a time.",
      name: "InvalidAllowlistSettings",
    },
  ],
  instructions: [
    {
      accounts: [
        { isMut: true, isSigner: false, name: "candyMachine" },
        { isMut: false, isSigner: true, name: "bullisticAuthority" },
      ],
      args: [
        { name: "index", type: "u32" },
        { name: "configLines", type: { vec: { defined: "ConfigLine" } } },
      ],
      name: "addConfigLines",
    },
    {
      accounts: [
        { isMut: false, isSigner: true, name: "bullisticAuthority" },
        { isMut: true, isSigner: false, name: "candyMachine" },
      ],
      args: [{ name: "rootsToAppend", type: { vec: { array: ["u8", 32] } } }],
      name: "appendMerkleAllowlistRoots",
    },
    {
      accounts: [
        { isMut: false, isSigner: true, name: "bullisticAuthority" },
        { isMut: true, isSigner: false, name: "candyMachine" },
      ],
      args: [],
      name: "clearMerkleAllowlistRoots",
    },
    {
      accounts: [
        { isMut: true, isSigner: false, name: "candyMachine" },
        { isMut: false, isSigner: false, name: "treasuryWallet" },
        { isMut: false, isSigner: false, name: "bullisticAuthority" },
        { isMut: false, isSigner: false, name: "creatorAuthority" },
        { isMut: false, isSigner: true, name: "payer" },
        { isMut: false, isSigner: false, name: "systemProgram" },
        { isMut: false, isSigner: false, name: "rent" },
      ],
      args: [{ name: "data", type: { defined: "CandyMachineData" } }],
      name: "initializeCandyMachine",
    },
    {
      accounts: [
        { isMut: true, isSigner: false, name: "candyMachine" },
        { isMut: false, isSigner: false, name: "candyMachineCreator" },
        { isMut: false, isSigner: true, name: "buyer" },
        { isMut: true, isSigner: false, name: "treasuryWallet" },
        { isMut: true, isSigner: false, name: "metadata" },
        { isMut: true, isSigner: true, name: "mint" },
        { isMut: false, isSigner: false, name: "creatorAuthority" },
        { isMut: true, isSigner: false, name: "masterEdition" },
        { isMut: false, isSigner: false, name: "tokenMetadataProgram" },
        { isMut: false, isSigner: false, name: "tokenProgram" },
        { isMut: false, isSigner: false, name: "systemProgram" },
        { isMut: false, isSigner: false, name: "rent" },
        { isMut: false, isSigner: false, name: "recentSlothashes" },
        { isMut: false, isSigner: false, name: "instructionSysvarAccount" },
        { isMut: false, isSigner: false, name: "botSignerAuthority" },
        { isMut: true, isSigner: false, name: "buyerInfoAccount" },
        { isMut: true, isSigner: false, name: "buyerTokenAccount" },
        { isMut: false, isSigner: false, name: "ataProgram" },
      ],
      args: [
        { name: "creatorBump", type: "u8" },
        { name: "buyerInfoAccountBump", type: "u8" },
        {
          name: "buyerMerkleAllowlistProofData",
          type: { option: { defined: "BuyerMerkleAllowlistProofData" } },
        },
        { name: "expectedPrice", type: "u64" },
      ],
      name: "mintNft",
    },
    {
      accounts: [
        { isMut: true, isSigner: false, name: "candyMachine" },
        { isMut: false, isSigner: true, name: "bullisticAuthority" },
        { isMut: true, isSigner: false, name: "collectionPda" },
        { isMut: false, isSigner: false, name: "metadata" },
        { isMut: false, isSigner: false, name: "mint" },
        { isMut: true, isSigner: false, name: "collectionAuthorityRecord" },
        { isMut: false, isSigner: false, name: "tokenMetadataProgram" },
      ],
      args: [],
      name: "removeCollection",
    },
    {
      accounts: [
        { isMut: true, isSigner: false, name: "candyMachine" },
        { isMut: true, isSigner: true, name: "bullisticAuthority" },
        { isMut: true, isSigner: false, name: "freezePda" },
      ],
      args: [],
      name: "removeFreeze",
    },
    {
      accounts: [
        { isMut: true, isSigner: false, name: "candyMachine" },
        { isMut: false, isSigner: true, name: "bullisticAuthority" },
        { isMut: true, isSigner: false, name: "creatorAuthority" },
        { isMut: true, isSigner: false, name: "collectionPda" },
        { isMut: false, isSigner: true, name: "payer" },
        { isMut: false, isSigner: false, name: "systemProgram" },
        { isMut: false, isSigner: false, name: "rent" },
        { isMut: true, isSigner: false, name: "metadata" },
        { isMut: false, isSigner: false, name: "mint" },
        { isMut: false, isSigner: false, name: "edition" },
        { isMut: true, isSigner: false, name: "collectionAuthorityRecord" },
        { isMut: false, isSigner: false, name: "tokenMetadataProgram" },
      ],
      args: [],
      name: "setCollection",
    },
    {
      accounts: [
        { isMut: false, isSigner: false, name: "candyMachine" },
        { isMut: false, isSigner: false, name: "metadata" },
        { isMut: false, isSigner: true, name: "buyer" },
        { isMut: true, isSigner: false, name: "collectionPda" },
        { isMut: false, isSigner: false, name: "tokenMetadataProgram" },
        { isMut: false, isSigner: false, name: "instructionSysvarAccount" },
        { isMut: false, isSigner: false, name: "collectionMint" },
        { isMut: false, isSigner: false, name: "collectionMetadata" },
        { isMut: false, isSigner: false, name: "collectionMasterEdition" },
        { isMut: false, isSigner: false, name: "creatorAuthority" },
        { isMut: false, isSigner: false, name: "collectionAuthorityRecord" },
      ],
      args: [],
      name: "setCollectionDuringMint",
    },
    {
      accounts: [
        { isMut: true, isSigner: false, name: "candyMachine" },
        { isMut: true, isSigner: true, name: "bullisticAuthority" },
        { isMut: true, isSigner: false, name: "freezePda" },
        { isMut: false, isSigner: false, name: "systemProgram" },
      ],
      args: [{ name: "freezeTime", type: "i64" }],
      name: "setFreeze",
    },
    {
      accounts: [
        { isMut: true, isSigner: false, name: "freezePda" },
        { isMut: true, isSigner: false, name: "candyMachine" },
        { isMut: true, isSigner: false, name: "tokenAccount" },
        { isMut: false, isSigner: false, name: "owner" },
        { isMut: false, isSigner: false, name: "mint" },
        { isMut: false, isSigner: false, name: "edition" },
        { isMut: true, isSigner: true, name: "payer" },
        { isMut: false, isSigner: false, name: "tokenProgram" },
        { isMut: false, isSigner: false, name: "tokenMetadataProgram" },
        { isMut: false, isSigner: false, name: "systemProgram" },
      ],
      args: [],
      name: "thawNft",
    },
    {
      accounts: [
        { isMut: true, isSigner: false, name: "candyMachine" },
        { isMut: true, isSigner: true, name: "bullisticAuthority" },
        { isMut: true, isSigner: false, name: "freezePda" },
        { isMut: false, isSigner: false, name: "systemProgram" },
      ],
      args: [],
      name: "unlockFunds",
    },
    {
      accounts: [
        { isMut: true, isSigner: false, name: "candyMachine" },
        { isMut: false, isSigner: true, name: "bullisticAuthority" },
        { isMut: false, isSigner: false, name: "treasuryWallet" },
      ],
      args: [{ name: "newAuthority", type: { option: "publicKey" } }],
      name: "updateAuthority",
    },
    {
      accounts: [
        { isMut: true, isSigner: false, name: "candyMachine" },
        { isMut: false, isSigner: true, name: "bullisticAuthority" },
        { isMut: false, isSigner: false, name: "treasuryWallet" },
      ],
      args: [{ name: "data", type: { defined: "CandyMachineData" } }],
      name: "updateCandyMachine",
    },
    {
      accounts: [
        { isMut: true, isSigner: false, name: "candyMachine" },
        { isMut: true, isSigner: true, name: "bullisticAuthority" },
      ],
      args: [],
      name: "withdrawFunds",
    },
  ],
  instructionsMap: {
    addConfigLines: ["candyMachine", "bullisticAuthority"],
    appendMerkleAllowlistRoots: ["bullisticAuthority", "candyMachine"],
    clearMerkleAllowlistRoots: ["bullisticAuthority", "candyMachine"],
    initializeCandyMachine: [
      "candyMachine",
      "treasuryWallet",
      "bullisticAuthority",
      "creatorAuthority",
      "payer",
      "systemProgram",
      "rent",
    ],
    mintNft: [
      "candyMachine",
      "candyMachineCreator",
      "buyer",
      "treasuryWallet",
      "metadata",
      "mint",
      "creatorAuthority",
      "masterEdition",
      "tokenMetadataProgram",
      "tokenProgram",
      "systemProgram",
      "rent",
      "recentSlothashes",
      "instructionSysvarAccount",
      "botSignerAuthority",
      "buyerInfoAccount",
      "buyerTokenAccount",
      "ataProgram",
    ],
    removeCollection: [
      "candyMachine",
      "bullisticAuthority",
      "collectionPda",
      "metadata",
      "mint",
      "collectionAuthorityRecord",
      "tokenMetadataProgram",
    ],
    removeFreeze: ["candyMachine", "bullisticAuthority", "freezePda"],
    setCollection: [
      "candyMachine",
      "bullisticAuthority",
      "creatorAuthority",
      "collectionPda",
      "payer",
      "systemProgram",
      "rent",
      "metadata",
      "mint",
      "edition",
      "collectionAuthorityRecord",
      "tokenMetadataProgram",
    ],
    setCollectionDuringMint: [
      "candyMachine",
      "metadata",
      "buyer",
      "collectionPda",
      "tokenMetadataProgram",
      "instructionSysvarAccount",
      "collectionMint",
      "collectionMetadata",
      "collectionMasterEdition",
      "creatorAuthority",
      "collectionAuthorityRecord",
    ],
    setFreeze: [
      "candyMachine",
      "bullisticAuthority",
      "freezePda",
      "systemProgram",
    ],
    thawNft: [
      "freezePda",
      "candyMachine",
      "tokenAccount",
      "owner",
      "mint",
      "edition",
      "payer",
      "tokenProgram",
      "tokenMetadataProgram",
      "systemProgram",
    ],
    unlockFunds: [
      "candyMachine",
      "bullisticAuthority",
      "freezePda",
      "systemProgram",
    ],
    updateAuthority: ["candyMachine", "bullisticAuthority", "treasuryWallet"],
    updateCandyMachine: ["candyMachine", "bullisticAuthority", "treasuryWallet"],
    withdrawFunds: ["candyMachine", "bullisticAuthority"],
  },
  name: "bullistic_candy_machine",
  types: [
    {
      docs: ["Candy machine settings data."],
      name: "CandyMachineData",
      type: {
        fields: [
          { name: "uuid", type: "string" },
          { name: "price", type: "u64" },
          { name: "premintPrice", type: { option: "u64" } },
          { name: "allowlistPrice", type: { option: "u64" } },
          {
            docs: ["The symbol for the asset"],
            name: "symbol",
            type: "string",
          },
          {
            docs: [
              "Royalty basis points that goes to creators in secondary sales (0-10000)",
            ],
            name: "sellerFeeBasisPoints",
            type: "u16",
          },
          { name: "maxSupply", type: "u64" },
          { name: "itemsAvailable", type: "u64" },
          { name: "isMutable", type: "bool" },
          { name: "allowlistSaleStartTime", type: { option: "i64" } },
          { name: "publicSaleStartTime", type: "i64" },
          { name: "publicSaleEndTime", type: "i64" },
          { name: "creators", type: { vec: { defined: "Creator" } } },
          { name: "omniMintWallets", type: { vec: "publicKey" } },
          {
            name: "hiddenSettings",
            type: { option: { defined: "HiddenSettings" } },
          },
          { name: "botProtectionEnabled", type: "bool" },
          { name: "limitPerAddress", type: "u16" },
          { name: "sequentialMintOrderEnabled", type: "bool" },
          {
            name: "merkleAllowlistRootList",
            type: { vec: { array: ["u8", 32] } },
          },
          {
            name: "splTokenAllowlistSettings",
            type: { option: { defined: "SplTokenAllowlistSettings" } },
          },
        ],
        kind: "struct",
      },
    },
    {
      docs: ["Individual config line for storing NFT data pre-mint."],
      name: "ConfigLine",
      type: {
        fields: [
          { name: "name", type: "string" },
          {
            docs: ["URI pointing to JSON representing the asset"],
            name: "uri",
            type: "string",
          },
        ],
        kind: "struct",
      },
    },
    {
      name: "Creator",
      type: {
        fields: [
          { name: "address", type: "publicKey" },
          { name: "verified", type: "bool" },
          { name: "share", type: "u8" },
        ],
        kind: "struct",
      },
    },
    {
      docs: ["Hidden Settings for large mints used with offline data."],
      name: "HiddenSettings",
      type: {
        fields: [
          { name: "name", type: "string" },
          { name: "uri", type: "string" },
          { name: "hash", type: { array: ["u8", 32] } },
        ],
        kind: "struct",
      },
    },
    {
      name: "BuyerMerkleAllowlistProofData",
      type: {
        fields: [
          { name: "amount", type: "u16" },
          { name: "proof", type: { vec: { array: ["u8", 32] } } },
          { name: "rootIndexForProof", type: "u16" },
        ],
        kind: "struct",
      },
    },
    {
      name: "SplTokenAllowlistSettings",
      type: {
        fields: [
          { name: "mode", type: { defined: "SplTokenAllowlistMode" } },
          { name: "mint", type: "publicKey" },
        ],
        kind: "struct",
      },
    },
    {
      name: "MintPhase",
      type: {
        kind: "enum",
        variants: [
          { name: "Premint" },
          { name: "Allowlist" },
          { name: "Public" },
          { name: "Expired" },
        ],
      },
    },
    {
      name: "SplTokenAllowlistMode",
      type: {
        kind: "enum",
        variants: [{ name: "BurnEveryTime" }, { name: "NeverBurn" }],
      },
    },
  ],
  version: "0.0.1",
};
