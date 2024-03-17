#![cfg_attr(not(feature = "export-abi"), no_std)]
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloc::{vec, vec::Vec};

use alloy_primitives::{uint, U256, U64, U8, U16};
use alloy_sol_types::sol;
use stylus_sdk::alloy_primitives::Address;
use stylus_sdk::call::transfer_eth;
use stylus_sdk::msg;
use stylus_sdk::prelude::*;
use stylus_sdk::stylus_proc::entrypoint;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

const CONTRACT_UNINIT: U8 = uint!(0_U8);
const CONTRACT_CREATED: U8 = uint!(1_U8);
const CONTRACT_ACTIVATED: U8 = uint!(2_U8);
const CONTRACT_LOCKED: U8 = uint!(3_U8);

const ARBITRAGE_FULFILL: u8 = 0;
const ARBITRAGE_REFUND: u8 = 1;

const BLOCK_PER_CHECKPOINT: u64 = 1;

const ARBITRATOR_ADDRESS: Address = Address::new([
    0xCe, 0xCD, 0x23, 0x95, 0x62, 0x53, 0x83, 0x5C, 0x84, 0xC2, 0xf1, 0x83, 0x7f, 0x13, 0x3c, 0x1F,
    0x51, 0x38, 0xEF, 0x7f,
]);

sol_storage! {
    pub struct Contract {
        uint8 stage;
        address issuer_addr;
        uint256 issuer_deposit;
        address filler_addr;
        uint256 filler_deposit;
        uint8[2][] checkpoints;
        uint16 current_checkpoint_idx;
        uint64 next_position_deadline;
    }
}

sol_storage! {
    #[entrypoint]
    pub struct TrajectoryEnforcer {
        mapping(uint256 => Contract) contracts;
    }
}

sol! {
    struct ObservedState {
        uint8 contract_stage;
        address issuer_addr;
        address filler_addr;
        uint8[2][] checkpoints;
        uint16 current_checkpoint_idx;
        uint64 next_checkpoint_target;
    }
}

pub enum CreationError {
    ContractIDInUse,
}

impl From<CreationError> for Vec<u8> {
    fn from(value: CreationError) -> Self {
        match value {
            CreationError::ContractIDInUse => vec![0],
        }
    }
}

pub enum ActivationError {
    NoContractAtID,
    ContractAlreadyActivated,
    WrongCollateralAmount,
    NotTheFiller,
}

impl From<ActivationError> for Vec<u8> {
    fn from(value: ActivationError) -> Self {
        match value {
            ActivationError::NoContractAtID => vec![0],
            ActivationError::ContractAlreadyActivated => vec![1],
            ActivationError::WrongCollateralAmount => vec![2],
            ActivationError::NotTheFiller => vec![3],
        }
    }
}

pub enum UploadError {
    NoContractAtID,
    NotTheFiller,
    ContractNotActive,
}

impl From<UploadError> for Vec<u8> {
    fn from(value: UploadError) -> Self {
        match value {
            UploadError::NoContractAtID => vec![0],
            UploadError::NotTheFiller => vec![1],
            UploadError::ContractNotActive => vec![2],
        }
    }
}

pub enum ArbitrationError {
    NoContractAtID,
    NotTheArbitrator,
    NotInvalid,
    InvalidDecision,
}

impl From<ArbitrationError> for Vec<u8> {
    fn from(value: ArbitrationError) -> Self {
        match value {
            ArbitrationError::NoContractAtID => vec![0],
            ArbitrationError::NotTheArbitrator => vec![1],
            ArbitrationError::NotInvalid => vec![2],
            ArbitrationError::InvalidDecision => vec![3],
        }
    }
}

pub enum CheckingError {
    WrongState,
}

impl From<CheckingError> for Vec<u8> {
    fn from(value: CheckingError) -> Self {
        match value {
            CheckingError::WrongState => vec![0],
        }
    }
}

#[external]
impl TrajectoryEnforcer {
    /// Called by an issuer to create a contract.
    #[payable]
    pub fn new_contract(
        &mut self,
        contract_id: U256,
        filler_addr: Address,
        filler_deposit: U256,
        checkpoints: Vec<[u8; 2]>,
    ) -> Result<(), Vec<u8>> {
        let mut new_contract = self.contracts.setter(contract_id);
        if *new_contract.stage != CONTRACT_UNINIT {
            Err(CreationError::ContractIDInUse.into())
        } else {
            new_contract.stage.set(CONTRACT_CREATED);
            new_contract.issuer_addr.set(msg::sender());
            new_contract.issuer_deposit.set(msg::value());
            new_contract.filler_addr.set(filler_addr);
            new_contract.filler_deposit.set(filler_deposit);
            for checkpoint in checkpoints {
                let mut new_checkpoint = new_contract.checkpoints.grow();
                new_checkpoint
                    .get_mut(0)
                    .unwrap()
                    .set(U8::from(checkpoint[0]));
                new_checkpoint
                    .get_mut(1)
                    .unwrap()
                    .set(U8::from(checkpoint[1]));
            }
            //new_contract.current_checkpoint_idx.set(U16::ZERO);
            Ok(())
        }
    }

