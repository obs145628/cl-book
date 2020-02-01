use crate::ir;

/*
use std::collections::HashMap;

struct UnresolvedJump {
    label_name: String,
}

impl UnresolvedJump {
    fn resolve(self, local_labels: &HashMap<String, ir::LocalLabel>) -> ir::Ins {
        match local_labels.get(&self.label_name) {
            Some(label) => ir::Ins::Jump(ir::InsJump::new(*label)),
            None => panic!("Cannot resolve jump: label {} undefined", self.label_name),
        }
    }
}

struct UnresolvedBr {
    ins: ir::InsBr,
    true_name: String,
    false_name: String,
}

impl UnresolvedBr {
    fn resolve(self, local_labels: &HashMap<String, ir::LocalLabel>) -> ir::Ins {
        let true_label = match local_labels.get(&self.true_name) {
            Some(label) => *label,
            None => panic!("Cannot resolve br: label {} undefined", self.true_name),
        };
        let false_label = match local_labels.get(&self.false_name) {
            Some(label) => *label,
            None => panic!("Cannot resolve br: label {} undefined", self.false_name),
        };
        ir::Ins::Br(ir::InsBr::new(self.ins.src(), true_label, false_label))
    }
}

struct UnresolvedCall {
    ins: ir::InsCall,
    fn_name: String,
}

impl UnresolvedCall {
    fn resolve(self, fun_names: &HashMap<String, ir::FunAddress>) -> ir::Ins {
        let fn_addr = match fun_names.get(&self.fn_name) {
            Some(addr) => *addr,
            None => panic!("Cannot resolve call: function {} undefined", self.fn_name),
        };
        ir::Ins::Call(ir::InsCall::new(
            self.ins.dst(),
            fn_addr,
            self.ins.args().clone(),
        ))
    }
}

enum UnresolvedIns {
    Resolved(ir::Ins),
    Jump(UnresolvedJump),
    Br(UnresolvedBr),
    Call(UnresolvedCall),
}

impl UnresolvedIns {
    fn resolve(
        self,
        local_labels: &HashMap<String, ir::LocalLabel>,
        fun_names: &HashMap<String, ir::FunAddress>,
    ) -> ir::Ins {
        match self {
            UnresolvedIns::Resolved(ins) => ins,
            UnresolvedIns::Jump(ins) => ins.resolve(local_labels),
            UnresolvedIns::Br(ins) => ins.resolve(local_labels),
            UnresolvedIns::Call(ins) => ins.resolve(fun_names),
        }
    }
}

struct FunBuilder {
    addr: ir::FunAddress,
    code: Vec<UnresolvedIns>,
    labels: HashMap<String, ir::LocalLabel>,
}

impl FunBuilder {
    fn new(addr: ir::FunAddress) -> Self {
        FunBuilder {
            addr,
            code: vec![],
            labels: HashMap::new(),
        }
    }

    fn get_last_pos(&self) -> ir::LocalLabel {
        assert!(self.code.len() > 0);
        ir::LocalLabel(self.code.len() - 1)
    }

    fn get_next_pos(&self) -> ir::LocalLabel {
        ir::LocalLabel(self.code.len())
    }

    fn add_ins(&mut self, ins: UnresolvedIns, label: Option<&str>) {
        if let Some(label_name) = label {
            let label = ir::LocalLabel(self.code.len());
            self.labels.insert(label_name.to_string(), label);
        }

        self.code.push(ins)
    }

    fn build(mut self, fun_names: &HashMap<String, ir::FunAddress>) -> ir::DefFun {
        let mut code = vec![];
        std::mem::swap(&mut code, &mut self.code);

        let code = code
            .into_iter()
            .map(|ins| ins.resolve(&self.labels, fun_names))
            .collect();
        ir::DefFun::new(self.addr, Some(code))
    }
}
*/

pub struct IRBuilder<'a> {
    bb: &'a mut ir::BasicBlock,
}

impl<'a> IRBuilder<'a> {
    pub fn new(module: &'a mut ir::Module, bb_id: ir::BasicBlockId) -> Self {
        let fun_id = module.get_basic_block_owner(bb_id);
        let fun = module.get_fun_mut(fun_id).unwrap();
        let bb = fun.get_basic_block_mut(bb_id);
        IRBuilder { bb }
    }

    /// Returns the basic block id of the actual insert point
    pub fn actual_basic_block(&self) -> ir::BasicBlockId {
        self.bb.id()
    }

    /// Returns the function id of the actual insert point
    pub fn actual_function(&self) -> ir::FunctionId {
        self.bb.fun_id()
    }

