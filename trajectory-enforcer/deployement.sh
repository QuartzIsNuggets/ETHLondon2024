# Prepare transactions data
cargo stylus deploy -e "$RPC_URL" --private-key "$PRIVATE_KEY" --dry-run --output-tx-data-to-dir .

# Get contract bytecode
bytecode=$(cat ./deployment_tx_data | od -An -v -tx1 | tr -d ' \n')
rm ./deployment_tx_data

# Send transaction to blockchain
echo "Sending contract creation transaction..."
cast send --rpc-url "$RPC_URL" --private-key "$PRIVATE_KEY" --create $bytecode > $DEPLOY_CONTRACT_RESULT_FILE

# Get contract address
contract_address_str=$(cat "$DEPLOY_CONTRACT_RESULT_FILE" | sed -n 4p)
contract_address_array=($contract_address_str)
contract_address=${contract_address_array[1]}
rm "$DEPLOY_CONTRACT_RESULT_FILE"

# Send activation transaction
echo "Sending activation transaction..."
if [ -f ./activation_tx_data ]; then
    cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY 0x0000000000000000000000000000000000000071 "activateProgram(address)" $contract_address > /dev/null
    rm ./activation_tx_data
else
    echo "Not needed, contract already activated"
fi

# Final result
echo "Contract deployed and activated at address: $contract_address"
