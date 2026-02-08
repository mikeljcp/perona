use crate::{client::Client, memory::Memory};

mod client;
mod memory;
mod parser;

pub fn read(pid: u32) -> Result<Client, String> {
    let process = proc_mem::Process::with_pid(pid);

    match process {
        Ok(process) => {
            let memory = Memory::new(process);

            if memory.main_module().is_err() {
                return Err("Main module not loadead.".to_string());
            }

            Ok(Client::new(memory))
        }
        Err(_) => Err("Execute .exe or terminal using mode administrator".to_string()),
    }
}
