use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use sysinfo::{ComponentExt, CpuExt, DiskExt, NetworkExt, PidExt, ProcessExt, System, SystemExt};

#[derive(Debug, Default, Serialize, Deserialize)]
struct Value {
    disks: Vec<String>,
    network: HashMap<String, (u64, u64)>,
    components: Vec<String>,
    memory: u64,
    used_memory: u64,
    swap: u64,
    used_swap: u64,
    system_name: Option<String>,
    kernel_version: Option<String>,
    os_version: Option<String>,
    host_name: Option<String>,
    len_cpu: usize,
    processes: Vec<ProcessInstance>,
    cpus: Vec<CpuInstance>,
}
#[derive(Debug, Serialize, Deserialize)]
struct CpuInstance {
    frequency: u64,
    cpu_usage: f32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProcessInstance {
    pid: u32,
    name: String,
    memory: u64,
    cpu_usage: f32,
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut value = Value::default();

    for disk in sys.disks() {
        value.disks.push(disk.name().to_str().unwrap().to_string());
    }

    for (interface_name, data) in sys.networks() {
        value.network.insert(
            interface_name.clone(),
            (data.received(), data.transmitted()),
        );
    }

    for component in sys.components() {
        value.components.push(component.label().to_string());
    }

    value.memory = sys.total_memory();
    value.used_memory = sys.used_memory();
    value.swap = sys.total_swap();
    value.used_swap = sys.used_swap();

    value.system_name = sys.name();
    value.kernel_version = sys.kernel_version();
    value.os_version = sys.os_version();
    value.host_name = sys.host_name();

    value.len_cpu = sys.cpus().len();

    for process in sys.processes() {
        value.processes.push(ProcessInstance {
            pid: process.0.as_u32(),
            name: process.1.name().to_string(),
            memory: process.1.memory(),
            cpu_usage: process.1.cpu_usage(),
        });
    }

    for cpu in sys.cpus() {
        value.cpus.push(CpuInstance {
            name: cpu.name().to_string(),
            frequency: cpu.frequency(),
            cpu_usage: cpu.cpu_usage(),
        });
    }

    println!("{}", json!(value));
}
