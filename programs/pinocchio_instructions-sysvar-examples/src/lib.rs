#![no_std]

use pinocchio::{entrypoint, account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult};

entrypoint!(process_instruction);

pub mod instructions;
pub use instructions::*;

// 22222222222222222222222222222222222222222222
pub const ID: Pubkey = [
    0x0f, 0x1e, 0x6b, 0x14, 0x21, 0xc0, 0x4a, 0x07, 
    0x04, 0x31, 0x26, 0x5c, 0x19, 0xc5, 0xbb, 0xee, 
    0x19, 0x92, 0xba, 0xe8, 0xaf, 0xd1, 0xcd, 0x07, 
    0x8e, 0xf8, 0xaf, 0x70, 0x47, 0xdc, 0x11, 0xf7, 
];


fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {    
    match instruction_data.split_first() {
        Some((GetInstructionRelative::DISCRIMINATOR, _)) => GetInstructionRelative::try_from(accounts)?.process(),
        Some((LoadInstructionAt::DISCRIMINATOR, _)) => LoadInstructionAt::try_from(accounts)?.process(),
        _ => Err(ProgramError::InvalidInstructionData)
    }
}

// Only include panic handler when compiling for Solana BPF/SBF target
#[cfg(target_os = "solana")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}