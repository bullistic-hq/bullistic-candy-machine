![](banner.jpeg)

<div align="center">
  <h1>Bullistic Candy Machine</h1>
  <a href="#overview">Overview</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#repo-structure">Repo Structure</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#initial-environment-setup">Initial Environment Setup</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#getting-started">Getting Started</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#sugar-cli">Sugar CLI</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#merkle-allowlist">Merkle Allowlist</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#testing">Testing</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#candy-machine-creation-flow">Candy Machine Creation Flow</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#fullstack-testing">Fullstack Testing</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#releases">Releases</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#tips">Tips</a>
  <br />
  <hr />
</div>


## Overview

Bullistic fork of [Metaplex Candy Machine v2](https://github.com/metaplex-foundation/metaplex-program-library/tree/master/candy-machine). This includes our modified program code and client SDK. The repo also includes a fork of the [Metaplex Sugar CLI](https://github.com/metaplex-foundation/sugar) which is used for creating and interacting with candy machines. The main change to the program is support for Merkle allowlists.

- Mainnet address: `gachaC2NGh63y4ogLK8xHLeB5ZFZ8ypDLXEQyKNm8sy`
- Devnet address (same as mainnet): `gachaC2NGh63y4ogLK8xHLeB5ZFZ8ypDLXEQyKNm8sy`

## Repo Structure

```
├── artifacts                # 3rd party program binaries (from solana program dump command)
├── create-example           # Folder containing example assets and config for a candy machine
├── keys                     # Program keypairs for devnet and testnet deployments
├── programs                 # Rust program source code
│   ├── bullistic-candy-machine # Candy machine program
│   ├── bullistic-sugar         # Sugar CLI program
├── scripts                  # Some helper bash scripts for the repo
├── src                      # TypeScript source folder
│   ├── ...                  # Other SDK folders
│   ├── constants            # SDK constants folder
│   ├── idl                  # Anchor-generated program IDL and types
│   ├── scripts              # SDK specific scripts (e.g. for generating the merkle allowlist)
│   ├── sdk                  # TypeScript SDK
│   ├── tests                # SDK tests
│   └── index.ts             # SDK Exports
├── ...                      # Other misc. project config files and folders
└── README.md
```

## Initial Environment Setup

Complete the following to setup your environment:

1. Install [Node.js](https://nodejs.org/en) (and [nvm](https://github.com/nvm-sh/nvm) if you want).
2. Follow the [Anchor setup instructions](https://book.anchor-lang.com/getting_started/installation.html). After this you should have Rust, Solana, Yarn and Anchor installed on your system.

## Getting Started

Once you have your environment setup you can run the following:

```sh
# Install dependencies
$ yarn

# Run setup steps
$ yarn setup

# Create template for the create-example folder.
# This isn't in the setup script because it will overwrite anything which is already in the folder.
$ cp -r create-example create
```

The following commands are also available:

```sh
# Run prettier checks
$ yarn prettier

# Run eslint checks
$ yarn eslint

# Run prettier and eslint with auto-fix flag
$ yarn lint

# Compile the program
$ yarn build-program

# Compile TypeScript code
$ yarn tsc

# Build the TS SDK
$ yarn build-sdk

# Build everything
$ yarn build

# Run all tests (see more on testing below)
$ yarn test
```

## Sugar CLI

There is a forked version of the Metaplex Sugar CLI in `programs/bullistic-sugar`. This is being used to handle uploading assets and candy machine initialization. It imports the `bullistic-candy-machine` program.

- Original repository: https://github.com/metaplex-foundation/sugar
- Documentation: https://docs.metaplex.com/sugar/introduction

To run commands with the CLI, you can `cd programs/bullistic-sugar` and run normal cargo commands, or run them from the top level like this:

```sh
# Runs cargo run -- --version, which invokes the Sugar CLI with the --version argument
$ yarn sugar -- -- --version
# Prints ~ sugar-cli 0.6.3

# Run normal CLI commands:
$ yarn sugar create-config
```

You will more likely use the Sugar CLI via helper bash scripts rather than running CLI commands directly (see more details below).

## Merkle Allowlist

The candy machine program supports a merkle tree address allowlist. For testing and production purposes, we can generate and manage allowlist data locally. The generated allowlist data is saved in the `gitignore`'d root level `merkle-allowlist` folder.

To generate the files in this folder, you can run the generation script:

```sh
# Run with defaults.
$ yarn generate-merkle-allowlist-config

# Run with arguments.
$ yarn generate-merkle-allowlist-config --allowlistSize=50 --leafCount=10
```

The size of the allowlist data will be determined by the settings in the `scripts/constants.ts` file. These can be adjusted as needed for testing/development purposes.

## Testing

There are several different tests included in the repo. To explain them and how to run them it's easier to just look at the npm scripts which are available (note: keep this in sync with `package.json`):

```sh
# Run the programs/bullistic-candy-machine/tests Rust tests using Cargo.
# These test the program directly using the solana_program_test crate.
$ yarn test-program
$ yarn test-program-debug

# Run unit tests for TS SDK utils and helper functions.
$ yarn test-unit

# Run the full sugar CLI test script.
$ yarn test-cli

# Run the sugar CLI test in local-only mode with the SDK tests.
# This spins up a validator using Anchor, generates allowlist(s),
# deploys a candy machine using the CLI and then runs the TS SDK tests.
$ yarn test-local-spl # Using SPL allowlist
$ yarn test-local-merkle # Using merkle allowlist
$ yarn test-local # Run both of the above

# Run all the tests in one go.
$ yarn test
```

## Candy Machine Creation Flow

See the `create-example/README.md` document for details on the candy machine creation workflow.

Here is a quick example for testing on localhost:

```sh
# With Merkle allowlist:
$ yarn generate-merkle-allowlist-config --environment=devnet --useAddressInput=true
$ yarn update-create-config-times --allowlistMinutesAhead 3 --publicSaleMinutesAhead 6
$ yarn create-candy-machine --network devnet --allowlist-type merkle
$ yarn upload-candy-machine --network local --includeMerkleAllowlist true --randomizeSeriesSlug true

# With SPL allowlist:
$ yarn generate-spl-allowlist-config --environment=devnet --amount=2
$ yarn update-create-config-times --allowlistMinutesAhead 3 --publicSaleMinutesAhead 6
$ yarn create-candy-machine --network devnet
$ yarn upload-candy-machine --network local --includeMerkleAllowlist false --randomizeSeriesSlug true
```

You can also update an on-chain candy machine. This will perform the update based on the configuration in your `config.json` file. The update command will use the candy machine address stored in the local `candy-machine-pubkey.json` file. To perform the update run:

```sh
$ yarn create-candy-machine --network devnet --update true
# Note: after this you'll need to run the upload-candy-machine script again to update the data in our db.
```

## Fullstack Testing

If you need to test the generative mints feature using with the frontend/backend, you can follow these steps:

1. Change `creatorAuthority` in `create/config.json` to some user you have in your localhost DB.
2. Follow the steps in `create-example/README.md` to create and upload a candy machine.
3. Modify your LD feature flag variation here https://app.launchdarkly.com/default/production/features/campaignsConfig/variations. Change the `creatorId` and `candyMachineId` fields to match the candy machine created in step 2.
4. Use “individual targeting” for LD so that your variation is used for your user on localhost.
5. Go to `http://localhost:3000/@<creator-username>/campaigns/popheadz` to test.

## Releases

Releases are based on git tags. There is a GitHub Action which is responsible for running releases.

The Solana program and TS SDK are versioned separately.

### Solana Program

For the Solana program we build a binary using Anchor. To publish a new binary:

1. Increment the version in the program `Cargo.toml` file.
2. Push a commit to the `main` branch in GitHub.

Note that if the Anchor version is upgraded you should update the anchor version in the GitHub action as well.

### Devnet Deployment

Run the following to deploy or upgrade the program on devnet or testnet:

```bash
# Set your CLI to the appropriate cluster.
$ solana config set -u devnet|testnet

# Give the script executable permissions (needed once on first use only)
$ chmod +x ./scripts/deploy-program.sh

# Get the deployer account address.
$ solana-keygen pubkey keys/devnet/deployer-keypair.json

# Ensure you have enough SOL. Repeat the following until you have ~10 SOL or more.
$ solana airdrop 1 G1K5YZmhg1LqaYUC9VXWK7YLCdwqJcVPLpgBt5tmUWVf

# Test, build and deploy the program. Pass argument for network.
$ yarn deploy-program devnet|testnet
```

To deploy the program from scratch to a new program address, do the following:

- Update the `DEPLOY_PROGRAM_ID` in `deploy-program.sh`.
- Add the new program address in `Anchor.toml` and all of the Anchor configs in `scripts/anchor-configs`.
- Add the new program and config keypairs for devnet/testnet in `keys/`.
- Update the address in `keys/README`.
- Run the above deploy steps.

### Mainnet Deployment

The program is deployed on Solana mainnet at `gachaC2NGh63y4ogLK8xHLeB5ZFZ8ypDLXEQyKNm8sy`.

### TypeScript SDK

Follow the following steps to publish a new version of the TypeScript SDK:

1. Run `yarn version` and enter a new appropriate [semver version](https://docs.npmjs.com/about-semantic-versioning) for the npm package. That will create a new tag and commit.
2. Run `git push origin NEW_TAG`.
3. `git push` the new commit as well.

This will push the new release tag to GitHub and trigger the release pipeline, after which clients can install the latest SDK with `yarn add @bullistic-hq/bullistic-candy-machine@latest`.

## Tips

Feel free to run `cargo clippy` ([Clippy](https://github.com/rust-lang/rust-clippy)) once in a while to see if there are any recommended improvements for the Rust code.
