#!/bin/bash

BYTECODE=../output-docker/jex-sc-governance/jex-sc-governance.wasm
PROXY=https://devnet-gateway.multiversx.com
SC_ADDRESS=$(mxpy data load --key=address-devnet)
CHAIN=D
SCRIPT_DIR=$(dirname $0)
SC_LOCKER_ADDRESS=erd1qqqqqqqqqqqqqpgqmmxzmktd09gq0hldtczerlv444ykt3pz6avsnys6m9
NFT_COLLECTION_ID=JIP-c2a017

source "${SCRIPT_DIR}/_common.snippets.sh"

# Reproducible build using:
# mxpy contract reproducible-build --docker-image="multiversx/sdk-rust-contract-builder:v5.3.0"
deploy() {
    echo 'You are about to deploy SC on devnet (Ctrl-C to abort)'
    read answer

    mxpy contract deploy --bytecode=${BYTECODE} \
        --keyfile=${1} --gas-limit=50000000 --outfile="deploy-devnet.interaction.json" \
        --arguments "${SC_LOCKER_ADDRESS}" "str:${NFT_COLLECTION_ID}" \
        --proxy=${PROXY} --chain=${CHAIN} --recall-nonce --send || return
    
    SC_ADDRESS=$(cat deploy-devnet.interaction.json | jq -r .contractAddress)

    mxpy data store --key=address-devnet --value=${SC_ADDRESS}

    echo ""
    echo "Smart contract address: ${SC_ADDRESS}"
}

upgrade() {
    echo 'You are about to upgrade current SC on devnet (Ctrl-C to abort)'
    read answer

    mxpy contract upgrade --bytecode=${BYTECODE} \
        --keyfile=${1} --gas-limit=50000000 --outfile="deploy-devnet.interaction.json" \
        --proxy=${PROXY} --chain=${CHAIN} --recall-nonce --send ${SC_ADDRESS} || return

    echo ""
    echo "Smart contract upgraded: ${SC_ADDRESS}"
}

CMD=$1
shift

$CMD $*
