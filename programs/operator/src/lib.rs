use pinocchio::{
    entrypoint::{InstructionContext, MaybeAccount},
    lazy_program_entrypoint,
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
    ProgramResult,
};

const CMP_KEY: Pubkey = [2u8; PUBKEY_BYTES];

lazy_program_entrypoint!(process_instruction);

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    let account = if let MaybeAccount::Account(account) = context.next_account()? {
        account
    } else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if cmp(account.key(), &CMP_KEY) {
        return Err(ProgramError::InvalidArgument);
    }

    Ok(())
}

/// Compares two Pubkeys for equality.
///
/// Use `no_mangle` to generate similar asm as a manual `u64` comparison.
#[inline(always)]
//#[no_mangle]
pub fn cmp(p1: &Pubkey, p2: &Pubkey) -> bool {
    p1 == p2
}
