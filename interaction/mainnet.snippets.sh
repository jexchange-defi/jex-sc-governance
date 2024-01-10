#!/bin/bash

set -eu

BYTECODE=../output-docker/jex-sc-governance/jex-sc-governance.wasm
PROXY=https://gateway.multiversx.com
SC_ADDRESS=$(mxpy data load --key=address-mainnet)
CHAIN=1
SCRIPT_DIR=$(dirname $0)
SC_LOCKER_ADDRESS=erd1qqqqqqqqqqqqqpgq05whpg29ggrrm9ww3ufsf9ud23f66msv6avs5s5xxy
NFT_COLLECTION_ID=JIP-df3784

source "${SCRIPT_DIR}/_common.snippets.sh"

# Reproducible build using:
# mxpy contract reproducible-build --docker-image="multiversx/sdk-rust-contract-builder:v5.3.0"
deploy() {
    echo 'You are about to deploy SC on mainnet (Ctrl-C to abort)'
    read answer

    mxpy contract deploy --bytecode=${BYTECODE} \
        --keyfile=${1} --gas-limit=50000000 --outfile="deploy-mainnet.interaction.json" \
        --arguments "${SC_LOCKER_ADDRESS}" "str:${NFT_COLLECTION_ID}" \
        --proxy=${PROXY} --chain=${CHAIN} --recall-nonce --send || return
    
    SC_ADDRESS=$(cat deploy-mainnet.interaction.json | jq -r .contractAddress)

    mxpy data store --key=address-mainnet --value=${SC_ADDRESS}

    echo ""
    echo "Smart contract address: ${SC_ADDRESS}"
}

upgrade() {
    echo 'You are about to upgrade current SC on mainnet (Ctrl-C to abort)'
    read answer

    mxpy contract upgrade --bytecode=${BYTECODE} \
        --keyfile=${1} --gas-limit=50000000 --outfile="deploy-mainnet.interaction.json" \
        --proxy=${PROXY} --chain=${CHAIN} --recall-nonce --send ${SC_ADDRESS} || return

    echo ""
    echo "Smart contract upgraded: ${SC_ADDRESS}"
}

CMD=$1
shift

$CMD $*
