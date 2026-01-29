##
# Info
##

echo "Proxy: ${PROXY}"
echo "SC address: ${SC_ADDRESS:-Not deployed}"

if [ "${SC_ADDRESS}" != "" ]
then
    SC_ADDRESS_HEX="$(mxpy wallet bech32 --decode "${SC_ADDRESS}")"
fi
NFT_COLLECTION_ID_HEX="$(echo -n "${NFT_COLLECTION_ID}" | xxd -ps)"

##
# Post deployment
##

setNftCreateRole() {
    ROLE="$(echo -n "ESDTRoleNFTCreate" | xxd -ps)"
    SET_ROLE_DATA="setSpecialRole@${NFT_COLLECTION_ID_HEX}@${SC_ADDRESS_HEX}@${ROLE}"
    SET_ROLE_ADDRESS=erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u

    echo 'Collection owner must send the following tx:'
    echo "receiver: ${SET_ROLE_ADDRESS}"
    echo 'value: 0 EGLD'
    echo 'gas limit: 51000000'
    echo 'data:'
    echo "${SET_ROLE_DATA}"
}

transferNftCreateRole() {
    read -p "From address: " FROM_ADDRESS
    FROM_ADDRESS_HEX="$(mxpy wallet bech32 --decode "${FROM_ADDRESS}")"

    ROLE="$(echo -n "ESDTRoleNFTCreate" | xxd -ps)"
    SET_ROLE_DATA="transferNFTCreateRole@${NFT_COLLECTION_ID_HEX}@${FROM_ADDRESS_HEX}@${SC_ADDRESS_HEX}"
    SET_ROLE_ADDRESS=erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u

    echo 'Collection owner must send the following tx:'
    echo "receiver: ${SET_ROLE_ADDRESS}"
    echo 'value: 0 EGLD'
    echo 'gas limit: 51000000'
    echo 'data:'
    echo "${SET_ROLE_DATA}"
}

setAdmin() {
    read -p "Admin address: " ADMIN_ADDRESS

    mxpy contract call ${SC_ADDRESS} \
        --function="setAdmin" \
        --arguments "${ADMIN_ADDRESS}" \
        --gas-limit=10000000 \
        --keyfile=${1} --proxy=${PROXY} --chain=${CHAIN} --send || return
}

##
# Admin
##

createProposal() {
    read -p "ID (JIP number): " PROP_ID
    read -p "Label: " LABEL
    read -p "Start vote timestamp (s): " START_VOTE_TIMESTAMP
    read -p "End vote timestamp (s): " END_VOTE_TIMESTAMP
    read -p "Nb choices: " NB_CHOICES
    read -p "Proposal content tx hash: " CONTENT_TX_HASH

    mxpy contract call ${SC_ADDRESS} \
        --function="createProposal" \
        --arguments "${PROP_ID}" "str:${LABEL}" \
            "${START_VOTE_TIMESTAMP}" "${END_VOTE_TIMESTAMP}" \
            "${NB_CHOICES}" "0x${CONTENT_TX_HASH}" \
        --gas-limit=10000000 \
        --keyfile=${1} --proxy=${PROXY} --chain=${CHAIN} --send || return
}

##
# Views
##

isVoter() {
    read -p "Address: " ADDRESS
    
    mxpy contract query ${SC_ADDRESS} \
        --function "isVoter" \
        --arguments "${ADDRESS}" \
        --proxy=${PROXY}
}

getAdminAddress() {
    mxpy contract query ${SC_ADDRESS} --function "getAdminAddress" --proxy=${PROXY}
}

getProposals() {
    FIRST=${1:-1}
    LAST=${2:-10}

    mxpy contract query ${SC_ADDRESS} \
        --function "getProposals" --arguments "${FIRST}" "${LAST}" \
        --proxy=${PROXY}
}

getProposalNftCollectionId() {
    mxpy contract query ${SC_ADDRESS} --function "getProposalNftCollectionId" --proxy=${PROXY}
}

getVoters() {
    read -p "Proposal ID: " PROPOSAL_ID

    mxpy contract query ${SC_ADDRESS} --function "getVoters" --arguments "${PROPOSAL_ID}" --proxy=${PROXY} \
        | jq -r .[] | while read a
            do
                mxpy wallet bech32 --encode "${a}"
            done
    echo ""
}

getVotingPower() {
    read -p "Address: " ADDRESS
    
    mxpy contract query ${SC_ADDRESS} \
        --function "getVotingPower" \
        --arguments "${ADDRESS}" \
        --proxy=${PROXY}
}
