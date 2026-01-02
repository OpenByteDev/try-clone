#[cfg(feature = "blanket-impl")]
use crate::ForwardTryCloneToClone;
use crate::{TryClone, TryCloneToOwned};
use std::{
    io,
    os::windows::io::{BorrowedHandle, BorrowedSocket, OwnedHandle, OwnedSocket},
};

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for OwnedHandle {}
impl TryClone for OwnedHandle {
    type Error = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        OwnedHandle::try_clone(self)
    }
}

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for OwnedSocket {}
impl TryClone for OwnedSocket {
    type Error = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        OwnedSocket::try_clone(self)
    }
}

impl TryCloneToOwned for BorrowedHandle<'_> {
    type Owned = OwnedHandle;
    type Error = io::Error;

    fn try_clone_to_owned(&self) -> Result<Self::Owned, Self::Error> {
        BorrowedHandle::try_clone_to_owned(self)
    }
}

impl TryCloneToOwned for BorrowedSocket<'_> {
    type Owned = OwnedSocket;
    type Error = io::Error;

    fn try_clone_to_owned(&self) -> Result<Self::Owned, Self::Error> {
        BorrowedSocket::try_clone_to_owned(self)
    }
}
