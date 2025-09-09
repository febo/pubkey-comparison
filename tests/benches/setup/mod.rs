use mollusk_svm::Mollusk;
use mollusk_svm_bencher::MolluskComputeUnitBencher;
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

/// System program ID, used for creating accounts.
const SYSTEM_PROGRAM: Pubkey = Pubkey::new_from_array([0; 32]);

/// Base lamports for accounts, used to ensure accounts are rent-exempt.
pub const BASE_LAMPORTS: u64 = 2_000_000_000u64;

/// Create a new Mollusk instance for the given program ID and name.
pub fn setup(program_id: &Pubkey, name: &'static str) -> Mollusk {
    std::env::set_var("SBF_OUT_DIR", "../target/deploy");
    solana_logger::setup();

    Mollusk::new(program_id, name)
}

/// Generate a set of unique public keys.
pub fn generate_pubkeys(count: usize) -> Vec<Pubkey> {
    let mut keys = Vec::with_capacity(count);
    for _ in 0..count {
        keys.push(Pubkey::new_unique());
    }
    keys
}

pub fn cmp_pubkey(program_id: &Pubkey, name: &'static str) {
    let mollusk = setup(program_id, name);
    let mut bencher = MolluskComputeUnitBencher::new(mollusk)
        .must_pass(true)
        .out_dir("../target/benches");

    let mut keys = generate_pubkeys(1);
    let mut accounts = Vec::with_capacity(keys.len());
    let mut account_metas = Vec::with_capacity(keys.len());

    for _ in 0..keys.len() {
        let key = keys.pop().unwrap();
        accounts.push((key, Account::new(BASE_LAMPORTS, 0, &SYSTEM_PROGRAM)));
        account_metas.push(AccountMeta::new_readonly(key, false));
    }

    let instruction = Instruction {
        program_id: *program_id,
        accounts: account_metas,
        data: vec![],
    };

    bencher = bencher.bench(("Different", &instruction, &accounts));

    let mut accounts = Vec::with_capacity(keys.len());
    let mut account_metas = Vec::with_capacity(keys.len());

    for _ in 0..keys.len() {
        accounts.push((*program_id, Account::new(BASE_LAMPORTS, 0, &SYSTEM_PROGRAM)));
        account_metas.push(AccountMeta::new_readonly(*program_id, false));
    }

    let instruction = Instruction {
        program_id: *program_id,
        accounts: account_metas,
        data: vec![],
    };

    bencher = bencher
        .bench(("Same", &instruction, &accounts))
        .must_pass(false);

    bencher.execute();
}
