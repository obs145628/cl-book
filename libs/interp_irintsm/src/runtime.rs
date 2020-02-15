use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::num::Wrapping;

use irintsm::ir;

/// Represent a word value in the Runtime, it's always signed 32 bits integer
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

// Contains the local variables and operands stack for each function frame
struct Frame {
    locals: HashMap<ir::LocalsIndex, RTVal>,
    operands: Vec<RTVal>,
}

impl Frame {
    fn new() -> Self {
        Frame {
            locals: HashMap::new(),
            operands: vec![],
        }
    }

    fn new_from_call(args: &Vec<RTVal>) -> Self {
        let mut res = Frame::new();
        for (idx, arg) in args.iter().enumerate() {
            res.set_local(ir::LocalsIndex::new(idx), *arg);
        }
        res
    }

    fn get_local(&self, id: ir::LocalsIndex) -> RTVal {
        match self.locals.get(&id) {
            Some(val) => *val,
            None => RTVal(0),
        }
    }

    fn set_local(&mut self, id: ir::LocalsIndex, val: RTVal) {
        self.locals.insert(id, val);
    }

    fn pop_op(&mut self) -> RTVal {
        self.operands
            .pop()
            .expect("Failed to pop operands stack: its already empty")
    }

    fn pop_2_ops(&mut self) -> (RTVal, RTVal) {
        let right = self.pop_op();
        let left = self.pop_op();
        (left, right)
    }

    fn pop_n_ops(&mut self, n: usize) -> Vec<RTVal> {
        let res: Vec<_> = (0..n).map(|_| self.pop_op()).collect();
        res.into_iter().rev().collect()
    }

    fn push_op(&mut self, val: RTVal) {
        self.operands.push(val);
    }
}

// Needed to know the next instruction to run

// A code address is representation by the function, basic block, and instruction offset in the basic block
#[derive(Clone, Debug)]
struct CodeAddress {
    fun: ir::FunctionRef,
    bb: ir::BasicBlockRef,
    ins_pos: usize,
}

impl CodeAddress {
    pub fn go_to_bb(&mut self, bb: ir::BasicBlockRef) {
        self.bb = bb;
        self.ins_pos = 0;
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

        self.call_stack
            .push(self.addr_of_function_begin(ir::FunctionRef::new(0)));
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
        let fun = self.code.get_fun(addr.fun);
        let bb = fun.get_bb(addr.bb);
        let ins = bb
            .ins_list()
            .get(addr.ins_pos)
            .expect("Failed to get instruction: invalid position address");
        ins
    }

    // Return the current instruction, doesn't move the pc
    fn fetch_ins(&self) -> &ir::Ins {
        self.get_ins(self.call_stack.last().unwrap())
    }

    // Simply go to the following instruction in the code (doesn't do any branch / call)
    fn next_ins(&mut self) {
        let addr = self.call_stack.last_mut().unwrap();
        addr.ins_pos += 1;
    }

    // Get register value on the current frame
    fn get_local(&self, id: ir::LocalsIndex) -> RTVal {
        self.frames.last().unwrap().get_local(id)
    }

    // Get register value on the current frame
    fn set_local(&mut self, id: ir::LocalsIndex, val: RTVal) {
        self.frames.last_mut().unwrap().set_local(id, val);
    }

    // Pop 1 value from the operands stack on the current frame
    fn pop_op(&mut self) -> RTVal {
        self.frames.last_mut().unwrap().pop_op()
    }

    // Pop 2 values from the operands stack on the current frame
    fn pop_2_ops(&mut self) -> (RTVal, RTVal) {
        self.frames.last_mut().unwrap().pop_2_ops()
    }

    // Pop n values from the operands stack on the current frame
    fn pop_n_ops(&mut self, n: usize) -> Vec<RTVal> {
        self.frames.last_mut().unwrap().pop_n_ops(n)
    }

    // Push 1 value to the operands stack on the current frame
    fn push_op(&mut self, val: RTVal) {
        self.frames.last_mut().unwrap().push_op(val);
    }

    fn addr_of_function_begin(&self, fun_id: ir::FunctionRef) -> CodeAddress {
        let fun = self.code.get_fun(fun_id);
        let bb = &fun.bb_list()[0];
        CodeAddress {
            fun: fun_id,
            bb: bb.id(),
            ins_pos: 0,
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
            ir::Ins::Pop(ins) => self.exec_ins_pop(ins),
            ir::Ins::Const(ins) => self.exec_ins_const(ins),
            ir::Ins::Load(ins) => self.exec_ins_load(ins),
            ir::Ins::Store(ins) => self.exec_ins_store(ins),
            ir::Ins::Opbin(ins) => self.exec_ins_opbin(ins),
            ir::Ins::Cmpbin(ins) => self.exec_ins_cmpbin(ins),
            ir::Ins::Jump(ins) => self.exec_ins_jump(ins),
            ir::Ins::Br(ins) => self.exec_ins_br(ins),
            ir::Ins::Call(ins) => self.exec_ins_call(ins),
            ir::Ins::Ret(ins) => self.exec_ins_ret(ins),
        }
    }

