mod array;
mod class;
mod code;
mod field;
mod interface_method;
mod method;
mod object;
mod opcode;
mod reference;

#[cfg(test)]
mod test;

use std::sync::PoisonError;

pub use array::*;
pub use class::*;
pub use code::*;
pub use field::*;
pub use interface_method::*;
pub use method::*;
pub use object::*;
pub use opcode::*;
pub use reference::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InternalError {
    EmptyStack,
    WrongType,
    PoisedMutex,
    EmptyLocals,
    LocalsOutOfBounds,
    InvalidWideLoad,
    InvalidProgrammCounter,
}

impl<Guard> From<PoisonError<Guard>> for InternalError {
    fn from(_: PoisonError<Guard>) -> Self {
        InternalError::PoisedMutex
    }
}
