use crate::parse::Event;
use comfy_table::{Cell, Table}; // Cell 추가
use macaddr::MacAddr6;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct Stats {
    pub ssid_beacons: BTreeMap<String, BeaconInfo>,
    pub clients: BTreeSet<MacAddr6>,
}

#[derive(Default)]
pub struct BeaconInfo {
    pub bssids: BTreeSet<MacAddr6>,
    pub channels: BTreeSet<u8>,
    pub beacons: u64,
}

impl Stats {
    pub fn ingest(&mut self, ev: Event) {
        match ev {
            Event::Beacon { bssid, ssid, chan } => {
                let mac = MacAddr6::from(bssid);
                let name = ssid.to_string();
                let entry = self.ssid_beacons.entry(name).or_default();
                entry.bssids.insert(mac);
                if let Some(c) = chan {
                    entry.channels.insert(c);
                }
                entry.beacons += 1;
            }
            Event::ProbeReq { sta, ssid } => {
                self.clients.insert(MacAddr6::from(sta));
                if let Some(name) = ssid {
                    if !name.is_empty() {
                        // Track which SSIDs clients are probing for
                        self.ssid_beacons.entry(name).or_default();
                    }
                }
            }
            Event::ProbeRes { bssid, ssid } => {
                let mac = MacAddr6::from(bssid);
                let name = ssid.to_string();
                let entry = self.ssid_beacons.entry(name).or_default();
                entry.bssids.insert(mac);
            }
        }
    }
}

pub fn render_table(stats: &Stats, table: &mut Table, list_clients: bool) {
    table.set_header(vec!["SSID", "BSSIDs", "Channels", "Beacons"]);
    for (ssid, info) in stats.ssid_beacons.iter() {
        let bssids = join_set(&info.bssids);
        let chans = join_set(&info.channels);
        table.add_row(vec![
            Cell::new(ssid.clone()),
            Cell::new(bssids),
            Cell::new(chans),
            Cell::new(info.beacons.to_string()),
        ]);
    }
    if list_clients {
        table.add_row(vec![
            Cell::new("—"),
            Cell::new("—"),
            Cell::new("—"),
            Cell::new("—"),
        ]);
        table.add_row(vec![
            Cell::new("Probing Clients"),
            Cell::new(format!("{} stations", stats.clients.len())),
            Cell::new(""),
            Cell::new(""),
        ]);
    }
}

fn join_set<T: ToString + Ord + Clone>(set: &BTreeSet<T>) -> String {
    let mut out = String::new();
    for (i, v) in set.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&v.to_string());
    }
    out
}
