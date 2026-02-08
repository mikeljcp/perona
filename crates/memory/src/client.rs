use perona_types::ClientAddress;

use crate::memory::Memory;

pub struct Client {
    memory: Memory,
}

impl Client {
    pub(crate) fn new(memory: &Memory) -> Self {
        Self {
            memory: memory.clone(),
        }
    }

    pub fn messages(&self) -> String {
        let module = self.memory.main_module();

        if let Ok(igcn) = module {
            let messages = self
                .memory
                .read::<[u8; 32]>(igcn, ClientAddress::Messages as usize);

            return self.memory.bytes_to_string(messages.unwrap());
        }

        return "".to_string();
    }
 }