    fn exec_ins_pop(&mut self, _ins: ir::InsPop) {
        self.pop_op();
        self.next_ins();
    }

    fn exec_ins_const(&mut self, ins: ir::InsConst) {
        self.push_op(RTVal(ins.val()));
        self.next_ins();
    }

    fn exec_ins_load(&mut self, ins: ir::InsLoad) {
        let val = self.get_local(ins.src());
        self.push_op(val);
        self.next_ins();
    }

    fn exec_ins_store(&mut self, ins: ir::InsStore) {
        let val = self.pop_op();
        self.set_local(ins.dst(), val);
        self.next_ins();
    }

    fn exec_ins_opbin(&mut self, ins: ir::InsOpbin) {
        let (src1, src2) = self.pop_2_ops();
        let (src1, src2) = (Wrapping(src1.0), Wrapping(src2.0));

        let res = match ins {
            ir::InsOpbin::Add => src1 + src2,
            ir::InsOpbin::Sub => src1 - src2,
            ir::InsOpbin::Mul => src1 * src2,
            ir::InsOpbin::Div => src1 / src2,
            ir::InsOpbin::Rem => src1 % src2,
        };

        //println!("{:?}: {} . {} => {}", ins, src1.0, src2.0, res.0);

        self.push_op(RTVal(res.0));
        self.next_ins();
    }

    fn exec_ins_cmpbin(&mut self, ins: ir::InsCmpbin) {
        let (src1, src2) = self.pop_2_ops();

        let res = match ins {
            ir::InsCmpbin::Eq => src1.0 == src2.0,
            ir::InsCmpbin::Lt => src1.0 < src2.0,
            ir::InsCmpbin::Gt => src1.0 > src2.0,
        } as i32;

        self.push_op(RTVal(res));
        self.next_ins();
    }

    fn exec_ins_jump(&mut self, ins: ir::InsJump) {
        self.call_stack.last_mut().unwrap().go_to_bb(ins.dst());
    }

    fn exec_ins_br(&mut self, ins: ir::InsBr) {
        let cond_val = self.pop_op();
        let dst = if cond_val.0 != 0 {
            ins.dst_true()
        } else {
            ins.dst_false()
        };
        self.call_stack.last_mut().unwrap().go_to_bb(dst);
    }

    fn exec_ins_call(&mut self, ins: ir::InsCall) {
        let args = self.pop_n_ops(ins.nb_args());
        let fun = self.code.get_fun(ins.fun());
        //println!("call {}: {:?}", fun.id(), args);
        if fun.is_extern() {
            let ret = self.call_native(ins.fun(), args);
            self.push_op(ret);
            self.next_ins();
            return;
        }

        self.frames.push(Frame::new_from_call(&args));
        self.call_stack.push(self.addr_of_function_begin(ins.fun()));
    }

    fn exec_ins_ret(&mut self, _ins: ir::InsRet) {
        if self.call_stack.len() == 1 {
            panic!("Failed to exec ret instruction: it's the top frame");
        }

        let ret_val = self.pop_op();
        self.frames.pop();
        self.call_stack.pop();
        self.push_op(ret_val);
        self.next_ins();
    }

    fn call_native(&mut self, fun: ir::FunctionRef, args: Vec<RTVal>) -> RTVal {
        let fun_putc = ir::FunctionRef::new(257);
        let fun_exit = ir::FunctionRef::new(258);
        let fun_getc = ir::FunctionRef::new(259);
        let fun_fmemget = ir::FunctionRef::new(260);
        let fun_fmemset = ir::FunctionRef::new(261);
        let fun_fmemcpy = ir::FunctionRef::new(262);

        if fun == fun_putc {
            self.call_native_putc(args)
        } else if fun == fun_exit {
            self.call_native_exit(args)
        } else if fun == fun_getc {
            self.call_native_getc(args)
        } else if fun == fun_fmemget {
            self.call_native_fmemget(args)
        } else if fun == fun_fmemset {
            self.call_native_fmemset(args)
        } else if fun == fun_fmemcpy {
            self.call_native_fmemcpy(args)
        } else {
            panic!("Failed to called extern function: unknown id {}", fun)
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
