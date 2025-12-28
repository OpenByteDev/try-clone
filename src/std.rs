use std::{
    fs::File,
    io::{self, PipeReader, PipeWriter},
    net::{TcpListener, TcpStream, UdpSocket},
};

#[cfg(feature = "blanket-impl")]
use crate::ForwardTryCloneToClone;
use crate::TryClone;

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for File {}
impl TryClone for File {
    type Err = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        File::try_clone(self)
    }
}

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for PipeReader {}
impl TryClone for PipeReader {
    type Err = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        PipeReader::try_clone(self)
    }
}

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for PipeWriter {}
impl TryClone for PipeWriter {
    type Err = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        PipeWriter::try_clone(self)
    }
}

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for TcpStream {}
impl TryClone for TcpStream {
    type Err = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        TcpStream::try_clone(self)
    }
}

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for TcpListener {}
impl TryClone for TcpListener {
    type Err = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        TcpListener::try_clone(self)
    }
}

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for UdpSocket {}
impl TryClone for UdpSocket {
    type Err = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        UdpSocket::try_clone(self)
    }
}
