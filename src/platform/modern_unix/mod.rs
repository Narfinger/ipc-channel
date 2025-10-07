/*
pub use self::os::{channel, OsOpaqueIpcChannel};
pub use self::os::{OsIpcChannel, OsIpcOneShotServer, OsIpcReceiverSet};
pub use self::os::{OsIpcSelectionResult, OsIpcSharedMemory};
*/

use std::os::{
    fd::{IntoRawFd, RawFd},
    unix::net::UnixStream,
};

use crate::ipc::IpcError;

#[derive(PartialEq)]
struct OsIpcSender {
    stream: RawFd,
}

#[derive(PartialEq)]
pub enum OsIpcChannel {
    Sender(OsIpcSender),
    Receiver(OsIpcReceiver),
}

impl OsIpcSender {
    pub fn send(
        &self,
        data: &[u8],
        channels: Vec<OsIpcChannel>,
        shared_memory_regions: Vec<OsIpcSharedMemory>,
    ) -> Result<(), UnixError> {
        
    }
}

#[derive(PartialEq)]
struct OsIpcReceiver {
    stream: RawFd,
}

pub fn channel() -> Result<(OsIpcSender, OsIpcReceiver), IpcError> {
    let (sender, recv) = std::os::unix::net::UnixStream::pair().unwrap();
    Ok((
        OsIpcSender {
            stream: sender.into_raw_fd(),
        },
        OsIpcReceiver {
            stream: recv.into_raw_fd(),
        },
    ))
}
