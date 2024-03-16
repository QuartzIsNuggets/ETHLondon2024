#![no_main]
#![no_std]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloc::vec::Vec;
use stylus_sdk::prelude::*;

use stylus_sdk::stylus_proc::entrypoint;

sol_storage! {
    #[entrypoint]
    pub struct {}
}
