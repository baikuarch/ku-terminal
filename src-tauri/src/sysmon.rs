use serde::Serialize;
use sysinfo::{Disks, Networks, System};
use std::sync::Mutex;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceStats {
    pub cpu: f32,
    pub mem_used: u64,
    pub mem_total: u64,
    pub disk_used: u64,
    pub disk_total: u64,
    pub net_rx: u64,
    pub net_tx: u64,
}

/// Holds persistent handles so CPU / network deltas are measured between calls.
pub struct SysMonitor {
    sys: Mutex<System>,
    networks: Mutex<Networks>,
    disks: Mutex<Disks>,
}

impl Default for SysMonitor {
    fn default() -> Self {
        Self {
            sys: Mutex::new(System::new_all()),
            networks: Mutex::new(Networks::new_with_refreshed_list()),
            disks: Mutex::new(Disks::new_with_refreshed_list()),
        }
    }
}

impl SysMonitor {
    pub fn sample(&self) -> ResourceStats {
        let mut sys = self.sys.lock().unwrap();
        sys.refresh_cpu_usage();
        sys.refresh_memory();

        let cpu = sys.global_cpu_usage();
        let mem_used = sys.used_memory();
        let mem_total = sys.total_memory();

        let mut disks = self.disks.lock().unwrap();
        disks.refresh(true);
        let mut disk_total = 0u64;
        let mut disk_avail = 0u64;
        for d in disks.iter() {
            disk_total += d.total_space();
            disk_avail += d.available_space();
        }
        let disk_used = disk_total.saturating_sub(disk_avail);

        let mut networks = self.networks.lock().unwrap();
        networks.refresh(true);
        let mut net_rx = 0u64;
        let mut net_tx = 0u64;
        for (_iface, data) in networks.iter() {
            net_rx += data.received();
            net_tx += data.transmitted();
        }

        ResourceStats {
            cpu,
            mem_used,
            mem_total,
            disk_used,
            disk_total,
            net_rx,
            net_tx,
        }
    }
}
