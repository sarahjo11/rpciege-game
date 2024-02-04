#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};
#[contract]
pub struct Contract ;
#[contractimpl]
impl Contract {
 pub fn run(_env : Env, _nft_dest : Address) {}
}