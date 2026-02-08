use crate::memory::Memory;

pub(crate) struct Client {
    memory: Memory,
}

impl Client {
    pub(crate) fn new(memory: &Memory) -> Self {
        Self {
            memory: memory.clone(),
        }
    }
}