use crate::{constants::EntityAddress, memory::Memory};

pub(crate) struct Entity {
    memory: Memory,
}

impl Entity {
    pub(crate) fn new(memory: &Memory) -> Self {
        Self {
            memory: memory.clone(),
        }
    }

    pub(crate) fn is_logged(&self) -> bool {
        let module = self.memory.igcn_module();

        if module.is_err() || self.username().is_empty() {
            return false;
        }

        return true;
    }

    pub(crate) fn username(&self) -> String {
        let module = self.memory.igcn_module();

        if let Ok(igcn) = module {
            let username = self
                .memory
                .read::<[u8; 32]>(igcn, EntityAddress::Nickname as usize);

            return self.memory.bytes_to_string(username.unwrap());
        }

        return "".to_string();
    }
}