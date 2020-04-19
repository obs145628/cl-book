use std::ffi::c_void;

#[repr(C)]
pub struct oc8_as_sfile_t {
    private: [u8; 0],
}

extern "C" {
    fn oc8_as_sfile_new() -> *mut oc8_as_sfile_t;
    fn oc8_as_sfile_free(af: *mut oc8_as_sfile_t);
    fn oc8_as_sfile_check(af: *mut oc8_as_sfile_t);

    fn oc8_as_sfile_ins_add_imm(af: *mut oc8_as_sfile_t, i_src: u8, r_dst: u8);

    fn oc8_as_print_sfile(
        af: *mut oc8_as_sfile_t,
        out_buf: *mut u8,
        out_buf_len: usize,
        buf_cb: *mut c_void,
        arg: *mut c_void,
    ) -> usize;
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

    pub fn check(&mut self) {
        unsafe { oc8_as_sfile_check(self.handle) }
    }

    pub fn ins_add_imm(&mut self, i_src: u32, r_dst: u32) {
        unsafe { oc8_as_sfile_ins_add_imm(self.handle, i_src as u8, r_dst as u8) }
    }

    pub fn to_text(&mut self) -> String {
        let size = unsafe {
            oc8_as_print_sfile(
                self.handle,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };
    }
}

impl Drop for ASMBuilder {
    fn drop(&mut self) {
        unsafe { oc8_as_sfile_free(self.handle) }
    }
}
