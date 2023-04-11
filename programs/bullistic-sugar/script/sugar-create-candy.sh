#!/bin/bash
# Sugar CLI create script to create a new candy machine using the assets and
# config in the bullistic-candy-machine/create directory. See usage instructions
# in the create-example/README.

ENVIRONMENT=devnet
RUN_UPDATE_ONLY=false
ALLOWLIST_TYPE="spl"

while test $# -gt 0; do
    case "$1" in
        --network*)
            shift
            ENVIRONMENT=$1
            shift
            ;;
        --update*)
            shift
            RUN_UPDATE_ONLY=$1
            shift
            ;;
        --allowlist-type*)
            shift
            ALLOWLIST_TYPE=$1
            shift
            ;;
        *)
            break
            ;;
  esac
done

if [ ! "$ALLOWLIST_TYPE" = "merkle" ] && [ ! "$ALLOWLIST_TYPE" = "spl" ]; then
    echo -e "Invalid option provided for --allowlist-type flag: only 'spl' or 'merkle' is allowd.\n"
    exit 1
fi

if [ ! "$RUN_UPDATE_ONLY" = true ] && [ ! "$RUN_UPDATE_ONLY" = false ]; then
    echo -e "Invalid option provided for --update flag: only true is allowd.\n"
    exit 1
fi

ENV_URL=""
RPC=""

if [[ "$ENVIRONMENT" == "devnet" ]]
then
    ENV_URL="devnet"
    # RPC="https://api.devnet.solana.com"
    # Alternate option:
    RPC="https://devnet.genesysgo.net"
elif [[ "$ENVIRONMENT" == "mainnet" ]]
then
    ENV_URL="mainnet"
    RPC="https://patient-proud-dew.solana-mainnet.quiknode.pro/5d277ae9935d3f2c1513be336f0d0bcb54f63a07/"
else
  echo -e "\nUnrecognized environment. Only 'devnet' or 'mainnet' is allowed.\n"
  exit 1
fi

# Launch by default. Switch to "n" for manual control over launch flow.
LAUNCH="n"

# Exported for Sugar to use for current directory.
export CURRENT_DIR="$(pwd)/create"
SCRIPT_DIR=$(cd -- $(dirname -- "${BASH_SOURCE[0]}") &>/dev/null && pwd)
PARENT_DIR="$(dirname "$SCRIPT_DIR")"
ASSETS_DIR=$CURRENT_DIR/assets
SUGAR_BIN="cargo run --quiet --bin bullistic_sugar --"
CONFIG_FILE=$CURRENT_DIR/config.json

SUGAR_FILES_DIR=".sugar-cli-run"
mkdir -p $CURRENT_DIR/$SUGAR_FILES_DIR

RESUME_FILE="$CURRENT_DIR/$SUGAR_FILES_DIR/.sugar_resume"
CACHE_DIR=$CURRENT_DIR/$SUGAR_FILES_DIR
SUGAR_LOG=$SUGAR_FILES_DIR/sugar.log

TOTAL_ASSETS=$(cd $ASSETS_DIR && ls | wc -l)
ITEMS=$(((TOTAL_ASSETS - 2) / 2))
LAST_INDEX=$((ITEMS - 1))

# output colours
RED() { echo $'\e[1;31m'$1$'\e[0m'; }
GRN() { echo $'\e[1;32m'$1$'\e[0m'; }
BLU() { echo $'\e[1;34m'$1$'\e[0m'; }
MAG() { echo $'\e[1;35m'$1$'\e[0m'; }
CYN() { echo $'\e[1;36m'$1$'\e[0m'; }

STORAGE="bundlr"
ARWEAVE_JWK="null"
INFURA_ID="null"
INFURA_SECRET="null"
AWS_BUCKET="null"
NFT_STORAGE_TOKEN="null"
SHDW_STORAGE_ACCOUNT="null"

#-----------------------------------------------------------------------------#
# SETTING UP                                                                  #
#-----------------------------------------------------------------------------#

