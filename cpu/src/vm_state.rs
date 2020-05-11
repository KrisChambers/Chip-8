use data::Nibble;
use instruction::Instruction;

/// Possible Virtual Machine States.
///
#[derive(PartialEq)]
pub enum VMState {
    Initializing,
    LoadingROM,
    Executing(Instruction),
    Paused,
    WaitingForKey(Option<Nibble>)
}