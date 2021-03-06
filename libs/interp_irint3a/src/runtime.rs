use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::num::Wrapping;

use irint3a::ir;

/// Represent a word value in the Runtime, it's usually a signed integer or an address
#[derive(Clone, Copy, Debug)]
struct RTVal(i32);

/// The exit code at the end of the execution
#[derive(Clone, Copy, Debug)]
pub struct ExitCode(u8);

impl ExitCode {
    pub fn get_val(&self) -> i32 {
        self.0 as i32
    }
}

// Contains all the registers and local variables for each function frame
struct Frame {
    regs: HashMap<ir::RegId, RTVal>,
    locals: Vec<RTVal>,
    ret_reg: ir::RegId, //where the caller wants the return value to be saved
}

impl Frame {
    fn new() -> Self {
        Frame {
            regs: HashMap::new(),
            locals: vec![],
            ret_reg: ir::RegId(0),
        }
    }

    fn new_from_call(args: &Vec<RTVal>, dst: ir::RegId) -> Self {
        let mut res = Frame::new();
        for (idx, arg) in args.iter().enumerate() {
            res.set_reg(ir::RegId(idx), *arg);
        }
        res.ret_reg = dst;
        res
    }

    fn get_ret_reg(&self) -> ir::RegId {
        self.ret_reg
    }

    fn get_reg(&self, reg: ir::RegId) -> RTVal {
        match self.regs.get(&reg) {
            Some(val) => *val,
            None => RTVal(0),
        }
    }

    fn set_reg(&mut self, reg: ir::RegId, val: RTVal) {
        self.regs.insert(reg, val);
    }

    // Allocate a new local variable, and returns its index
    fn alloca(&mut self) -> usize {
        let res = self.locals.len();
        self.locals.push(RTVal(0));
        res
    }
}

// Needed to know the next instruction to run

// A code address is representation by the function and the instruction index in the function
#[derive(Clone, Debug)]
struct CodeAddress {
    fun: ir::FunctionId,
    bb: ir::BasicBlockId,
    pos: usize, //offset in basicblock
}

impl CodeAddress {
    pub fn jump_to_bb(&mut self, id: ir::BasicBlockId) {
        self.bb = id;
        self.pos = 0;
    }
}

// Only local variables on the stack are addressable
// As such, an adress has 2 parts: the frame index, and the local index in the frame
#[derive(Clone, Copy, Debug)]
struct MemAddress(RTVal);

impl MemAddress {
    fn new(frame_idx: usize, local_idx: usize) -> Self {
        let frame_idx = frame_idx as u32;
        let local_idx = local_idx as u32;
        let addr = (frame_idx << 16) | local_idx;
        MemAddress(RTVal(addr as i32))
    }

    fn frame_idx(&self) -> usize {
        let addr = (self.0).0 as u32;
        (addr >> 16) as usize
    }

    fn local_idx(&self) -> usize {
        let addr = (self.0).0 as u32;
        (addr & 0xFFFF) as usize
    }
}

const FLAT_MEMORY_SIZE: i32 = 16 * 1024 * 1024;

pub struct FlatMemory {
    data: Vec<i32>,
}

impl FlatMemory {
    pub fn new() -> Self {
        FlatMemory { data: vec![] }
    }

    pub fn load(&self, pos: i32) -> i32 {
        self.check_idx(pos);
        if self.data.len() == 0 {
            0
        } else {
            self.data[pos as usize]
        }
    }

    pub fn store(&mut self, pos: i32, val: i32) {
        self.check_idx(pos);
        self.lazy_init();
        self.data[pos as usize] = val;
    }

    pub fn copy(&mut self, dst: i32, src: i32, len: i32) {
        self.check_idx(src);
        self.check_idx(src + len - 1);
        self.check_idx(dst);
        self.check_idx(dst + len - 1);
        if self.data.len() == 0 {
            return;
        }

        let data_ptr = self.data.as_mut_ptr();
        unsafe {
            let src_ptr = data_ptr.offset(src as isize);
            let dst_ptr = data_ptr.offset(dst as isize);
            std::ptr::copy(src_ptr, dst_ptr, len as usize);
        }
    }

    fn check_idx(&self, idx: i32) {
        if idx < 0 {
            panic!("flat memory: trying to access negative index");
        }
        if idx >= FLAT_MEMORY_SIZE {
            panic!("flat memory: trying to access beyond fmem size");
        }
    }

