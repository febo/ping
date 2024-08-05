use pinocchio::{account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::{instruction::Instruction, processor::process_ping};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instruction::unpack(instruction_data)?;

    match instruction {
        Instruction::Ping => process_ping(),
    }
}
