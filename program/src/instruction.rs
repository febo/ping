use pinocchio::program_error::ProgramError;

#[derive(Clone, Debug)]
#[rustfmt::skip]
pub enum Instruction {
    Ping,
}

impl Instruction {
    /// Unpacks a byte buffer into a [Instruction](enum.Instruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        match input.split_first() {
            // 0 - Ping
            Some((&0, [])) => Ok(Instruction::Ping),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
