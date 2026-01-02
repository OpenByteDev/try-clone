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
    type Error = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        File::try_clone(self)
    }
}

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for PipeReader {}
impl TryClone for PipeReader {
    type Error = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        PipeReader::try_clone(self)
    }
}

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for PipeWriter {}
impl TryClone for PipeWriter {
    type Error = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        PipeWriter::try_clone(self)
    }
}

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for TcpStream {}
impl TryClone for TcpStream {
    type Error = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        TcpStream::try_clone(self)
    }
}

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for TcpListener {}
impl TryClone for TcpListener {
    type Error = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        TcpListener::try_clone(self)
    }
}

#[cfg(feature = "blanket-impl")]
impl !ForwardTryCloneToClone for UdpSocket {}
impl TryClone for UdpSocket {
    type Error = io::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        UdpSocket::try_clone(self)
    }
}
