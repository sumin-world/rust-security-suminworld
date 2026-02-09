use chrono::Utc;
use clap::{ArgAction, Parser};
use comfy_table::{presets::UTF8_FULL, Table};

mod capture;
mod parse;
mod report;

#[derive(Parser, Debug)]
#[command(name = "wifi_audit")]
#[command(about = "Passive Wi-Fi audit: SSIDs, beacons, probe req/resp, basic channel stats")]
struct Args {
    /// Wireless interface in monitor mode (e.g., wlan0mon)
    #[arg(short, long)]
    iface: String,

    /// Optional BPF filter (defaults to mgmt beacons + probe req/resp)
    #[arg(short, long)]
    filter: Option<String>,

    /// Stop after N frames (0 = run until Ctrl+C)
    #[arg(short = 'n', long, default_value_t = 0)]
    max_frames: u64,

    /// Print live table every N seconds
    #[arg(long, default_value_t = 5)]
    refresh_secs: u64,

    /// Also list stations that sent probe requests (potential clients)
    #[arg(long, action = ArgAction::SetTrue)]
    list_clients: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let default_filter =
        String::from("type mgt and (subtype beacon or subtype probe-req or subtype probe-res)");
    let filter = args.filter.unwrap_or(default_filter);

    let mut cap = capture::open(&args.iface, &filter)?;

    let mut stats = report::Stats::default();
    let mut frames_seen: u64 = 0;
    let mut next_print = Utc::now() + chrono::TimeDelta::seconds(args.refresh_secs as i64);

    loop {
        match cap.next_frame() {
            Ok(bytes) => {
                frames_seen += 1;
                if let Some(ev) = parse::parse_wifi_frame(bytes) {
                    stats.ingest(ev);
                }
            }
            Err(capture::CaptureError::Timeout) => {}
            Err(e) => return Err(anyhow::anyhow!(e)),
        }

        let now = Utc::now();
        if now >= next_print {
            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            report::render_table(&stats, &mut table, args.list_clients);
            println!("{}", table);
            next_print = now + chrono::TimeDelta::seconds(args.refresh_secs as i64);
        }

        if args.max_frames > 0 && frames_seen >= args.max_frames {
            break;
        }
    }

    Ok(())
}
