use std::collections::HashMap;

use irint3a::ir;

#[derive(Clone, Copy, Debug)]
struct RTVal(i32);

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
    fun: ir::FunAddress,
    pos: ir::LocalLabel,
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

pub struct Runtime {
    code: ir::Module,
    frames: Vec<Frame>,
    call_stack: Vec<CodeAddress>,
}

impl Runtime {
    /// Create a new initialized runtime
    pub fn new(code: ir::Module) -> Self {
        let mut res = Runtime {
            code,
            frames: vec![],
            call_stack: vec![],
        };
        res.reset();
        res
    }

    /// Reset the Runtime to the starting point of the program
    pub fn reset(&mut self) {
        self.frames.clear();
        self.call_stack.clear();

        self.call_stack.push(CodeAddress {
            fun: ir::FunAddress(0),
            pos: ir::LocalLabel(0),
        });
        self.frames.push(Frame::new());
    }

    /// Run only one instruction
    /// Returns false if the instruction couldnt be run (only happens when program execution finished)
    pub fn step(&mut self) -> bool {
        if self.call_stack.len() == 0 {
            return false;
        }

        let ins = self.fetch_ins().clone();
        self.exec_ins(ins);
        true
    }

    /// Run the program until the end
    pub fn run(&mut self) {
        loop {
            if !self.step() {
                break;
            }
        }
    }

    fn get_ins(&self, addr: &CodeAddress) -> &ir::Ins {
        self.code
            .defs()
            .get(addr.fun.0)
            .expect("Failed to get instruction: invalid function address")
            .body()
            .expect("Failed to get instruction: function is extern")
            .get(addr.pos.0)
            .expect("Failed to get instruction: invalid position address")
    }

    // Return the current instruction, doesn't move the pc
    fn fetch_ins(&self) -> &ir::Ins {
        self.get_ins(self.call_stack.last().unwrap())
    }

    // Simply go to the following instruction in the code (doesn't do any branch / call)
    fn next_ins(&mut self) {
        let addr = self.call_stack.last_mut().unwrap();
        addr.pos.0 += 1;
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

    fn exec_ins(&mut self, ins: ir::Ins) {}

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
    }

    fn exec_ins_store(&mut self, ins: ir::InsStore) {
        let dst_addr = MemAddress(self.get_reg(ins.dst()));
        self.store(&dst_addr, self.get_reg(ins.src()));
        self.next_ins();
    }

    fn exec_ins_alloca(&mut self, ins: ir::InsAlloca) {
        let frame_idx = self.frames.len() - 1;
        let local_idx = self.frames.last_mut().unwrap().alloca();
        let addr = MemAddress::new(frame_idx, local_idx);
        self.set_reg(ins.dst(), addr.0);
        self.next_ins();
    }

    fn exec_ins_opbin(&mut self, ins: ir::InsOpbin) {
        let src1 = self.get_reg(ins.src1()).0;
        let src2 = self.get_reg(ins.src2()).0;

        let res = match ins.kind() {
            ir::InsOpbinKind::Add => src1 + src2,
            ir::InsOpbinKind::Sub => src1 - src2,
            ir::InsOpbinKind::Mul => src1 * src2,
            ir::InsOpbinKind::Div => src1 / src2,
            ir::InsOpbinKind::Mod => src1 % src2,
        };

        self.set_reg(ins.dst(), RTVal(res));
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
        self.call_stack.last_mut().unwrap().pos = ins.label();
    }

    fn exec_ins_br(&mut self, ins: ir::InsBr) {
        let cond_val = self.get_reg(ins.src());
        let next_label = if cond_val.0 != 0 {
            ins.label_true()
        } else {
            ins.label_false()
        };
        self.call_stack.last_mut().unwrap().pos = next_label;
    }

    fn exec_ins_call(&mut self, ins: ir::InsCall) {
        //TODO: extern functions
        let args = ins.args().iter().map(|x| self.get_reg(*x)).collect();
        self.frames.push(Frame::new_from_call(&args, ins.dst()));
        self.call_stack.push(CodeAddress {
            fun: ins.fun(),
            pos: ir::LocalLabel(0),
        });
    }

    fn exec_ins_ret(&mut self, ins: ir::InsRet) {
        let ret_val = self.get_reg(ins.src());
        let ret_reg = self.frames.last().unwrap().get_ret_reg();
        self.frames.pop();
        self.call_stack.pop();
        self.next_ins();
        self.set_reg(ret_reg, ret_val);
    }
}
