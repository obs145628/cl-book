use std::collections::HashMap;

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
        (0..n).map(|_| self.pop_op()).rev().collect()
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

pub struct Runtime {
    code: ir::Module,
    frames: Vec<Frame>,
    call_stack: Vec<CodeAddress>,
    ins_status: Option<ExitCode>, //status of last executed instruction

    stdout: Vec<u8>,
}

impl Runtime {
    /// Create a new initialized runtime
    pub fn new(code: ir::Module) -> Self {
        let mut res = Runtime {
            code,
            frames: vec![],
            call_stack: vec![],
            ins_status: None,

            stdout: vec![],
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

        let res = match ins {
            ir::InsOpbin::Add => src1.0 + src2.0,
            ir::InsOpbin::Sub => src1.0 - src2.0,
            ir::InsOpbin::Mul => src1.0 * src2.0,
            ir::InsOpbin::Div => src1.0 / src2.0,
            ir::InsOpbin::Rem => src1.0 % src2.0,
        };

        self.push_op(RTVal(res));
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
        if fun.is_extern() {
            self.call_native(ins.fun(), args);
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

    fn call_native(&mut self, fun: ir::FunctionRef, args: Vec<RTVal>) {
        let fun_putc = ir::FunctionRef::new(257);
        let fun_exit = ir::FunctionRef::new(258);

        if fun == fun_putc {
            self.call_native_putc(args);
        } else if fun == fun_exit {
            self.call_native_exit(args);
        } else {
            panic!("Failed to called extern function: unknown id {}", fun);
        }
    }

    fn call_native_putc(&mut self, args: Vec<RTVal>) {
        if args.len() != 1 {
            panic!(
                "Failed to call putc: expected 1 argument, got {}",
                args.len()
            )
        }

        let bval = args[0].0 as u8;
        self.stdout.push(bval);
        self.push_op(RTVal(0));
    }

    fn call_native_exit(&mut self, args: Vec<RTVal>) {
        if args.len() != 1 {
            panic!(
                "Failed to call exit: expected 1 argument, got {}",
                args.len()
            )
        }

        let exit_val = args[0].0 as u8;
        self.ins_status = Some(ExitCode(exit_val));
    }
}