WALLET_KEY=$CURRENT_DIR/keypair.json
WALLET_PUBKEY=$(solana-keygen pubkey $WALLET_KEY)
CACHE_NAME="sugar-test"
CACHE_FILE="$CACHE_DIR/cache-${CACHE_NAME}.json"

TIMESTAMP=`date "+%d/%m/%y %T"`

# removes temporary files
function clean_up {
    rm -rf $CACHE_FILE 2>/dev/null
    rm -rf $SUGAR_LOG 2>/dev/null
    rm -rf $RESUME_FILE 2>/dev/null
}

if [ -f "$RESUME_FILE" ]; then
    echo ""
    echo "Resume file found! Will resume previous run."
    source $RESUME_FILE
    RESUME=1
fi

# Resume checkpoint
cat >$RESUME_FILE <<-EOM
#!/bin/bash
# File timestamp: $TIMESTAMP

ENV_URL="$ENV_URL"
RPC="$RPC"
ITEMS=$ITEMS

STORAGE="$STORAGE"
ARWEAVE_JWK="$ARWEAVE_JWK"
INFURA_ID="$INFURA_ID"
INFURA_SECRET="$INFURA_SECRET"
AWS_BUCKET="$AWS_BUCKET"
AWS_PROFILE="$AWS_PROFILE"
AWS_DIRECTORY="$AWS_DIRECTORY"
NFT_STORAGE_TOKEN="$NFT_STORAGE_TOKEN"
SHDW_STORAGE_ACCOUNT="$SHDW_STORAGE_ACCOUNT"
EOM

#-----------------------------------------------------------------------------#
# AUXILIARY FUNCTIONS                                                         #
#-----------------------------------------------------------------------------#

# run the upload command
function upload {
    $SUGAR_BIN upload -c ${CONFIG_FILE} --keypair $WALLET_KEY --cache $CACHE_FILE -r $RPC $ASSETS_DIR
    EXIT_CODE=$?
    if [ ! $EXIT_CODE -eq 0 ]; then
        MAG "<<<"
        RED "[$(date "+%T")] Aborting: upload failed"
        exit 1
    fi
}

# run the deploy command
function deploy {
    if [ $ALLOWLIST_TYPE = "merkle" ]; then
        $SUGAR_BIN deploy -c ${CONFIG_FILE} --keypair $WALLET_KEY --cache $CACHE_FILE -r $RPC --use-merkle-allowlist
    else
        $SUGAR_BIN deploy -c ${CONFIG_FILE} --keypair $WALLET_KEY --cache $CACHE_FILE -r $RPC
    fi
    EXIT_CODE=$?
    if [ ! $EXIT_CODE -eq 0 ]; then
        MAG "<<<"
        RED "[$(date "+%T")] Aborting: deploy failed"
        exit 1
    fi
}

# run the update command
function update {
    $SUGAR_BIN update -c ${CONFIG_FILE} --keypair $WALLET_KEY -r $RPC
    EXIT_CODE=$?
    if [ ! $EXIT_CODE -eq 0 ]; then
        MAG "<<<"
        RED "[$(date "+%T")] Aborting: update failed"
        exit 1
    fi
}

# run the set-merkle-allowlist command
function setMerkleAllowlist {
    $SUGAR_BIN set-merkle-allowlist -c ${CONFIG_FILE} --keypair $WALLET_KEY --cache $CACHE_FILE -r $RPC
    EXIT_CODE=$?
    if [ ! $EXIT_CODE -eq 0 ]; then
        MAG "<<<"
        RED "[$(date "+%T")] Aborting: setting the merkle allowlist failed"
        exit 1
    fi
}

# run the clear merkle allowlist command
function clearMerkleAllowlist {
    $SUGAR_BIN clear-merkle-allowlist -c ${CONFIG_FILE} --keypair $WALLET_KEY --cache $CACHE_FILE -r $RPC
    EXIT_CODE=$?
    if [ ! $EXIT_CODE -eq 0 ]; then
        MAG "<<<"
        RED "[$(date "+%T")] Aborting: clearing merkle allowlist failed"
        exit 1
    fi
}

