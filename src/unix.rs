#[cfg(feature = "blanket-impl")]
use crate::ForwardTryCloneToClone;
#[cfg(not(any(
    target_arch = "wasm32",
    target_os = "hermit",
    target_os = "trusty",
    target_os = "motor"
)))]
use std::{
    io,
    os::{
        fd::{BorrowedFd, OwnedFd},
        unix::net::{UnixDatagram, UnixListener, UnixStream},
    },
};

use crate::{TryClone, TryCloneToOwned};

#[cfg(all(
    feature = "blanket-impl",
    not(any(
        target_arch = "wasm32",
        target_os = "hermit",
        target_os = "trusty",
        target_os = "motor"
    ))
))]
impl !ForwardTryCloneToClone for OwnedFd {}
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

#[cfg(all(
    feature = "blanket-impl",
    not(any(
        target_arch = "wasm32",
        target_os = "hermit",
        target_os = "trusty",
        target_os = "motor"
    ))
))]
impl !ForwardTryCloneToClone for UnixStream {}
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

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for UnixStream {}
impl TryClone for UnixDatagram {
    type Err = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        UnixDatagram::try_clone(self)
    }
}

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for UnixStream {}
impl TryClone for UnixListener {
    type Err = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        UnixListener::try_clone(self)
    }
}

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for UnixStream {}
impl TryClone for UnixStream {
    type Err = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        UnixStream::try_clone(self)
    }
}