    /// Called by the filler to activate a contract.
    #[payable]
    pub fn activate_contract(&mut self, contract_id: U256) -> Result<(), Vec<u8>> {
        let mut contract = self.contracts.setter(contract_id);
        match *contract.stage {
            CONTRACT_UNINIT => Err(ActivationError::NoContractAtID.into()),
            CONTRACT_CREATED if msg::sender() != contract.filler_addr.get() => {
                Err(ActivationError::NotTheFiller.into())
            }
            CONTRACT_CREATED if msg::value() != contract.filler_deposit.get() => {
                Err(ActivationError::WrongCollateralAmount.into())
            }
            CONTRACT_CREATED => {
                contract.stage.set(CONTRACT_ACTIVATED);
                contract
                    .next_position_deadline
                    .set(U64::from(stylus_sdk::block::number()));
                Ok(())
            }
            _ => Err(ActivationError::ContractAlreadyActivated.into()),
        }
    }

    /// Called by the filler to update its position.
    pub fn upload_position(&mut self, contract_id: U256, position: [u8; 2]) -> Result<(), Vec<u8>> {
        let mut contract = self.contracts.setter(contract_id);
        let block_number = stylus_sdk::block::number();
        if contract.stage.get() == CONTRACT_UNINIT {
            Err(UploadError::NoContractAtID.into())
        } else if msg::sender() != contract.filler_addr.get() {
            Err(UploadError::NotTheFiller.into())
        } else if contract.stage.get() != CONTRACT_ACTIVATED {
            Err(UploadError::ContractNotActive.into())
        } else if U64::from(block_number) > contract.next_position_deadline.get() {
            contract.stage.set(CONTRACT_LOCKED);
            Ok(())
        } else {
            let idx1 = contract.current_checkpoint_idx.get();
            let idx = usize::from_be_bytes([0, 0, idx1.byte(1), idx1.byte(0)]);
            let checkpoint_pos = contract.checkpoints.get(idx).unwrap();
            let (checkpoint_x, checkpoint_y) = (
                checkpoint_pos.get(0).unwrap().byte(0),
                checkpoint_pos.get(1).unwrap().byte(1),
            );
            let d1 = Self::distance(checkpoint_x, checkpoint_y, position[0], position[1]);
            if idx == contract.checkpoints.len() - 1 {
                match d1 {
                    _ if d1 <= 1.41421356237f64 => {
                        contract.stage.set(CONTRACT_UNINIT);
                        transfer_eth(
                            contract.filler_addr.get(),
                            contract.issuer_deposit.get() + contract.filler_deposit.get(),
                        )
                    }
                    _ if d1 <= 4.0f64 => Ok(()),
                    _ => {
                        contract.stage.set(CONTRACT_LOCKED);
                        Ok(())
                    }
                }
            } else if d1 >= 7.0f64 {
                contract.stage.set(CONTRACT_LOCKED);
                Ok(())
            } else {
                let next_checkpoint_pos = contract.checkpoints.get(idx + 1).unwrap();
                let (next_checkpoint_x, next_checkpoint_y) = (
                    next_checkpoint_pos.get(0).unwrap().byte(0),
                    next_checkpoint_pos.get(1).unwrap().byte(1),
                );
                let d2 = Self::distance(
                    next_checkpoint_x,
                    next_checkpoint_y,
                    position[0],
                    position[1],
                );
                if d2 <= d1 {
                    contract.current_checkpoint_idx.set(idx1 + U16::from(1));
                    contract
                        .next_position_deadline
                        .set(U64::from(block_number + BLOCK_PER_CHECKPOINT));
                }
                Ok(())
            }
        }
    }

    /// Called by the issuer to monitor the travel route.
    pub fn check_state(&self, contract_id: U256) -> Result<u8, Vec<u8>> {
        let contract = self.contracts.getter(contract_id);
        if U64::from(stylus_sdk::block::number()) > contract.next_position_deadline.get() {
            Ok(CONTRACT_LOCKED.byte(0))
        } else {
            Ok(contract.stage.get().byte(0))
        }
    }

    /// Called by the arbitrator when the contract is invalid.
    pub fn arbitrate(&mut self, contract_id: U256, decision: u8) -> Result<(), Vec<u8>> {
        let mut contract = self.contracts.setter(contract_id);
        if contract.stage.get() == CONTRACT_UNINIT {
            Err(ArbitrationError::NoContractAtID.into())
        } else if msg::sender() != ARBITRATOR_ADDRESS {
            Err(ArbitrationError::NotTheArbitrator.into())
        } else if contract.stage.get() != CONTRACT_LOCKED
            && contract.next_position_deadline.get() <= U64::from(stylus_sdk::block::number())
        {
            Err(ArbitrationError::NotInvalid.into())
        } else {
            match decision {
                ARBITRAGE_FULFILL => {
                    contract.stage.set(CONTRACT_UNINIT);
                    transfer_eth(
                        contract.filler_addr.get(),
                        contract.issuer_deposit.get() + contract.filler_deposit.get(),
                    )
                }
                ARBITRAGE_REFUND => {
                    contract.stage.set(CONTRACT_UNINIT);
                    transfer_eth(
                        contract.issuer_addr.get(),
                        contract.issuer_deposit.get() + contract.filler_deposit.get(),
                    )
                }
                _ => Err(ArbitrationError::InvalidDecision.into()),
            }
        }
    }
}

impl TrajectoryEnforcer {
    fn distance(x1: u8, y1: u8, x2: u8, y2: u8) -> f64 {
        let dx = f64::from(x1) - f64::from(x2);
        let dy = f64::from(y1) - f64::from(y2);
        f64::sqrt(dx * dx + dy * dy)
    }
}