# run the verify upload command
function verify {
    $SUGAR_BIN verify --keypair $WALLET_KEY --cache $CACHE_FILE -r $RPC
    EXIT_CODE=$?
    if [ ! $EXIT_CODE -eq 0 ]; then
        MAG "<<<"
        RED "[$(date "+%T")] Aborting: verify failed"
        exit 1
    fi
}

if $RUN_UPDATE_ONLY; then
    echo ""
    CYN "Updating on-chain candy machine..."
    echo ""
    update
    exit 0
fi

#-----------------------------------------------------------------------------#
# COMMAND EXECUTION                                                           #
#-----------------------------------------------------------------------------#

echo ""
echo "-----------------------------------------------------------------------------"
echo "Here we go! Are you ready:"
echo "- Deploying Candy Machine with $ITEMS items"
echo "- Environment: ${ENV_URL}"
echo "- RPC URL: ${RPC}"
echo "- Using '${STORAGE}' storage"
echo "- Current directory '${CURRENT_DIR}'"
echo "- Assets directory '${ASSETS_DIR}'"
echo "- Using keypair '${WALLET_KEY}'"
echo "- For wallet pubkey '${WALLET_PUBKEY}'"
echo "- With config file '${CONFIG_FILE}'"
echo "- Allowlist type: '${ALLOWLIST_TYPE}' (if 'spl', it will use the config.json settings, if they exist.)"
echo "-----------------------------------------------------------------------------"
echo ""

if [ $ALLOWLIST_TYPE = "merkle" ]; then
    echo -e "Note: Be sure create/config.json does not include any SPL token allowlist settings!\n"
fi
read -p "Ready to go? Enter y/Y to continue." -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]
then
    rm -rf $RESUME_FILE 2>/dev/null
    echo "Nevermind, bye!"
    exit 1
fi

echo "Building sugar binary..."
cargo build
echo ""

if [ "$LAUNCH" = "Y" ]; then
    echo ""
    CYN "Executing Sugar launch: steps [1, 2, 3, 4]"
    echo ""
    MAG ">>>"
    if [ "$ALLOWLIST_TYPE" = "merkle" ]; then
        $SUGAR_BIN launch -c ${CONFIG_FILE} --keypair $WALLET_KEY --cache $CACHE_FILE -r $RPC $ASSETS_DIR --skip-collection-prompt --use-merkle-allowlist
    else
        $SUGAR_BIN launch -c ${CONFIG_FILE} --keypair $WALLET_KEY --cache $CACHE_FILE -r $RPC $ASSETS_DIR --skip-collection-prompt
    fi
    EXIT_CODE=$?
    MAG "<<<"
    
    if [ ! $EXIT_CODE -eq 0 ]; then
        RED "[$(date "+%T")] Aborting: launch failed"
        exit 1
    fi
else
    echo ""
    CYN "1. Validating JSON metadata files"
    echo ""
    MAG ">>>"
    $SUGAR_BIN validate $ASSETS_DIR -c ${CONFIG_FILE} --skip-collection-prompt
    EXIT_CODE=$?
    MAG "<<<"

    if [ ! $EXIT_CODE -eq 0 ]; then
        RED "[$(date "+%T")] Aborting: validation failed"
        exit 1
    fi

    echo ""
    CYN "2. Uploading assets"
    echo ""
    MAG ">>>"
    upload
    MAG "<<<"
    echo ""

    echo ""
    CYN "3. Deploying Candy Machine"
    echo ""
    MAG ">>>"
    deploy
    MAG "<<<"
    echo ""

    if [ "$ALLOWLIST_TYPE" = "merkle" ]; then
        echo ""
        CYN "Clearing merkle allowlist"
        echo ""
        MAG ">>>"
        clearMerkleAllowlist
        MAG "<<<"
        echo ""

        echo ""
        CYN "Uploading merkle allowlist"
        echo ""
        MAG ">>>"
        setMerkleAllowlist
        MAG "<<<"
        echo ""
    fi

    echo ""
    CYN "4. Verifying deployment"
    echo ""
    MAG ">>>"
    verify
    MAG "<<<"
fi

clean_up

echo "[$(date "+%T")] Deploy completed!"