use crate::{
    client::Client,
    entity::Entity,
    memory::Memory,
};

mod client;
mod constants;
mod entity;
mod memory;

pub fn run(pid: u32)  {
    let process = proc_mem::Process::with_pid(pid);

    match process {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}