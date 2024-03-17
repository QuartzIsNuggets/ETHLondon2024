// @ts-nocheck
// src/lib/wallet.js
import {ethers} from 'ethers';

async function connect_wallet() {
    if (typeof window.ethereum !== 'undefined') {
        try {
            await window.ethereum.request({ method: 'eth_requestAccounts' });
            const provider = new ethers.providers.Web3Provider(window.ethereum);
            const signer = provider.getSigner();
            console.log("Wallet connected");
            return signer;
        } catch (error) {
            console.error("User denied account access");
        }
    } else {
        console.log('Please install MetaMask!');
    }
}

const contract_abi = [
    "function newContract(uint256 contract_id, address filler_addr, uint256 filler_deposit, uint8[2][] checkpoints) external payable",
    "function activateContract(uint256 contract_id) external payable",
    "function uploadPosition(uint256 contract_id, uint8[2] position) external",
    "function checkState(uint256 contract_id) external view returns (uint8)",
    "function arbitrate(uint256 contract_id, uint8 decision) external"
];
const contract_address = "YOUR_CONTRACT_ADDRESS";

let client_contract;
let filler_contract;


async function create_new_contract(contract_id, filler_address, filler_deposit, checkpoints) {
    const signer = await connect_wallet();
    if (!signer) {
        console.error("Wallet not connected");
        return;
    }

    // Create a contract instance
    client_contract = new ethers.Contract(contract_address, contract_abi, signer);

    try {
        const options = {
            value: ethers.utils.parseEther(filler_deposit.toString()) // Example: sending 1 Ether
        };
        // Call the `new_contract` function
        const tx = await client_contract.newContract(contract_id, filler_address, filler_deposit, checkpoints, options);
        await tx.wait(); // Wait for the transaction to be mined
        console.log("Contract creation transaction successful", tx);
    } catch (error) {
        console.error("Failed to create new contract", error);
    }
}

async function activate_contract(contract_id, collateral_deposit) {
    const signer = await connect_wallet();
    if (!signer) {
        console.error("Wallet not connected");
        return;
    }

    // Create a contract instance
    filler_contract = new ethers.Contract(contract_address, contract_abi, signer);

    try {
        // Convert the collateral deposit to the correct unit (wei, for example)
        // ethers.utils.parseEther can convert ether to wei if collateral_deposit is in ether
        const options = {
            value: ethers.utils.parseEther(collateral_deposit.toString()) // Assuming collateral_deposit is a string representing the ether amount
        };

        // Call the activateContract function with contract_id and send collateral_deposit as value
        const tx = await filler_contract.activateContract(contract_id, options);
        await tx.wait(); // Wait for the transaction to be mined
        console.log("activateContract transaction successful", tx);
    } catch (error) {
        console.error("Failed to activate contract", error);
    }
}

async function upload_position(contract_id, position) {
    try {
        const tx = await filler_contract.uploadPosition(contract_id, position);
        await tx.wait();
        console.log("activateContract transaction successful", tx);
    } catch (error) {
        console.error("Failed to activate contract", error);
    }
}

async function check_state(contract_id) {
    try {
        const result = await client_contract.checkState(contract_id);
        console.log("activateContract transaction successful", tx);
        return result;
    } catch (error) {
        console.error("Failed to activate contract", error);
        return 0;
    }
}

async function arbitrate(contract_id, decision){
    try {
        const tx = await client_contract.arbitrate(contract_id, decision);
        await tx.wait();
        console.log("activateContract transaction successful", tx);
    } catch (error) {
        console.error("Failed to activate contract", error);
    }
}


export { create_new_contract, activate_contract, upload_position, check_state, arbitrate  };
