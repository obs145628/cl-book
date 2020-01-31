use std::collections::HashMap;
use std::fmt;

// Basic rules about the IR
//
// There is no registers, we manipulate data using directly only in the stack frame, in 2 ways:
// - We can load / store local variables
// - There is an operands stack, where we can push values.
// Instructions pop values from the stack and push result on the stack
//
// Datatypes:
// There is only one datatype manipulated by the instructions:
// - 32 bits signed integer
//
// Function call / return:
// all function arguments must be put on the operands stack
// The callee puts the return value of top of the operands stack
// The caller receives return value on top of the operands stack
// Each function has it's own stack frame,
// so the local variables and stack operands remain unchanged
// (expect for the arguments and return value)
//
// Basic blocks:
// Each function code is a sequence of basic blocks
// Each basic block is a sequence of instructions with a few rules;
// - the last instruction must be a control flow instruction (br, jump, or ret)
// - there must not any other control flow instruction
// - br and jump can only branch to an instruction at the beginning of a basic block

/// Represent a simple IR instruction
#[derive(Clone, Copy, Debug)]
pub enum Ins {
    Pop(InsPop),
    Const(InsConst),
    Load(InsLoad),
    Store(InsStore),
    Opbin(InsOpbin),   //add, sub, mov, div, mod
    Cmpbin(InsCmpbin), //cmpeq, cmplt, cmpgt
    Jump(InsJump),
    Br(InsBr),
    Call(InsCall),
    Ret(InsRet),
}

impl Ins {
    pub fn is_control_flow(&self) -> bool {
        match self {
            Ins::Jump(_) | Ins::Br(_) | Ins::Ret(_) => true,
            _ => false,
        }
    }
}

/// Trait used to compute the effects of instructions on the Operands Stack
pub trait OperandsSizeEffect {
    fn operands_size_change(&self) -> i32;
}

impl OperandsSizeEffect for Ins {
    fn operands_size_change(&self) -> i32 {
        match self {
            Ins::Pop(ins) => ins.operands_size_change(),
            Ins::Const(ins) => ins.operands_size_change(),
            Ins::Load(ins) => ins.operands_size_change(),
            Ins::Store(ins) => ins.operands_size_change(),
            Ins::Opbin(ins) => ins.operands_size_change(),
            Ins::Cmpbin(ins) => ins.operands_size_change(),
            Ins::Jump(ins) => ins.operands_size_change(),
            Ins::Br(ins) => ins.operands_size_change(),
            Ins::Call(ins) => ins.operands_size_change(),
            Ins::Ret(ins) => ins.operands_size_change(),
        }
    }
}

/// In the IR, Functions are identified by an unique usize (in the Module)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FunctionRef(usize);

impl FunctionRef {
    pub fn new(id: usize) -> Self {
        FunctionRef(id)
    }
}

impl fmt::Display for FunctionRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// In the IR, BasicBlocks are identfied by an unique usize (in the Module)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BasicBlockRef(usize);

impl BasicBlockRef {
    pub fn new(id: usize) -> Self {
        BasicBlockRef(id)
    }
}

impl fmt::Display for BasicBlockRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// In the IR, accessing the locals variables is done through an usize index
/// Hardcoded index, cannot be dynamic
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LocalsIndex(usize);

impl LocalsIndex {
    pub fn new(id: usize) -> Self {
        LocalsIndex(id)
    }
}

impl fmt::Display for LocalsIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Instruction pop
/// Discard value on top of the operands stack
/// pop
#[derive(Clone, Copy, Debug)]
pub struct InsPop {}

impl InsPop {
    pub fn new() -> Self {
        InsPop {}
    }
}

impl OperandsSizeEffect for InsPop {
    fn operands_size_change(&self) -> i32 {
        -1
    }
}

/// Instruction const
/// Push a constant on the operands stack
/// const <i32-const>
#[derive(Clone, Copy, Debug)]
pub struct InsConst {
    val: i32,
}

