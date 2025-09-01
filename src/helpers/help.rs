use crate::{
    db::DatabaseManager,
    models::help::{DatabaseStatus, SystemMetrics},
};
use std::time::Instant;
use sysinfo::{Disks, System};

pub async fn check_database_health(db: &DatabaseManager) -> DatabaseStatus {
    let start_time = Instant::now();

    match sqlx::query("SELECT 1 as test").fetch_one(db.get_pool())
        .await
    {
        Ok(_) => DatabaseStatus {
            connected: true,
            response_time_ms: Some(start_time.elapsed().as_millis() as u64),
            error: None,
        },
        Err(e) => DatabaseStatus {
            connected: false,
            response_time_ms: None,
            error: Some(e.to_string()),
        },
    }
}

pub fn get_system_metrics() -> SystemMetrics {
    let mut sys = System::new();

    sys.refresh_cpu_usage();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();
    sys.refresh_memory();

    let cpu_usage = if !sys.cpus().is_empty() {
        let total: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum();
        total / sys.cpus().len() as f32
    } else {
        0.0
    };

    let cpu_count = sys.cpus().len().max(1);

    let memory_used = sys.used_memory() / 1024 / 1024;
    let memory_total = sys.total_memory() / 1024 / 1024;
    let memory_usage_percent = if memory_total > 0 {
        (memory_used as f32 / memory_total as f32) * 100.0
    } else {
        0.0
    };

    let disks = Disks::new_with_refreshed_list();
    let disk_usage_percent = if let Some(disk) = disks.first() {
        let total = disk.total_space();
        let available = disk.available_space();
        if total > 0 {
            let used = total - available;
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        }
    } else {
        0.0
    };

    println!(
        "Debug CPU: individual_cores=[{}], average={:.1}%",
        sys.cpus()
            .iter()
            .map(|cpu| format!("{:.1}", cpu.cpu_usage()))
            .collect::<Vec<_>>()
            .join(", "),
        cpu_usage
    );

    SystemMetrics {
        cpu_usage,
        cpu_count,
        memory_used_mb: memory_used,
        memory_total_mb: memory_total,
        memory_usage_percent,
        disk_usage_percent,
        uptime: System::uptime(),
    }
}
