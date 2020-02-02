use crate::ir;
use crate::irvalidation;

/// An IRBuilder is linked to a function, and can manipulate basic blocks / instructions
/// in a more convenient way that by just modifying the Function
/// TODO: might be possible to directly hold a reference to a basicblock using unsafe code or some trick
/// That way I could avoid the lookup every time.
pub struct IRBuilder<'a> {
    fun: &'a mut ir::Function,
    bb_id: Option<ir::BasicBlockId>,
}

impl<'a> IRBuilder<'a> {
    pub fn new(fun: &'a mut ir::Function) -> Self {
        IRBuilder { fun, bb_id: None }
    }

    /// Returns the function id the IRBuilder is linked to
    pub fn actual_function(&self) -> ir::FunctionId {
        self.fun.id()
    }

    /// Returns the basic block id of the actual insert point
    pub fn get_insert_point(&self) -> Option<ir::BasicBlockId> {
        self.bb_id
    }

    /// Change the insert point to another basic block
    pub fn set_insert_point(&mut self, bb: ir::BasicBlockId) {
        assert!(self.fun.get_basic_block(bb).id() == bb);
        self.bb_id = Some(bb);
    }

    /// Remove the current insert point
    pub fn reset_insert_point(&mut self) {
        self.bb_id = None;
    }

    ///Create a basic block at the end of the function
    pub fn create_basic_block(&mut self) -> ir::BasicBlockId {
        self.fun.create_basic_block()
    }

    /// run the validation on the current function
    /// panic is the function is invalid
    pub fn validate(&self) {
        irvalidation::validate_function(self.fun, None);
    }

    /// Append an instruction at the insert point
    /// Usually this function shouldn't be called, it's better to use ins_* to build instructions
    pub fn append_ins(&mut self, ins: ir::Ins) {
        let id = self
            .bb_id
            .expect("Cannot insert instruction: no insert point set");
        let bb = self.fun.get_basic_block_mut(id);
        bb.push_ins(ins);
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

    pub fn ins_call(&mut self, dst: ir::RegId, fun: ir::FunctionId, args: Vec<ir::RegId>) {
        self.append_ins(ir::Ins::Call(ir::InsCall::new(dst, fun, args)));
    }

    pub fn ins_ret(&mut self, src: ir::RegId) {
        self.append_ins(ir::Ins::Ret(ir::InsRet::new(src)));
    }
}
