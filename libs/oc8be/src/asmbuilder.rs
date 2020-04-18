#[repr(C)]
pub struct oc8_as_sfile_t {
    private: [u8; 0],
}

extern "C" {
    fn oc8_as_sfile_new() -> *mut oc8_as_sfile_t;
    fn oc8_as_sfile_free(af: *mut oc8_as_sfile_t);
}

pub struct ASMBuilder {
    handle: *mut oc8_as_sfile_t,
}

impl ASMBuilder {
    pub fn new() -> ASMBuilder {
        ASMBuilder {
            handle: unsafe { oc8_as_sfile_new() },
        }
    }
}

impl Drop for ASMBuilder {
    fn drop(&mut self) {
        unsafe { oc8_as_sfile_free(self.handle) };
    }
}
