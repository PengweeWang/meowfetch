//! System information gathering and formatting.

use colored::Colorize;
use sysinfo::{Disks, Networks, System};

const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
const MAX_DISKS: usize = 5;

const IGNORED_IFACE_PREFIXES: &[&str] = &[
    "docker", "veth", "virbr", "br-", "vmnet", "VMware", "vboxnet",
];

const IGNORED_FILE_SYSTEMS: &[&str] = &[
    "tmpfs", "devtmpfs", "overlay", "squashfs", "iso9660", "autofs", "devfs",
];

fn label_value(label: &str, value: &str) -> String {
    format!("{}: {}", label.bright_blue(), value)
}

fn usage_percentage(used: f64, total: f64) -> Option<f64> {
    if total > 0.0 {
        Some(used / total * 100.0)
    } else {
        None
    }
}

fn colorize_percentage(percentage: Option<f64>) -> colored::ColoredString {
    match percentage {
        None => "N/A".bright_black(),
        Some(p) => {
            let text = format!("{p:.1}%");
            if p < 50.0 {
                text.green()
            } else if p < 90.0 {
                text.yellow()
            } else {
                text.red()
            }
        }
    }
}

fn gib(bytes: u64) -> f64 {
    bytes as f64 / GIB
}

fn format_uptime(secs: u64) -> String {
    let days = secs / 86_400;
    let hours = secs % 86_400 / 3_600;
    let mins = secs % 3_600 / 60;
    if days > 0 {
        format!("{days}d {hours}h")
    } else if hours > 0 {
        format!("{hours}h {mins}m")
    } else if mins > 0 {
        format!("{mins}m")
    } else {
        format!("{secs}s")
    }
}

fn local_ips() -> Option<String> {
    let networks = Networks::new_with_refreshed_list();
    let mut out = Vec::new();
    for (name, net) in &networks {
        if IGNORED_IFACE_PREFIXES
            .iter()
            .any(|prefix| name.starts_with(prefix))
        {
            continue;
        }
        for ip_net in net.ip_networks() {
            let ip = ip_net.addr;
            if !ip.is_ipv4() || ip.is_loopback() {
                continue;
            }
            if let std::net::IpAddr::V4(v4) = ip {
                let octets = v4.octets();
                if octets[0] == 169 && octets[1] == 254 {
                    continue;
                }
            }
            out.push(format!("{}/{} ({})", ip, ip_net.prefix, name.cyan()));
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out.join(", "))
    }
}

fn disk_rows() -> Vec<String> {
    let disks = Disks::new_with_refreshed_list();
    let mut rows = Vec::new();
    for disk in disks.list().iter().take(MAX_DISKS * 2) {
        if rows.len() >= MAX_DISKS {
            break;
        }
        let fs = disk.file_system().to_string_lossy();
        if IGNORED_FILE_SYSTEMS
            .iter()
            .any(|ignored| fs.eq_ignore_ascii_case(ignored))
        {
            continue;
        }
        let total = gib(disk.total_space());
        if total <= 0.0 {
            continue;
        }
        let used = total - gib(disk.available_space());
        let mount = disk.mount_point().to_string_lossy();
        rows.push(label_value(
            &format!("disk ({mount})"),
            &format!(
                "{used:.2} / {total:.2} GiB ({}) - {fs}",
                colorize_percentage(usage_percentage(used, total))
            ),
        ));
    }
    rows
}

pub(crate) fn collect() -> Vec<String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut rows = Vec::new();

    let title = format!(
        "{}@{}",
        whoami::username(),
        System::host_name().unwrap_or_else(|| "unknown".into())
    );
    rows.push(title.clone().bright_green().to_string());
    rows.push("━".repeat(title.chars().count()));

    let os = format!(
        "{} {}",
        System::name().unwrap_or_default(),
        System::os_version().unwrap_or_default()
    );
    let os = os.trim();
    rows.push(label_value(
        "sys ",
        if os.is_empty() { "unknown" } else { os },
    ));

    rows.push(label_value(
        "kern",
        &System::kernel_version().unwrap_or_else(|| "unknown".into()),
    ));
    rows.push(label_value("up  ", &format_uptime(System::uptime())));

    let cpu_brand = sys
        .cpus()
        .first()
        .map(|cpu| cpu.brand().trim())
        .filter(|brand| !brand.is_empty())
        .unwrap_or("Unknown CPU");
    rows.push(label_value(
        "cpu ",
        &format!("{cpu_brand} ({})", sys.cpus().len()),
    ));

    let mem_total = gib(sys.total_memory());
    let mem_used = gib(sys.used_memory());
    rows.push(label_value(
        "mem ",
        &format!(
            "{mem_used:.2} / {mem_total:.2} GiB ({})",
            colorize_percentage(usage_percentage(mem_used, mem_total))
        ),
    ));

    let swap_total = gib(sys.total_swap());
    if swap_total > 0.0 {
        let swap_used = gib(sys.used_swap());
        rows.push(label_value(
            "swap",
            &format!(
                "{swap_used:.2} / {swap_total:.2} GiB ({})",
                colorize_percentage(usage_percentage(swap_used, swap_total))
            ),
        ));
    } else {
        rows.push(label_value("swap", "disabled"));
    }

    rows.push(label_value(
        "ipv4",
        &local_ips().unwrap_or_else(|| "unknown".into()),
    ));

    rows.extend(disk_rows());

    rows.push(format!(
        "{}{}{}{}{}{}{}{}",
        "███".bright_red(),
        "███".bright_yellow(),
        "███".bright_green(),
        "███".bright_cyan(),
        "███".bright_blue(),
        "███".bright_magenta(),
        "███".bright_black(),
        "███".bright_white()
    ));
    rows.push(format!(
        "{}{}{}{}{}{}{}{}",
        "███".red(),
        "███".yellow(),
        "███".green(),
        "███".cyan(),
        "███".blue(),
        "███".magenta(),
        "███".black(),
        "███".white()
    ));

    rows
}
