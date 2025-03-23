
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, sysvars::instructions::load_instruction_at_checked, ProgramResult
};

pub struct LoadInstructionAtAccounts<'a> {
    pub from: &'a AccountInfo,
    pub to: &'a AccountInfo,
    pub sysvar_instructions: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for LoadInstructionAtAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [from, to, sysvar_instructions] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Return the accounts
        Ok(Self { from, to, sysvar_instructions })
    }
}

pub struct LoadInstructionAt<'a> {
    pub accounts: LoadInstructionAtAccounts<'a>,
}

impl<'a> TryFrom<&'a [AccountInfo]> for LoadInstructionAt<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = LoadInstructionAtAccounts::try_from(accounts)?;

        Ok(Self {
            accounts,
        })
    }
}

impl<'a> LoadInstructionAt<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;

    pub fn process(&mut self) -> ProgramResult {
        let instruction = load_instruction_at_checked(0, &self.accounts.sysvar_instructions)?;

        if instruction.get_program_id() != &pinocchio_system::ID {
            return Err(ProgramError::InvalidInstructionData);
        }

        let mut instruction_data = [0u8; 12];
        instruction_data[0..4].copy_from_slice(&2u32.to_le_bytes());
        instruction_data[4..12].copy_from_slice(&100_000_000u64.to_le_bytes());
 
        if instruction.get_instruction_data() != instruction_data {
            return Err(ProgramError::InvalidInstructionData);
        }

        if instruction.get_account_meta_at(0)?.key() != self.accounts.from.key() {
            return Err(ProgramError::InvalidAccountData);
        }

        if instruction.get_account_meta_at(1)?.key() != self.accounts.to.key() {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}
