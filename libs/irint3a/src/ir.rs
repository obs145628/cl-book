use std::collections::HashMap;
use std::collections::HashSet;

// Basic rules about the IR
//
// Registers:
// All egistrs are 32 bits
// There is an infinite number of registers
// Register names are %0, %1, %2, etc
//
// Datatypes:
// The IR is not typed, but basically instructions work on 2 types of data:
// - 32 bits signed integer
// - 32 bits memory address
//
// Function call / return:
// all function arguments are 32 bits, and stored in register 0, 1, 2, etc
// The notion of stack frame / calling convention is abstracted away and not present in the IR
// There is no special register to pass arguments, or get return value: it's all chosen with call instruction
// There is no special register to set return value: it's all chosen with ret instruction
// All the registers remain unchanged after returning from a function call (even if callee change some registers)
//
// Labels and Branching:
// This only concern IR file syntax
// We can give a label to any instruction, by preceding it with <labelname> :. labelname is any identifier
// the jump / br instructions take this identifier to locale the right label then instruction
//
// Function reference:
// This only concern IR file syntax
// To refer to the function with the call instruction, we simply use it's function identifier

/// Represent a simple IR instruction
pub enum Ins {
    Movi(InsMovi),
    Movr(InsMovr),
    Load(InsLoad),
    Store(InsStore),
    Alloca(InsAlloca),
    Opbin(InsOpbin),   //add, sub, mov, div, mod
    Cmpbin(InsCmpbin), //cmpeq, cmplt, cmpgt
    Jump(InsJump),
    Br(InsBr),
    Call(InsCall),
    Ret(InsRet),
}

/// In the IR, registers are represented by a unique usize identifier
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct RegId(pub usize);

/// Represent a label to an instruction in the current function
/// Used for the br jump instructions, to refer to where we want to jump
/// It's an index in the list of instructions of the function
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LocalLabel(pub usize);

/// Represent an address to a local or extern function
/// Its actually the unique identifier of the function
/// This address is always known (no resolving needed later)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FunAddress(pub usize);

/// Instruction movi
/// Move constant integer into a register
/// movi <dstreg>, <constint>
#[derive(Clone, Copy, Debug)]
pub struct InsMovi {
    dst: RegId,
    const_val: i32,
}

impl InsMovi {
    pub fn new(dst: RegId, const_val: i32) -> Self {
        InsMovi { dst, const_val }
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }

    pub fn const_val(&self) -> i32 {
        self.const_val
    }
}

/// Instruction movr
/// Copy value of one register to another
/// movr <dstreg>, <srcreg>
#[derive(Clone, Copy, Debug)]
pub struct InsMovr {
    dst: RegId,
    src: RegId,
}

impl InsMovr {
    pub fn new(dst: RegId, src: RegId) -> Self {
        InsMovr { dst, src }
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }

    pub fn src(&self) -> RegId {
        self.src
    }
}

/// Instruction load
/// Read the 32bit address stored in src register, and load the value at that address to the register dst
/// load <dstreg>, <srcreg>
#[derive(Clone, Copy, Debug)]
pub struct InsLoad {
    dst: RegId,
    src: RegId,
}

impl InsLoad {
    pub fn new(dst: RegId, src: RegId) -> Self {
        InsLoad { dst, src }
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }

    pub fn src(&self) -> RegId {
        self.src
    }
}

/// Instruction store
/// Read the 32bit adress stored in dst register, and store the content of src register at that address
/// store <dstreg>, <srcreg>
#[derive(Clone, Copy, Debug)]
pub struct InsStore {
    dst: RegId,
    src: RegId,
}

impl InsStore {
    pub fn new(dst: RegId, src: RegId) -> Self {
        InsStore { dst, src }
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }

    pub fn src(&self) -> RegId {
        self.src
    }
}

/// Instruction Alloca
/// Allocate a variable local to the function, and write it's memory address in dst register
/// alloca <dstreg>
#[derive(Clone, Copy, Debug)]
pub struct InsAlloca {
    dst: RegId,
}