    /// Append an instruction at the insert point
    /// Usually this function shouldn't be called, it's better to use ins_* to build instructions
    pub fn append_ins(&mut self, ins: ir::Ins) {
        self.bb.push_ins(ins);
    }

    pub fn ins_movi(&mut self, dst: ir::RegId, const_val: i32) {
        self.append_ins(ir::Ins::Movi(ir::InsMovi::new(dst, const_val)));
    }

    pub fn ins_movr(&mut self, dst: ir::RegId, src: ir::RegId) {
        self.append_ins(ir::Ins::Movr(ir::InsMovr::new(dst, src)));
    }

    pub fn ins_load(&mut self, dst: ir::RegId, src: ir::RegId) {
        self.append_ins(ir::Ins::Load(ir::InsLoad::new(dst, src)));
    }

    pub fn ins_store(&mut self, dst: ir::RegId, src: ir::RegId) {
        self.append_ins(ir::Ins::Store(ir::InsStore::new(dst, src)));
    }

    pub fn ins_alloca(&mut self, dst: ir::RegId) {
        self.append_ins(ir::Ins::Alloca(ir::InsAlloca::new(dst)));
    }

    pub fn ins_add(&mut self, dst: ir::RegId, src1: ir::RegId, src2: ir::RegId) {
        self.append_ins(ir::Ins::Opbin(ir::InsOpbin::new(
            ir::InsOpbinKind::Add,
            dst,
            src1,
            src2,
        )));
    }

    pub fn ins_sub(&mut self, dst: ir::RegId, src1: ir::RegId, src2: ir::RegId) {
        self.append_ins(ir::Ins::Opbin(ir::InsOpbin::new(
            ir::InsOpbinKind::Sub,
            dst,
            src1,
            src2,
        )));
    }

    pub fn ins_mul(&mut self, dst: ir::RegId, src1: ir::RegId, src2: ir::RegId) {
        self.append_ins(ir::Ins::Opbin(ir::InsOpbin::new(
            ir::InsOpbinKind::Mul,
            dst,
            src1,
            src2,
        )));
    }

    pub fn ins_div(&mut self, dst: ir::RegId, src1: ir::RegId, src2: ir::RegId) {
        self.append_ins(ir::Ins::Opbin(ir::InsOpbin::new(
            ir::InsOpbinKind::Div,
            dst,
            src1,
            src2,
        )));
    }

    pub fn ins_mod(&mut self, dst: ir::RegId, src1: ir::RegId, src2: ir::RegId) {
        self.append_ins(ir::Ins::Opbin(ir::InsOpbin::new(
            ir::InsOpbinKind::Mod,
            dst,
            src1,
            src2,
        )));
    }

    pub fn ins_cmpeq(&mut self, dst: ir::RegId, src1: ir::RegId, src2: ir::RegId) {
        self.append_ins(ir::Ins::Cmpbin(ir::InsCmpbin::new(
            ir::InsCmpbinKind::Eq,
            dst,
            src1,
            src2,
        )));
    }

    pub fn ins_cmplt(&mut self, dst: ir::RegId, src1: ir::RegId, src2: ir::RegId) {
        self.append_ins(ir::Ins::Cmpbin(ir::InsCmpbin::new(
            ir::InsCmpbinKind::Lt,
            dst,
            src1,
            src2,
        )));
    }

    pub fn ins_cmpgt(&mut self, dst: ir::RegId, src1: ir::RegId, src2: ir::RegId) {
        self.append_ins(ir::Ins::Cmpbin(ir::InsCmpbin::new(
            ir::InsCmpbinKind::Gt,
            dst,
            src1,
            src2,
        )));
    }

    pub fn ins_jump(&mut self, dst: ir::BasicBlockId) {
        self.append_ins(ir::Ins::Jump(ir::InsJump::new(dst)));
    }

    pub fn ins_br(
        &mut self,
        src: ir::RegId,
        dst_true: ir::BasicBlockId,
        dst_false: ir::BasicBlockId,
    ) {
        self.append_ins(ir::Ins::Br(ir::InsBr::new(src, dst_true, dst_false)));
    }

    pub fn ins_call_addr(&mut self, dst: ir::RegId, fun: ir::FunctionId, args: Vec<ir::RegId>) {
        self.append_ins(ir::Ins::Call(ir::InsCall::new(dst, fun, args)));
    }

    pub fn ins_ret(&mut self, src: ir::RegId) {
        self.append_ins(ir::Ins::Ret(ir::InsRet::new(src)));
    }
}
