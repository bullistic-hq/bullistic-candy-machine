#!/bin/bash

ALLOWLIST_TYPE=$1

if [ "$ALLOWLIST_TYPE" = "spl" ]; then
  echo "Starting test using SPL token allowlist."

  solana airdrop 10 $(solana address) -u localhost

  echo -e "\n🚧 Generating candy SPL token allowlist...\n"

  yarn generate-spl-allowlist-config

  echo -e "\n🚧 Deploying candy machine and allowlist using formfn-sugar CLI..."

  yarn test-cli use-spl-allowlist || {
    echo -e "\nAborting test run, yarn test-cli commmand failed."
    exit 1
  }

  echo -e "\n🚧 Running SDK tests...\n"

  DEBUG=true yarn jest mintNftUsingSplTokenAllowlist.test.ts

  echo -e "\n🚧 CLI local test complete using SPL token allowlist!\n"
  exit 0
fi

if [ "$ALLOWLIST_TYPE" = "merkle" ]; then
  echo "Starting test using Merkle allowlist."

  solana airdrop 10 $(solana address) -u localhost

  echo -e "\n🚧 Generating candy machine merkle allowlist...\n"

  yarn generate-merkle-allowlist-config

  echo -e "\n🚧 Deploying candy machine and allowlist using formfn-sugar CLI..."

  yarn test-cli use-merkle-allowlist || {
    echo -e "\nAborting test run, yarn test-cli commmand failed."
    exit 1
  }

  echo -e "\n🚧 Running SDK tests...\n"

  DEBUG=true yarn jest mintNftUsingMerkleAllowlist.test.ts

  echo -e "\n🚧 CLI local test complete using merkle allowlist!\n"
  exit 0
fi

echo -e "Unrecognized command, please pass 'spl' or 'merkle' as the second argument to set the allowlist type for the test run."
exit 1