impl InsAlloca {
    pub fn new(dst: RegId) -> Self {
        InsAlloca { dst }
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }
}

#[derive(Clone, Copy, Debug)]
pub enum InsOpbinKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

/// Represent multiple instructions for binary operations
/// Read int32 values from both src registers, compute and store result in dst register
/// <ins> <dstreg>, <src1reg>, <src2reg>
/// <ins> may be any of add, sub, mul, div, mod
#[derive(Clone, Copy, Debug)]
pub struct InsOpbin {
    kind: InsOpbinKind,
    dst: RegId,
    src1: RegId,
    src2: RegId,
}

impl InsOpbin {
    pub fn new(kind: InsOpbinKind, dst: RegId, src1: RegId, src2: RegId) -> Self {
        InsOpbin {
            kind,
            dst,
            src1,
            src2,
        }
    }

    pub fn kind(&self) -> InsOpbinKind {
        self.kind
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }

    pub fn src1(&self) -> RegId {
        self.src1
    }

    pub fn scr2(&self) -> RegId {
        self.src2
    }
}

#[derive(Clone, Copy, Debug)]
pub enum InsCmpbinKind {
    Eq,
    Lt,
    Gt,
}

/// Represent multiple instructions for binary comparisons
/// Read int32 values from both src registers, compute and store result in dst register: 1 is cmp is true, 0 if false
/// <ins> <dstreg>, <src1reg>, <src2reg>
/// <ins> may be any of cmpeq, cmplt, cmpgt
#[derive(Clone, Copy, Debug)]
pub struct InsCmpbin {
    kind: InsCmpbinKind,
    dst: RegId,
    src1: RegId,
    src2: RegId,
}

impl InsCmpbin {
    pub fn new(kind: InsCmpbinKind, dst: RegId, src1: RegId, src2: RegId) -> Self {
        InsCmpbin {
            kind,
            dst,
            src1,
            src2,
        }
    }

    pub fn kind(&self) -> InsCmpbinKind {
        self.kind
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }

    pub fn src1(&self) -> RegId {
        self.src1
    }

    pub fn scr2(&self) -> RegId {
        self.src2
    }
}

/// Instruction jump
/// Unconditional jump to some other part of the function code
/// jump <label>
#[derive(Clone, Copy, Debug)]
pub struct InsJump {
    label: LocalLabel,
}

impl InsJump {
    pub fn new(label: LocalLabel) -> Self {
        InsJump { label }
    }

    pub fn label(&self) -> LocalLabel {
        self.label
    }
}

/// Instruction Br
/// Conditional jump depending on value of register src
/// If value != 0, jump to labeltrue, otherwhise to labelfalse
/// br <regsrc>, <labeltrue>, <labelfalse>
#[derive(Clone, Copy, Debug)]
pub struct InsBr {
    src: RegId,
    label_true: LocalLabel,
    label_false: LocalLabel,
}

impl InsBr {
    pub fn new(src: RegId, label_true: LocalLabel, label_false: LocalLabel) -> Self {
        InsBr {
            src,
            label_true,
            label_false,
        }
    }

    pub fn src(&self) -> RegId {
        self.src
    }

    pub fn label_true(&self) -> LocalLabel {
        self.label_true
    }

    pub fn label_false(&self) -> LocalLabel {
        self.label_false
    }
}

/// Instruction call
/// Call specified function with arguments stored in given args registers, and store return value in dst register
/// call <dstreg>, <fun>, <arg1reg>, <arg2reg>, ...
#[derive(Clone, Debug)]
pub struct InsCall {
    dst: RegId,
    fun: FunAddress,
    args: Vec<RegId>,
}

impl InsCall {
    pub fn new(dst: RegId, fun: FunAddress, args: Vec<RegId>) -> Self {
        InsCall { dst, fun, args }
    }

    pub fn dst(&self) -> RegId {
        self.dst
    }

    pub fn fun(&self) -> FunAddress {
        self.fun
    }

    pub fn args(&self) -> &Vec<RegId> {
        &self.args
    }
}

