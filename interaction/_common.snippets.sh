##
# Info
##

echo "Proxy: ${PROXY}"
echo "SC address: ${SC_ADDRESS:-Not deployed}"


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

getVotingPower() {
    read -p "Address: " ADDRESS
    
    mxpy contract query ${SC_ADDRESS} \
        --function "getVotingPower" \
        --arguments "${ADDRESS}" \
        --proxy=${PROXY}
}