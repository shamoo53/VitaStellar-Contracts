//! Typed contract errors.

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    InputTooLong = 4,
    /// Raised when `reentrancy::enter` returns `false` because the lock is
    /// already held — i.e. a guarded function was re-entered mid-call.
    ReentrantCall = 5,
}