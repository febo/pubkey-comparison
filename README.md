# `pubkey-comparison`

A quick test of pubkey values. Pubkeys can be compared as a sequence of 4 `u64` values ("program_manual"), which in some cases if more efficient than doing a direct `==` comparison ("program_operator").

### Building and Running

A [`Makefile`](https://github.com/febo/playground/blob/main/Makefile) is provided with basic commands to:
* `bench`: run a specific bench test against a program.
* `build`: build both programs.
* `clean`: remove all build files.
* `clippy`: run `cargo clippy` on the workspace.
* `format`: run `cargo fmt` on the workspace.

To execute a program, it is first necessary to build them:
```bash
make build
```

To run a `bench` in a particular program:
```bash
make bench <program>
```

For example, to run the bench tests on the "program_manual":
```bash
make bench program_manual
```

After the execution, mollusk with report the compute units in a `compute_units.md` located at `./target/benches`.
```
#### 2025-09-16 10:49:26.381107 UTC

Solana CLI Version: solana-cli 2.1.22 (src:26944979; feat:1416569292, client:Agave)

| Name | CUs | Delta |
|------|------|-------|
| Different | 10 | - new - |
| Same | 18 | - new - |
```

When you make modification or run a different program but execute the same bench test, the "Delta" column will show the difference in CUs compared to the previous run.

## Adding a bench test

To add a new bench test, go to `tests/benches/setup` folder and edit the `mod.rs` file with a new `MolluskComputeUnitBencher`. Then call the new bench from the test runner of each program – these are located in `tests/benches`.

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
