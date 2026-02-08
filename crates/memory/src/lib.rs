use crate::{client::Client, memory::Memory};

mod client;
mod memory;

pub fn run(pid: u32) -> Result<Client, String> {
    let process = proc_mem::Process::with_pid(pid);

    match process {
        Ok(process) => {
            let memory = Memory::new(process);
            let client = Client::new(&memory);

            if memory.igcn_module().is_err() {
                return Err("IGCN module not loadead.".to_string());
            }

            Ok(client)
        }
        Err(_) => Err("Execute .exe or terminal using mode administrator".to_string()),
    }
}
