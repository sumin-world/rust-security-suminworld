use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum Event {
    Beacon {
        bssid: [u8; 6],
        ssid: Cow<'static, str>,
        chan: Option<u8>,
    },
    ProbeReq {
        sta: [u8; 6],
        ssid: Option<String>,
    },
    ProbeRes {
        bssid: [u8; 6],
        ssid: Cow<'static, str>,
    },
}

pub fn parse_wifi_frame(mut bytes: &[u8]) -> Option<Event> {
    if bytes.len() < 4 {
        return None;
    }
    // Radiotap 헤더(보통 앞에 붙음) 스킵
    if bytes[0] == 0 && bytes[1] == 0 {
        let rt_len = u16::from_le_bytes([bytes[2], bytes[3]]) as usize;
        if rt_len >= 8 && rt_len < bytes.len() {
            bytes = &bytes[rt_len..];
        }
    }

    if bytes.len() < 24 {
        return None;
    }
    let fc = u16::from_le_bytes([bytes[0], bytes[1]]);
    let ftype = ((fc >> 2) & 0b11) as u8; // 0 = mgmt
    let fsub = ((fc >> 4) & 0b1111) as u8; // subtype
    if ftype != 0 {
        return None;
    }

    match fsub {
        8 => parse_beacon(bytes),
        4 => parse_probe_req(bytes),
        5 => parse_probe_res(bytes),
        _ => None,
    }
}

fn parse_beacon(bytes: &[u8]) -> Option<Event> {
    if bytes.len() < 36 {
        return None;
    }
    let bssid = bytes[16..22].try_into().ok()?;
    let mut pos = 24 + 12; // timestamp+interval+capab
    let mut ssid: Option<String> = None;
    let mut chan: Option<u8> = None;

    while pos + 2 <= bytes.len() {
        let id = bytes[pos];
        let len = bytes[pos + 1] as usize;
        pos += 2;
        if pos + len > bytes.len() {
            break;
        }
        match id {
            0 => {
                ssid = Some(String::from_utf8_lossy(&bytes[pos..pos + len]).to_string());
            }
            3 => {
                if len == 1 {
                    chan = Some(bytes[pos]);
                }
            }
            _ => {}
        }
        pos += len;
    }

    Some(Event::Beacon {
        bssid,
        ssid: ssid.unwrap_or_default().into(),
        chan,
    })
}

fn parse_probe_req(bytes: &[u8]) -> Option<Event> {
    if bytes.len() < 24 {
        return None;
    }
    let sta = bytes[10..16].try_into().ok()?; // SA
    let mut pos = 24;
    let mut ssid: Option<String> = None;
    while pos + 2 <= bytes.len() {
        let id = bytes[pos];
        let len = bytes[pos + 1] as usize;
        pos += 2;
        if pos + len > bytes.len() {
            break;
        }
        if id == 0 {
            ssid = Some(String::from_utf8_lossy(&bytes[pos..pos + len]).to_string());
        }
        pos += len;
    }
    Some(Event::ProbeReq { sta, ssid })
}

fn parse_probe_res(bytes: &[u8]) -> Option<Event> {
    if bytes.len() < 24 {
        return None;
    }
    let bssid = bytes[10..16].try_into().ok()?; // SA as BSSID in many responses
    let mut pos = 24 + 12;
    if pos > bytes.len() {
        return None;
    }
    let mut ssid: Option<String> = None;
    while pos + 2 <= bytes.len() {
        let id = bytes[pos];
        let len = bytes[pos + 1] as usize;
        pos += 2;
        if pos + len > bytes.len() {
            break;
        }
        if id == 0 {
            ssid = Some(String::from_utf8_lossy(&bytes[pos..pos + len]).to_string());
        }
        pos += len;
    }
    Some(Event::ProbeRes {
        bssid,
        ssid: ssid.unwrap_or_default().into(),
    })
}