impl InsConst {
    pub fn new(val: i32) -> Self {
        InsConst { val }
    }

    pub fn val(&self) -> i32 {
        self.val
    }
}

impl OperandsSizeEffect for InsConst {
    fn operands_size_change(&self) -> i32 {
        1
    }
}

/// Instruction load
/// Push a local variable to the operands stack
/// load <src-local-index>
#[derive(Clone, Copy, Debug)]
pub struct InsLoad {
    src: LocalsIndex,
}

impl InsLoad {
    pub fn new(src: LocalsIndex) -> Self {
        InsLoad { src }
    }

    pub fn src(&self) -> LocalsIndex {
        self.src
    }
}

impl OperandsSizeEffect for InsLoad {
    fn operands_size_change(&self) -> i32 {
        1
    }
}

/// Instruction store
/// Pop value from the operand stacks and store it to a local
/// store <dst-local-index>
#[derive(Clone, Copy, Debug)]
pub struct InsStore {
    dst: LocalsIndex,
}

impl InsStore {
    pub fn new(dst: LocalsIndex) -> Self {
        InsStore { dst }
    }

    pub fn dst(&self) -> LocalsIndex {
        self.dst
    }
}

impl OperandsSizeEffect for InsStore {
    fn operands_size_change(&self) -> i32 {
        -1
    }
}

/// Represent multiple instructions for binary operations
/// Pop the 2 int32 argument values from the operands stack
/// Push the results to the operands stack
/// It pops the right operand first, and then the left
/// It means you must push the left operand then the right operand
/// <ins>
/// <ins> may be any of add, sub, mul, div, rem
#[derive(Clone, Copy, Debug)]
pub enum InsOpbin {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

impl OperandsSizeEffect for InsOpbin {
    fn operands_size_change(&self) -> i32 {
        -1
    }
}

/// Represent multiple instructions for binary comparisons
/// Pop the 2 int32 argument values from the operands stack
/// Push the results to the operands stack
/// Result is int32(1) if cmp is ture, int32(0) otherwhise
/// It pops the right operand first, and then the left
/// It means you must push the left operand then the right operand
/// <ins>
/// <ins> may be any of cmpeq, cmplt, cmpgt
#[derive(Clone, Copy, Debug)]
pub enum InsCmpbin {
    Eq,
    Lt,
    Gt,
}

impl OperandsSizeEffect for InsCmpbin {
    fn operands_size_change(&self) -> i32 {
        -1
    }
}

/// Instruction jump
/// Unconditional jump to the beginning of a basic block in the same function
/// jump %<dst-bb-name>
#[derive(Clone, Copy, Debug)]
pub struct InsJump {
    dst: BasicBlockRef,
}

impl InsJump {
    pub fn new(dst: BasicBlockRef) -> Self {
        InsJump { dst }
    }

    pub fn dst(&self) -> BasicBlockRef {
        self.dst
    }
}

impl OperandsSizeEffect for InsJump {
    fn operands_size_change(&self) -> i32 {
        0
    }
}

/// Instruction br
/// Jump conditionaly based on the value poped from the operands stack
/// If value != 0, jump to dst-true, otherwhise to dst-false
/// br %<dst-true-bb-name>, %<dst-false-bb-name>
#[derive(Clone, Copy, Debug)]
pub struct InsBr {
    dst_true: BasicBlockRef,
    dst_false: BasicBlockRef,
}

impl InsBr {
    pub fn new(dst_true: BasicBlockRef, dst_false: BasicBlockRef) -> Self {
        InsBr {
            dst_true,
            dst_false,
        }
    }

    pub fn dst_true(&self) -> BasicBlockRef {
        self.dst_true
    }

