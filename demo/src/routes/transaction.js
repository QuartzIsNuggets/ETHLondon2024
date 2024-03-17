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

const contractABI = ["..."];
const contractAddress = "YOUR_CONTRACT_ADDRESS";

async function create_new_contract(contractId, fillerAddress, fillerDeposit, checkpoints) {
    const signer = await connect_wallet();
    if (!signer) {
        console.error("Wallet not connected");
        return;
    }

    // Create a contract instance
    const contract = new ethers.Contract(contractAddress, contractABI, signer);

    try {
        // Call the `new_contract` function
        const tx = await contract.new_contract(contractId, fillerAddress, fillerDeposit, checkpoints);
        await tx.wait(); // Wait for the transaction to be mined
        console.log("Contract creation transaction successful", tx);
    } catch (error) {
        console.error("Failed to create new contract", error);
    }
}

export { create_new_contract };