    fn lazy_init(&mut self) {
        if self.data.len() == 0 {
            self.data = Vec::with_capacity(FLAT_MEMORY_SIZE as usize);
            unsafe {
                self.data.set_len(FLAT_MEMORY_SIZE as usize);
            }
        }
    }
}

pub struct Runtime {
    code: ir::Module,
    frames: Vec<Frame>,
    call_stack: Vec<CodeAddress>,
    ins_status: Option<ExitCode>, //status of last executed instruction

    stdin: Vec<u8>,
    stdin_pos: usize,
    stdout: Vec<u8>,
    fmem: FlatMemory,
}

impl Runtime {
    /// Create a new initialized runtime
    pub fn new(code: ir::Module) -> Self {
        let mut res = Runtime {
            code,
            frames: vec![],
            call_stack: vec![],
            ins_status: None,

            stdin: vec![],
            stdin_pos: 0,
            stdout: vec![],
            fmem: FlatMemory::new(),
        };
        res.reset();
        res
    }

    /// Reset the Runtime to the starting point of the program
    pub fn reset(&mut self) {
        self.frames.clear();
        self.call_stack.clear();
        self.stdout.clear();
        self.ins_status = None;

        self.call_stack.push(self.begin_of_fun(ir::FunctionId(0)));
        self.frames.push(Frame::new());
    }

    /// Run only one instruction
    /// Returns an exitcode if the instruction calls exit
    pub fn step(&mut self) -> Option<ExitCode> {
        let ins = self.fetch_ins().clone();
        self.exec_ins(ins);
        self.ins_status
    }

    /// Run the program until the end
    pub fn run(&mut self) -> ExitCode {
        loop {
            if let Some(ret) = self.step() {
                return ret;
            }
        }
    }

    /// Set stdin stream from raw bytes data
    pub fn reset_stdin_raw(&mut self, data: &[u8]) {
        self.stdin = Vec::from(data);
        self.stdin_pos = 0;
    }

    /// Set stdin stream from binary file
    pub fn reset_stdin_path(&mut self, path: &str) {
        let mut f = File::open(path).expect("Failed to open stdin file");
        self.stdin.clear();
        f.read_to_end(&mut self.stdin)
            .expect("Failed to read stdin file");
        self.stdin_pos = 0;
    }

    /// Returns the output of the program
    pub fn stdout(&self) -> &[u8] {
        &self.stdout
    }

    fn get_ins(&self, addr: &CodeAddress) -> &ir::Ins {
        self.code
            .get_fun(addr.fun)
            .expect("Failed to get instruction: invalid function address")
            .get_basic_block(addr.bb)
            .get_ins(addr.pos)
    }

    // Return the current instruction, doesn't move the pc
    fn fetch_ins(&self) -> &ir::Ins {
        self.get_ins(self.call_stack.last().unwrap())
    }

    // Simply go to the following instruction in the code (doesn't do any branch / call)
    fn next_ins(&mut self) {
        let addr = self.call_stack.last_mut().unwrap();
        addr.pos += 1;
    }

    fn get_mem(&self, addr: &MemAddress) -> &RTVal {
        self.frames
            .get(addr.frame_idx())
            .expect("Failed to access memory: invalid frame index")
            .locals
            .get(addr.local_idx())
            .expect("Failed to access memory: invalid local index")
    }

    fn get_mem_mut(&mut self, addr: &MemAddress) -> &mut RTVal {
        self.frames
            .get_mut(addr.frame_idx())
            .expect("Failed to access memory: invalid frame index")
            .locals
            .get_mut(addr.local_idx())
            .expect("Failed to access memory: invalid local index")
    }

    // Load 32b data from memory
    fn load(&self, addr: &MemAddress) -> RTVal {
        *self.get_mem(addr)
    }

    // Store 32n data to memory
    fn store(&mut self, addr: &MemAddress, val: RTVal) {
        *self.get_mem_mut(addr) = val;
    }

    // Get register value on the current frame
    fn get_reg(&self, reg: ir::RegId) -> RTVal {
        self.frames.last().unwrap().get_reg(reg)
    }

    // Get register value on the current frame
    fn set_reg(&mut self, reg: ir::RegId, val: RTVal) {
        self.frames.last_mut().unwrap().set_reg(reg, val);
    }