/// Instruction ret
/// Stop the current function, and return the value in the register src
/// ret <srcreg>
#[derive(Clone, Copy, Debug)]
pub struct InsRet {
    src: RegId,
}

impl InsRet {
    pub fn new(src: RegId) -> Self {
        InsRet { src }
    }

    pub fn src(&self) -> RegId {
        self.src
    }
}

/// Function Definition
/// There is no arguments numbers, all passed arguments are simply stored in registers
/// Functions with no body are extern, the code might be resolved later
/// They have an unisgned unique identifier
///
/// Writing a local function syntax is:
/// define <addr> <name> <body>
/// For extern functions syntax is:
/// define <addr> <name> extern
pub struct DefFun {
    addr: FunAddress,
    body: Option<Vec<Ins>>, //if no body, function is extern
}

impl DefFun {
    pub fn new(addr: FunAddress, body: Option<Vec<Ins>>) -> Self {
        DefFun { addr, body }
    }

    pub fn addr(&self) -> FunAddress {
        self.addr
    }

    pub fn body(&self) -> Option<&Vec<Ins>> {
        self.body.as_ref()
    }
}

/// A module represent the whole definition of an IR file
/// It's simply a list of functions.
/// A module must have a function wirh id 0, this is the entry point
pub struct Module {
    defs: Vec<DefFun>,
}

impl Module {
    pub fn new(defs: Vec<DefFun>) -> Self {
        Module { defs }
    }

    pub fn defs(&self) -> &Vec<DefFun> {
        &self.defs
    }
}

/// Module with extra metadedata
/// This are fur debug perposes, it contains function names
pub struct ModuleExtended {
    module: Module,
    funs: HashMap<FunAddress, FunExtended>,
}

impl ModuleExtended {
    pub fn new(module: Module, funs: HashMap<FunAddress, FunExtended>) -> Self {
        let mut res = ModuleExtended { module, funs };
        res.fill_funs();

        for def in res.module.defs() {
            let fn_ex = res.funs.get_mut(&def.addr()).unwrap();
            fn_ex.fill(def);
        }

        res
    }

    pub fn module(&self) -> &Module {
        &self.module
    }

    pub fn get_fun(&self, addr: FunAddress) -> &FunExtended {
        self.funs.get(&addr).unwrap()
    }

    fn fill_funs(&mut self) {
        let mut def_idx = 1;
        for def in &self.module.defs {
            if self.funs.get(&def.addr()).is_none() {
                let fn_name = format!("f{}", def_idx);
                def_idx += 1;
                self.funs.insert(
                    def.addr(),
                    FunExtended::new(def.addr(), fn_name, HashMap::new()),
                );
            }
        }
    }
}

pub struct FunExtended {
    addr: FunAddress,
    name: String,
    labels: HashMap<LocalLabel, String>,
}

impl FunExtended {
    pub fn new(addr: FunAddress, name: String, labels: HashMap<LocalLabel, String>) -> Self {
        FunExtended { addr, name, labels }
    }

    pub fn addr(&self) -> FunAddress {
        self.addr
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn label_of(&self, id: LocalLabel) -> Option<&String> {
        self.labels.get(&id)
    }

    // gives generated names (L1, L2, etc) to all instructions with a JUMP points
    fn fill(&mut self, fun: &DefFun) {
        if fun.body().is_none() {
            return;
        }

        let mut need_labels = HashSet::new();

        for ins in fun.body().unwrap() {
            match ins {
                Ins::Jump(ins) => {
                    need_labels.insert(ins.label().0);
                }
                Ins::Br(ins) => {
                    need_labels.insert(ins.label_true().0);
                    need_labels.insert(ins.label_false().0);
                }
                _ => {}
            }
        }

        let mut need_labels: Vec<usize> = need_labels.into_iter().collect();
        need_labels.sort();

        let mut label_idx = 1;
        for ins_idx in need_labels {
            let label_name = format!("L{}", label_idx);
            label_idx += 1;
            self.labels.insert(LocalLabel(ins_idx), label_name);
        }
    }
}
