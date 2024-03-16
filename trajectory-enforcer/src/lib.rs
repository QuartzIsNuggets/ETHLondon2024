//! A permanent on-chain contract registry and executor. Each instance of the contract binds two
//! parties. One expects a travel route to be respected, the other expects to be paid at the end.
//!
//! - A "route issuer" has a route in mind - for a taxi, for a delivery, etc. -, derives a contract
//!   instance from it, and locks the cost to fulfill the service here. The cost selection is out of
//!   scope, as it depends on the application using the framework.
//! - A "route filler" starts to fill the contract. It gives updates on a regular basis, threatened
//!   by a refund and a consumption of the collateral to pay for the gas - retroactively - and the
//!   rest to somewhere, agreed upon at contract instance creation.

#![no_main]
#![no_std]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use stylus_sdk::prelude::*;
use stylus_sdk::storage::{StorageMap, StorageU256, StorageU8, StorageVec};

use stylus_sdk::stylus_proc::entrypoint;

#[solidity_storage]
pub struct Checkpoint {
    x: StorageU8,
    y: StorageU8,
}

#[solidity_storage]
pub struct Contract {

}

#[entrypoint]
pub struct TrajectoryEnforcer {
    contracts: StorageMap<StorageU256, Contract>,
}

#[external]
impl TrajectoryEnforcer {
    // Initialize the contract.
    // Read warn level.
    /// Upload a new position.
    pub fn upload_checkpoint(&mut self, checkpoint: Checkpoint) {
    }
}