    fn begin_of_fun(&self, fun_id: ir::FunctionId) -> CodeAddress {
        let fun = self.code.get_fun(fun_id).unwrap();
        let bb_id = fun.basic_blocks_list()[0];
        CodeAddress {
            fun: fun_id,
            bb: bb_id,
            pos: 0,
        }
    }

    fn exec_ins(&mut self, ins: ir::Ins) {
        /*
        println!(
                "exec ins {:?} at {:?}",
                ins,
                self.call_stack.last().unwrap()
            );
        */

        self.ins_status = None;
        match ins {
            ir::Ins::Movi(ins) => self.exec_ins_movi(ins),
            ir::Ins::Movr(ins) => self.exec_ins_movr(ins),
            ir::Ins::Load(ins) => self.exec_ins_load(ins),
            ir::Ins::Store(ins) => self.exec_ins_store(ins),
            ir::Ins::Alloca(ins) => self.exec_ins_alloca(ins),
            ir::Ins::Opbin(ins) => self.exec_ins_opbin(ins),
            ir::Ins::Cmpbin(ins) => self.exec_ins_cmpbin(ins),
            ir::Ins::Jump(ins) => self.exec_ins_jump(ins),
            ir::Ins::Br(ins) => self.exec_ins_br(ins),
            ir::Ins::Call(ins) => self.exec_ins_call(ins),
            ir::Ins::Ret(ins) => self.exec_ins_ret(ins),
        }
    }

    fn exec_ins_movi(&mut self, ins: ir::InsMovi) {
        self.set_reg(ins.dst(), RTVal(ins.const_val()));
        self.next_ins();
    }

    fn exec_ins_movr(&mut self, ins: ir::InsMovr) {
        self.set_reg(ins.dst(), self.get_reg(ins.src()));
        self.next_ins();
    }

    fn exec_ins_load(&mut self, ins: ir::InsLoad) {
        let src_addr = MemAddress(self.get_reg(ins.src()));
        self.set_reg(ins.dst(), self.load(&src_addr));
        self.next_ins();
        /*
        println!(
                "load r{}, r{} ({})",
                ins.dst().0,
                ins.src().0,
                self.load(&src_addr).0,
            );
        */
    }

    fn exec_ins_store(&mut self, ins: ir::InsStore) {
        let dst_addr = MemAddress(self.get_reg(ins.dst()));
        self.store(&dst_addr, self.get_reg(ins.src()));
        self.next_ins();
        /*
        println!(
                "store r{}, r{} ({})",
                ins.dst().0,
                ins.src().0,
                self.get_reg(ins.src()).0,
            );
        */
    }

    fn exec_ins_alloca(&mut self, ins: ir::InsAlloca) {
        let frame_idx = self.frames.len() - 1;
        let local_idx = self.frames.last_mut().unwrap().alloca();
        let addr = MemAddress::new(frame_idx, local_idx);
        self.set_reg(ins.dst(), addr.0);
        self.next_ins();
    }

    fn exec_ins_opbin(&mut self, ins: ir::InsOpbin) {
        let src1 = Wrapping(self.get_reg(ins.src1()).0);
        let src2 = Wrapping(self.get_reg(ins.src2()).0);

        let res = match ins.kind() {
            ir::InsOpbinKind::Add => src1 + src2,
            ir::InsOpbinKind::Sub => src1 - src2,
            ir::InsOpbinKind::Mul => src1 * src2,
            ir::InsOpbinKind::Div => src1 / src2,
            ir::InsOpbinKind::Mod => src1 % src2,
        };

        self.set_reg(ins.dst(), RTVal(res.0));
        self.next_ins();
    }

    fn exec_ins_cmpbin(&mut self, ins: ir::InsCmpbin) {
        let src1 = self.get_reg(ins.src1()).0;
        let src2 = self.get_reg(ins.src2()).0;

        let res = match ins.kind() {
            ir::InsCmpbinKind::Eq => src1 == src2,
            ir::InsCmpbinKind::Lt => src1 < src2,
            ir::InsCmpbinKind::Gt => src1 > src2,
        } as i32;

        self.set_reg(ins.dst(), RTVal(res));
        self.next_ins();
    }

    fn exec_ins_jump(&mut self, ins: ir::InsJump) {
        self.call_stack.last_mut().unwrap().jump_to_bb(ins.dst());
        //println!("jump L{}", ins.dst().0);
    }

