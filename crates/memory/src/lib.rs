use crate::{
    client::Client,
    entity::Entity,
    memory::Memory,
};

mod client;
mod entity;
mod memory;

pub fn run(pid: u32)  {
    let process = proc_mem::Process::with_pid(pid);

     match process {
        Ok(process) => {
            let memory = Memory::new(process);

            if !memory.loaded() {
                return;
            }

            let entity = Entity::new(&memory);
            let _ = Client::new(&memory);

            if memory.igcn_module().is_err() {
                return
            }

            if !entity.is_logged() {
                return;
            }
        }
        Err(_) => {
            return;
        }
    }
}