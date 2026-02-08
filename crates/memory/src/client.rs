use perona_types::ClientAddress;

use crate::{memory::Memory, parser::Parser};

pub struct Client {
    memory: Memory,
}

impl Client {
    pub(crate) fn new(memory: Memory) -> Self {
        Self { memory }
    }

    pub fn messages(&self) -> Result<Parser, ()> {
        let module = self.memory.main_module();

        if let Ok(main) = module {
            let messages = self
                .memory
                .read::<[u8; 32]>(main, ClientAddress::Messages as usize);

            match messages {
                Ok(message) => {
                    let bytes_to_string = self.memory.bytes_to_string(message);
                    let parser = Parser::new(bytes_to_string.clone());

                    println!("[DEBUG - Messages] value: {}", bytes_to_string);

                    if parser.is_valid() {
                        return Ok(parser);
                    }

                    return Err(());
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                    return Err(());
                }
            }
        }

        Err(())
    }
}
