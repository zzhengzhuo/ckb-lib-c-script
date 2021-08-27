
#![no_std]

#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod code_hashes;
#[cfg(feature = "no_std")]
mod libsmt;

pub use code_hashes::CODE_HASH_CKB_SMT;
#[cfg(feature = "no_std")]
pub use libsmt::*;


#[cfg(feature = "c_file")]
pub fn get_libsmt_bin() -> std::vec::Vec<u8> {
    include_bytes!("../lib/ckb_smt").to_vec()
}