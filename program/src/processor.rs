use pinocchio::{entrypoint::ProgramResult, msg};

pub fn process_ping() -> ProgramResult {
    msg!("I've got no strings");
    msg!("To hold me down");
    msg!("To make me fret");
    msg!("Or make me frown");
    msg!("I had strings");
    msg!("But now I'm free");
    msg!("There are no strings on me");

    Ok(())
}
