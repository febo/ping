use pinocchio::{entrypoint::ProgramResult, msg};

pub fn process_ping() -> ProgramResult {
    msg!("I've got no dependencies");
    msg!("To hold me down");
    msg!("To make me fret");
    msg!("Or make me frown");
    msg!("I had dependencies");
    msg!("But now I'm free");
    msg!("There are no dependencies on me");

    Ok(())
}
