use crate::cheese::mem::Process;

impl Process {
    #[allow(dead_code)]
    pub unsafe fn get_base_addr(&self) -> usize {
        self.base_address
    }
}
