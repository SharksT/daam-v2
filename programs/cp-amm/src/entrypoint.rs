use anchor_lang::solana_program;

#[cfg(not(feature = "no-entrypoint"))]
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) = solana_program::entrypoint::deserialize(input);

    anchor_lang::entry(program_id, &accounts, instruction_data)
}

solana_program::custom_heap_default!();
solana_program::custom_panic_default!();
