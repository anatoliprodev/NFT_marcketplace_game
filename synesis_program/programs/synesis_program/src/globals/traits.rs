use anchor_lang::prelude::*;

pub trait Processor<T> {
    fn process(&mut self, args: T) -> ProgramResult;
}

pub trait ProcessorWithProgramId<T> {
    fn process(&mut self, args: T, program_id: &Pubkey) -> ProgramResult;
}