    fn exec_ins_br(&mut self, ins: ir::InsBr) {
        let cond_val = self.get_reg(ins.src());
        let next_bb = if cond_val.0 != 0 {
            ins.dst_true()
        } else {
            ins.dst_false()
        };
        self.call_stack.last_mut().unwrap().jump_to_bb(next_bb);
        /*
        println!(
                "br r{}, L{}, L{}, (L{})",
                ins.src().0,
                ins.dst_true().0,
                ins.dst_false().0,
                next_bb.0
            );
        */
    }

    fn exec_ins_call(&mut self, ins: ir::InsCall) {
        let args = ins.args().iter().map(|x| self.get_reg(*x)).collect();
        let fun = self.code.get_fun(ins.fun()).expect(&format!(
            "Failed to call function: unkown function address {}",
            ins.fun().0
        ));
        if fun.is_extern() {
            let ret = self.call_native(ins.fun(), args);
            self.set_reg(ins.dst(), ret);
            self.next_ins();
            return;
        }

        self.frames.push(Frame::new_from_call(&args, ins.dst()));
        self.call_stack.push(self.begin_of_fun(ins.fun()));
        //println!("call F{}", ins.fun().0);
    }

    fn exec_ins_ret(&mut self, ins: ir::InsRet) {
        if self.call_stack.len() == 1 {
            panic!("Failed to exec ret instruction: it's the top frame");
        }

        let ret_val = self.get_reg(ins.src());
        let ret_reg = self.frames.last().unwrap().get_ret_reg();
        self.frames.pop();
        self.call_stack.pop();
        self.next_ins();
        self.set_reg(ret_reg, ret_val);
    }

    fn call_native(&mut self, fun: ir::FunctionId, args: Vec<RTVal>) -> RTVal {
        match fun.0 {
            257 => self.call_native_putc(args),
            258 => self.call_native_exit(args),
            259 => self.call_native_getc(args),
            260 => self.call_native_fmemget(args),
            261 => self.call_native_fmemset(args),
            262 => self.call_native_fmemcpy(args),
            _ => panic!("Failed to called extern function: unknown id {}", fun.0),
        }
    }

    fn call_native_putc(&mut self, args: Vec<RTVal>) -> RTVal {
        if args.len() != 1 {
            panic!(
                "Failed to call putc: expected 1 argument, got {}",
                args.len()
            )
        }

        let bval = args[0].0 as u8;
        self.stdout.push(bval);
        RTVal(0)
    }

    fn call_native_exit(&mut self, args: Vec<RTVal>) -> RTVal {
        if args.len() != 1 {
            panic!(
                "Failed to call exit: expected 1 argument, got {}",
                args.len()
            )
        }

        let exit_val = args[0].0 as u8;
        self.ins_status = Some(ExitCode(exit_val));
        RTVal(0)
    }

    fn call_native_getc(&mut self, args: Vec<RTVal>) -> RTVal {
        if args.len() != 0 {
            panic!(
                "Failed to call getc: expected 0 argument, got {}",
                args.len()
            )
        }

        match self.stdin.get(self.stdin_pos) {
            Some(bval) => {
                self.stdin_pos += 1;
                RTVal(*bval as i32)
            }
            None => RTVal(-1), //eof
        }
    }

    fn call_native_fmemget(&mut self, args: Vec<RTVal>) -> RTVal {
        if args.len() != 1 {
            panic!(
                "Failed to call fmemget: expected 1 argument, got {}",
                args.len()
            )
        }

        let pos = args[0].0;
        let val = self.fmem.load(pos);
        //println!("get @{} => {}", pos, val);
        RTVal(val)
    }

    fn call_native_fmemset(&mut self, args: Vec<RTVal>) -> RTVal {
        if args.len() != 2 {
            panic!(
                "Failed to call fmemdet: expected 2 arguments, got {}",
                args.len()
            )
        }

        let pos = args[0].0;
        let val = args[1].0;
        self.fmem.store(pos, val);
        //println!("set @{}, {}", pos, val);
        RTVal(0)
    }

    fn call_native_fmemcpy(&mut self, args: Vec<RTVal>) -> RTVal {
        if args.len() != 3 {
            panic!(
                "Failed to call fmemdet: expected 3 arguments, got {}",
                args.len()
            )
        }

        let dst = args[0].0;
        let src = args[1].0;
        let len = args[2].0;
        self.fmem.copy(dst, src, len);
        RTVal(0)
    }
}
