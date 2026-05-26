use crate::system::{ProcessInfo, SystemStats};

pub struct App {
    pub system: SystemStats,
    pub processes: Vec<ProcessInfo>,
    pub should_quit: bool,
    pub selected_index: usize,
}

impl App {
    pub fn new() -> Self {
        let system = SystemStats::new();
        let processes = system.get_processes();
        Self {
            system,
            processes,
            should_quit: false,
            selected_index: 0,
        }
    }

    pub fn tick(&mut self) {
        self.system.refresh();
        self.processes = self.system.get_processes();
        // Keep selection within bounds
        if !self.processes.is_empty() && self.selected_index >= self.processes.is_empty().then(|| 0).unwrap_or(self.processes.len()) {
             self.selected_index = self.processes.len().saturating_sub(1);
        }
    }

    pub fn next(&mut self) {
        if !self.processes.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.processes.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.processes.is_empty() {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = self.processes.len() - 1;
            }
        }
    }
}
