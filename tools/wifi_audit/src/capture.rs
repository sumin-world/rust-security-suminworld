use pcap::{Active, Capture};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CaptureError {
    #[error("timeout")]
    Timeout,
    #[error(transparent)]
    Other(#[from] pcap::Error),
}

pub struct Cap {
    cap: Capture<Active>,
}

pub fn open(iface: &str, filter: &str) -> Result<Cap, CaptureError> {
    let mut cap = Capture::from_device(iface)?
        .rfmon(true)
        .promisc(true)
        .immediate_mode(true)
        .timeout(500) // ms (i32)
        .open()?;

    // 👇 setnonblock()는 self를 consume 하므로, 반드시 재대입해야 함
    cap = cap.setnonblock()?; 

    cap.filter(filter, true)?;
    Ok(Cap { cap })
}

impl Cap {
    pub fn next_frame(&mut self) -> Result<&[u8], CaptureError> {
        match self.cap.next_packet() {
            Ok(pkt) => Ok(pkt.data),
            Err(pcap::Error::NoMorePackets) | Err(pcap::Error::TimeoutExpired) => {
                Err(CaptureError::Timeout)
            }
            Err(e) => Err(CaptureError::Other(e)),
        }
    }
}
