use proc_mem::{Module, ProcMemError, Process};

#[derive(Debug, Clone)]
pub(crate) struct Memory {
    process: Process,
}

impl Memory {
    pub(crate) fn new(process: Process) -> Self {
        Self { process }
    }

    pub(crate) fn read<T: std::default::Default>(
        &self,
        module: Module,
        address: usize,
    ) -> Result<T, ProcMemError> {
        self.process.read_mem::<T>(module.base_address() + address)
    }

    pub(crate) fn bytes_to_string(&self, value: [u8; 32]) -> String {
        std::ffi::CStr::from_bytes_until_nul(&value)
            .unwrap()
            .to_str()
            .unwrap_or("")
            .to_string()
    }

    pub(crate) fn main_module(&self) -> Result<Module, ProcMemError> {
        self.process.module(self.process.name())
    }

    pub(crate) fn igcn_module(&self) -> Result<Module, ProcMemError> {
        self.process.module("IGC.dll")
    }
}