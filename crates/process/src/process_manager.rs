use std::{ffi::OsString, path::Path};
use sysinfo::{Pid, Process, System};

#[derive(Debug, Clone)]
pub struct Processes {
    pub pids: Vec<u32>,
    pub details: Vec<Details>,
}

#[derive(Debug, Clone)]
pub struct Details {
    pub pid: u32,
    pub name: String,
}

pub struct ProcessManager;

impl ProcessManager {
    fn system(&self) -> System {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();
        system
    }

    pub fn get_process(&self, process_name: String) -> Option<Processes> {
        let system = self.system();

        let process_name = self.process_name(process_name);

        let processes: Vec<(&Pid, &Process)> = system.processes().iter().collect();

        let game_pids = Self.game_pids(&process_name, &processes);

        if game_pids.is_empty() {
            return None;
        }

        Some(Processes {
            pids: game_pids,
            details: self.process_details(&processes),
        })
    }

    fn process_name(&self, name: String) -> String {
        (name + ".exe").to_lowercase()
    }

    fn game_pids(&self, process_name: &String, processes: &Vec<(&Pid, &Process)>) -> Vec<u32> {
        processes
            .iter()
            .filter(|(_, process)| {
                process.name().to_ascii_lowercase() == OsString::from(process_name)
                    && self.exe_exists(process.exe())
            })
            .map(|(_, process)| process.pid().as_u32())
            .collect()
    }

    fn exe_exists(&self, path: Option<&Path>) -> bool {
        match path {
            Some(exe) => exe.exists(),
            None => false,
        }
    }

    fn process_details(&self, processes: &Vec<(&Pid, &Process)>) -> Vec<Details> {
        processes
            .iter()
            .map(|(_, process)| Details {
                pid: process.pid().as_u32(),
                name: process.name().to_string_lossy().to_lowercase(),
            })
            .collect()
    }
}