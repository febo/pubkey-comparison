#![feature(test)]

mod setup;

extern crate mollusk_svm;
extern crate mollusk_svm_bencher;
extern crate solana_account;
extern crate solana_instruction;
extern crate solana_pubkey;
extern crate test;

use solana_pubkey::Pubkey;

const PROGRAM_ID: Pubkey = Pubkey::new_from_array([2; 32]);

#[cfg(test)]
mod pinocchio {

    use super::*;
    use test::Bencher;

    #[bench]
    fn run_different(_bencher: &mut Bencher) {
        setup::cmp_pubkey(&PROGRAM_ID, "operator");
    }
}
