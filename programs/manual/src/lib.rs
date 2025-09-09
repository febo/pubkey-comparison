use std::ptr::read_unaligned;

use pinocchio::{
    entrypoint::{InstructionContext, MaybeAccount},
    lazy_program_entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

lazy_program_entrypoint!(process_instruction);

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    let account = if let MaybeAccount::Account(account) = context.next_account()? {
        account
    } else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let program_id = context.program_id()?;

    if cmp(account.key(), program_id) {
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}

/// Compares two Pubkeys for equality.
///
/// Use `no_mangle` to generate similar asm as using `==` operator.
#[inline(always)]
//#[no_mangle]
pub const fn cmp(p1: &Pubkey, p2: &Pubkey) -> bool {
    let p1_ptr = p1.as_ptr() as *const u64;
    let p2_ptr = p2.as_ptr() as *const u64;

    unsafe {
        read_unaligned(p1_ptr) == read_unaligned(p2_ptr)
            && read_unaligned(p1_ptr.add(1)) == read_unaligned(p2_ptr.add(1))
            && read_unaligned(p1_ptr.add(2)) == read_unaligned(p2_ptr.add(2))
            && read_unaligned(p1_ptr.add(3)) == read_unaligned(p2_ptr.add(3))
    }
}