    pub fn dst_false(&self) -> BasicBlockRef {
        self.dst_false
    }
}

impl OperandsSizeEffect for InsBr {
    fn operands_size_change(&self) -> i32 {
        -1
    }
}

/// Instruction call
/// Pop arguments from the operands stack and call a function
/// Return value pushed to the operands stack
/// Arguments poped in the opposite order
/// It means arguments must be push in the right order
/// call %<fun>, <nb-args>
#[derive(Clone, Copy, Debug)]
pub struct InsCall {
    fun: FunctionRef,
    nb_args: usize,
}

impl InsCall {
    pub fn new(fun: FunctionRef, nb_args: usize) -> Self {
        InsCall { fun, nb_args }
    }

    pub fn fun(&self) -> FunctionRef {
        self.fun
    }

    pub fn nb_args(&self) -> usize {
        self.nb_args
    }
}

impl OperandsSizeEffect for InsCall {
    fn operands_size_change(&self) -> i32 {
        1 - (self.nb_args() as i32)
    }
}

/// Instruction ret
/// Return from the current function
/// The return value is poped from the operands stack
/// ret
#[derive(Clone, Copy, Debug)]
pub struct InsRet {}

impl InsRet {
    pub fn new() -> Self {
        InsRet {}
    }
}

impl OperandsSizeEffect for InsRet {
    fn operands_size_change(&self) -> i32 {
        -1
    }
}

/// Representation of a basic block for the IR
/// This a a simple data holder struct, it doesn't ensure any of the rules
#[derive(Debug)]
pub struct BasicBlock {
    id: BasicBlockRef,
    ins_list: Vec<Ins>,
}

impl BasicBlock {
    pub fn new(id: BasicBlockRef, ins_list: Vec<Ins>) -> Self {
        BasicBlock { id, ins_list }
    }

    pub fn id(&self) -> BasicBlockRef {
        self.id
    }

    pub fn ins_list(&self) -> &[Ins] {
        &self.ins_list
    }
}

#[derive(Debug)]
/// Represensation of a function for the IR
/// This a a simple data holder struct, it doesn't ensure any of the rules
/// A funcion without bb_list is an extern declaration
pub struct Function {
    id: FunctionRef,
    bb_list: Option<Vec<BasicBlock>>,
    bb_mapping: HashMap<BasicBlockRef, usize>,
}

impl Function {
    pub fn new(id: FunctionRef, bb_list: Option<Vec<BasicBlock>>) -> Self {
        let mut res = Function {
            id,
            bb_list,
            bb_mapping: HashMap::new(),
        };

        if let Some(bb_list) = &res.bb_list {
            for (bb_idx, bb) in bb_list.iter().enumerate() {
                res.bb_mapping.insert(bb.id(), bb_idx);
            }
        }

        res
    }

    pub fn id(&self) -> FunctionRef {
        self.id
    }

    pub fn bb_list(&self) -> &[BasicBlock] {
        self.bb_list.as_ref().unwrap()
    }

    pub fn is_extern(&self) -> bool {
        self.bb_list.is_none()
    }

    pub fn get_bb(&self, id: BasicBlockRef) -> &BasicBlock {
        let idx = *self.bb_mapping.get(&id).unwrap();
        &self.bb_list.as_ref().unwrap()[idx]
    }
}

pub struct Module {
    fun_list: Vec<Function>,
    fun_mapping: HashMap<FunctionRef, usize>,
}

/// Represensation the defition of the whole module for the IR
/// This a a simple data holder struct, it doesn't ensure any of the rules
impl Module {
    pub fn new(fun_list: Vec<Function>) -> Self {
        let mut res = Module {
            fun_list,
            fun_mapping: HashMap::new(),
        };
        for (fun_idx, fun) in res.fun_list.iter().enumerate() {
            res.fun_mapping.insert(fun.id(), fun_idx);
        }

        res
    }

    pub fn fun_list(&self) -> &[Function] {
        &self.fun_list
    }

    pub fn get_fun(&self, id: FunctionRef) -> &Function {
        let idx = *self.fun_mapping.get(&id).unwrap();
        &self.fun_list[idx]
    }
}
