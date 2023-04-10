# Candy Machine Creation Flow

## Candy Machine Initial Setup

To create a new candy machine, follow these steps:

1. First include all of the assets for the deployment in the `formfn-candy-machine/create` folder. The current folder, `create-example`, serves as an example of what you need. In particular, you need:

- A `create/assets` folder which contains all of the candy machine collection NFT images and associated metadata files, in addition to a `collection.json` and `collection.png` file. For testing, you can just copy `create-example/assets`.
- A `create/config.json` file which includes config for the candy machine. For testing, you can just copy `create-example/config.json` (and modify it if you want to change the config at all).
- A `create/keypair.json` local wallet keypair (which has been funded with SOL) which will sign the deploy transactions. This deployer keypair does not have to be the same as the `creatorAuthority` of the candy machine. For testing, you can just copy `create-example/keypair.json`.

On the `creatorAuthority`:

The candy machine account includes a `creatorAuthority` field. Currently, this wallet must be the same as the wallet used to create the collection (which occurs during the candy machine setup in `SetCollection`) because it is later used in the validation in `SetCollectionDuringMint`. It's possible this [Metaplex instruction](https://github.com/metaplex-foundation/metaplex-program-library/blob/e196820659b72a0c7ed6c61bfd31be5a699f2d0c/token-metadata/program/src/processor/collection/approve_collection_authority.rs#L18) may change in the future, but currently this update authority can only be set by providing it as a signer. And because we setup the candy machine on behalf of the actual creator(s), this wallet must therefore be one we manage and not the creator's wallet. So the short summary is:

- The `creatorAuthority` field is actually a FF managed wallet.
- There is an additional field, `creatorAuthorityOverride` which represents the "actual" creator wallet address, which will be used to define the Series and minted NFT creators in the FF database.

## Allowlist Setup

If you want to generate an allowlist with specific addresses, follow these steps:

### SPL Token Allowlist

1. First, define your SPL token allowlist config in the `allowlist-config/input/spl-allowlist.json` file. Add all of the addresses you want allowlisted like this:

```json
[
  "4vBpz8WyMuZX6qucrJp8RfhMKivsNiWmq1cp3NRSXFAP",
  "7B24ixFVAsgciqvfyBEnUvDsZPB9Q4UKyNj795rF7XTU"
]
```

2. Once the config file is ready, run the generation script:

```sh
$ yarn generate-spl-allowlist-config --environment=devnet|testnet|mainnet
```

This should create an SPL token, airdrop it to all of the allowlisted addresses, and then write the SPL token allowlist settings to your local `create/config.json` file which will be used next in creating the candy machine.

### Merkle Allowlist

1. First, define your merkle allowlist config in the `allowlist-config/input/merkle-allowlist.json` file. Add all of the address you want allowlisted along with the amounts they are allowlisted for, like this:

```json
[
  {
    "address": "7B24ixFVAsgciqvfyBEnUvDsZPB9Q4UKyNj795rF7XTU",
    "amount": 5
  },
  {
    "address": "CS11P12u5dkyi7L4S41SRa44vgTpFMdQAgHvi1LFrXUu",
    "amount": 4
  }
]
```

The amount is optional and if absent will be defaulted to `1`. If present, it must be a positive integer. Addresses should not be listed more than once in the input list.

2. Once the config file is ready, run the generation script:

```sh
$ yarn generate-merkle-allowlist-config --environment=devnet|testnet|mainnet --useAddressInput=true
```

## Candy Machine Creation

If you generated an allowlist, the data should now be in the `allowlist-config` folder. Check this file to be sure it looks correct.

Finally, you need to create the onchain candy machine.

1. [Optional] If you need to configure a Hydra wallet for the candy machine treasury, do this first:

- Setup a Hydra wallet by [following these steps](https://github.com/formfunction-hq/formfn-hydra#creating-a-hydra-wallet).
- Provide the created holding account wallet address (or ata) as the candy machine treasury wallet in the `config.json` file (created below) as the `solTreasuryAccount` or `splTokenAccount`. This holding account address should be printed out clearly in the Hydra wallet setup commands.

2. [Optional] Change the public sale and allowlist start times

This is useful if you are creating a Candy Machine for testing purposes, and want to test all 3 phases (premint, allowlist, public).

```bash
$ yarn update-create-config-times --allowlistMinutesAhead 5 --publicSaleMinutesAhead 10
```

3. Run the create command:

If you are using a merkle allowlist, run:

```bash
$ yarn create-candy-machine --network devnet|mainnet --allowlist-type merkle
```

Note: Be sure `create/config.json` has no SPL token allowlist settings.

If you are using an SPL allowlist, run:

```bash
# SPL allowlist is the default
$ yarn create-candy-machine --network devnet|mainnet
```

If you are not including an allowlist, just run `yarn create-candy-machine --network devnet|mainnet` but be sure `config.json` has no SPL token allowlist settings.

4. Once the script completes, you can check the output with this command, passing in the address of your candy machine:

```bash
$ yarn print-candy-machine --candyMachine <2HQf...aBjW>
```

5. Once a candy machine has been created, there is a script provided to upload it to our backed which you can run like this:

```bash
# includeMerkleAllowlist is optional and if false or not provided no merkle allowlist data will be uploaded.
yarn upload-candy-machine --candyMachine <2HQf...aBjW> --network local|devnet|testnet|mainnet --includeMerkleAllowlist true|false --randomizeSeriesSlug true|false
```

When testing, it is recommended that you set `--randomizeSeriesSlug` to `true`. When deploying a Candy Machine to production, it is recommended to set it to `false`.

## Upload Failures

If the candy machine creation script fails for any reason (larger mints can take a while to process) you should be able to re-run the same command and the CLI will pick up from where it left off.

## Closing a Candy Machine

If you'd like to close a Candy Machine in order to reclaim rent, you can run something like this:

```
CURRENT_DIR=PATH_TO_DIR yarn sugar withdraw --candy-machine CANDY_MACHINE_ID -k PATH_TO_KEYPAIR_THAT_CREATED_CANDY_MACHINE -r RPC_URL
```

This will close the Candy Machine account, and send the rent SOL back to the keypair you pass.

Make sure `PATH_TO_DIR/.sugar-cli-run` exists—a `sugar.log` file will be stored there. `PATH_TO_DIR` can be anywhere you're comfortable storing the logs, it doesn't affect sugar's execution.
