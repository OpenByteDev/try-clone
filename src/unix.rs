#[cfg(not(any(
    target_arch = "wasm32",
    target_os = "hermit",
    target_os = "trusty",
    target_os = "motor"
)))]
use std::os::fd::BorrowedFd;
use std::os::fd::OwnedFd;
use std::{
    io,
    os::unix::net::{UnixDatagram, UnixListener, UnixStream},
};

use crate::TryClone;
use crate::TryCloneToOwned;

#[cfg(not(any(
    target_arch = "wasm32",
    target_os = "hermit",
    target_os = "trusty",
    target_os = "motor"
)))]
impl TryClone for OwnedFd {
    type Err = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        OwnedFd::try_clone(self)
    }
}

#[cfg(not(any(
    target_arch = "wasm32",
    target_os = "hermit",
    target_os = "trusty",
    target_os = "motor"
)))]
impl TryCloneToOwned for BorrowedFd<'_> {
    type Owned = OwnedFd;
    type Err = io::Error;

    fn try_clone_to_owned(&self) -> Result<Self::Owned, Self::Err> {
        BorrowedFd::try_clone_to_owned(self)
    }
}

impl TryClone for UnixDatagram {
    type Err = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        UnixDatagram::try_clone(self)
    }
}

impl TryClone for UnixListener {
    type Err = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        UnixListener::try_clone(self)
    }
}

impl TryClone for UnixStream {
    type Err = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        UnixStream::try_clone(self)
    }
}
