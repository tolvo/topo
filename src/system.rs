use sysinfo::System;

pub struct SystemStats {
    pub sys: System,
}

impl SystemStats {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
    }

    pub fn get_cpu_usage(&self) -> Vec<f32> {
        self.sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect()
    }

    pub fn get_mem_usage(&self) -> (u64, u64) {
        (self.sys.used_memory(), self.sys.total_memory())
    }

    pub fn get_processes(&self) -> Vec<ProcessInfo> {
        self.sys
            .processes()
            .values()
            .map(|p| ProcessInfo {
                pid: p.pid().as_u32(),
                name: p.name().to_string_lossy().into_owned(),
                cpu: p.cpu_usage(),
                mem: p.memory(),
            })
            .collect()
    }
}

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu: f32,
    pub mem: u64,
}
