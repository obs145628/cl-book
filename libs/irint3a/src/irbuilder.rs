use crate::ir;

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

pub struct IRBuilder {
    local_defs: Vec<FunBuilder>,
    extern_defs: Vec<ir::DefFun>,
    defs_named: HashMap<String, ir::FunAddress>,
    current_fun: Option<FunBuilder>,
    next_fun_addr: usize,
    next_fun_name: usize,
}

impl IRBuilder {
    pub fn new() -> Self {
        IRBuilder {
            local_defs: vec![],
            extern_defs: vec![],
            defs_named: HashMap::new(),
            current_fun: None,
            next_fun_addr: 1001,
            next_fun_name: 0,
        }
    }

    /// Add an extern function to the module
    /// * `name` optional, if not set, one is generated
    pub fn add_extern_fun(&mut self, name: Option<&str>, id: ir::FunAddress) {
        let name = match name {
            Some(name) => name.to_string(),
            None => self.gen_fun_name(),
        };

        if self.defs_named.get(&name).is_some() {
            panic!("There is already a function with the name {}", name);
        }
        let def = ir::DefFun::new(id, None);
        self.defs_named.insert(name.to_string(), id);
        self.extern_defs.push(def);
    }

    /// Start the definition of a new function
    /// All features created instructions will be appended to this function body
    /// once the function is complete, call end_function()
    /// * `name` optional, if not set, one is generated;
    /// * `id` optional FunAddress, if not set, one is generated
    pub fn begin_function(&mut self, name: Option<&str>, id: Option<ir::FunAddress>) {
        let name = match name {
            Some(name) => name.to_string(),
            None => self.gen_fun_name(),
        };

        if self.current_fun.is_some() {
            panic!("Cannot begin a function, there is still one unfinished");
        }
        if self.defs_named.get(&name).is_some() {
            panic!("There is already a function with the name {}", name);
        }

        let id = match id {
            Some(id) => id,
            None => self.gen_fun_addr(),
        };

        self.defs_named.insert(name.to_string(), id);
        self.current_fun = Some(FunBuilder::new(id));
    }

    /// Complete the definition of the current function
    /// begin_function() must be called before
    pub fn end_function(&mut self) {
        if self.current_fun.is_none() {
            panic!("There is no unfinished function");
        }

        let mut fun = None;
        std::mem::swap(&mut self.current_fun, &mut fun);
        self.local_defs.push(fun.unwrap());
    }

    /// Completes the module, build and returns a fully functional ModuleExtended
    pub fn build(mut self) -> ir::ModuleExtended {
        if self.current_fun.is_some() {
            panic!("Cannot complete modile, one function is still being defined");
        }

        let mut local_defs = vec![];
        std::mem::swap(&mut local_defs, &mut self.local_defs);
        let local_defs = local_defs
            .into_iter()
            .map(|f| f.build(&self.defs_named))
            .collect();

        let mut defs: Vec<ir::DefFun> = local_defs;
        defs.append(&mut self.extern_defs);
        let module = ir::Module::new(defs);

        let mut funs = HashMap::new();
        for (fun_name, fun_addr) in self.defs_named {
            let labels = HashMap::new();
            funs.insert(fun_addr, ir::FunExtended::new(fun_addr, fun_name, labels));
        }

        ir::ModuleExtended::new(module, funs)
    }

    /// Append an instruction to the current function body
    /// Usually this function shouldn't be called, it's better to use ins_* to build instructions
    pub fn append_ins(&mut self, ins: ir::Ins, label: Option<&str>) {
        self.add_unresolved(UnresolvedIns::Resolved(ins), label);
    }

    pub fn ins_movi(&mut self, dst: ir::RegId, const_val: i32, label: Option<&str>) {
        self.append_ins(ir::Ins::Movi(ir::InsMovi::new(dst, const_val)), label);
    }

    pub fn ins_movr(&mut self, dst: ir::RegId, src: ir::RegId, label: Option<&str>) {
        self.append_ins(ir::Ins::Movr(ir::InsMovr::new(dst, src)), label);
    }

    pub fn ins_load(&mut self, dst: ir::RegId, src: ir::RegId, label: Option<&str>) {
        self.append_ins(ir::Ins::Load(ir::InsLoad::new(dst, src)), label);
    }

    pub fn ins_store(&mut self, dst: ir::RegId, src: ir::RegId, label: Option<&str>) {
        self.append_ins(ir::Ins::Store(ir::InsStore::new(dst, src)), label);
    }

    pub fn ins_alloca(&mut self, dst: ir::RegId, label: Option<&str>) {
        self.append_ins(ir::Ins::Alloca(ir::InsAlloca::new(dst)), label);
    }

    pub fn ins_add(
        &mut self,
        dst: ir::RegId,
        src1: ir::RegId,
        src2: ir::RegId,
        label: Option<&str>,
    ) {
        self.append_ins(
            ir::Ins::Opbin(ir::InsOpbin::new(ir::InsOpbinKind::Add, dst, src1, src2)),
            label,
        );
    }

    pub fn ins_sub(
        &mut self,
        dst: ir::RegId,
        src1: ir::RegId,
        src2: ir::RegId,
        label: Option<&str>,
    ) {
        self.append_ins(
            ir::Ins::Opbin(ir::InsOpbin::new(ir::InsOpbinKind::Sub, dst, src1, src2)),
            label,
        );
    }

    pub fn ins_mul(
        &mut self,
        dst: ir::RegId,
        src1: ir::RegId,
        src2: ir::RegId,
        label: Option<&str>,
    ) {
        self.append_ins(
            ir::Ins::Opbin(ir::InsOpbin::new(ir::InsOpbinKind::Mul, dst, src1, src2)),
            label,
        );
    }

    pub fn ins_div(
        &mut self,
        dst: ir::RegId,
        src1: ir::RegId,
        src2: ir::RegId,
        label: Option<&str>,
    ) {
        self.append_ins(
            ir::Ins::Opbin(ir::InsOpbin::new(ir::InsOpbinKind::Div, dst, src1, src2)),
            label,
        );
    }

    pub fn ins_mod(
        &mut self,
        dst: ir::RegId,
        src1: ir::RegId,
        src2: ir::RegId,
        label: Option<&str>,
    ) {
        self.append_ins(
            ir::Ins::Opbin(ir::InsOpbin::new(ir::InsOpbinKind::Mod, dst, src1, src2)),
            label,
        );
    }

    pub fn ins_cmpeq(
        &mut self,
        dst: ir::RegId,
        src1: ir::RegId,
        src2: ir::RegId,
        label: Option<&str>,
    ) {
        self.append_ins(
            ir::Ins::Cmpbin(ir::InsCmpbin::new(ir::InsCmpbinKind::Eq, dst, src1, src2)),
            label,
        );
    }

    pub fn ins_cmplt(
        &mut self,
        dst: ir::RegId,
        src1: ir::RegId,
        src2: ir::RegId,
        label: Option<&str>,
    ) {
        self.append_ins(
            ir::Ins::Cmpbin(ir::InsCmpbin::new(ir::InsCmpbinKind::Lt, dst, src1, src2)),
            label,
        );
    }

    pub fn ins_cmpgt(
        &mut self,
        dst: ir::RegId,
        src1: ir::RegId,
        src2: ir::RegId,
        label: Option<&str>,
    ) {
        self.append_ins(
            ir::Ins::Cmpbin(ir::InsCmpbin::new(ir::InsCmpbinKind::Gt, dst, src1, src2)),
            label,
        );
    }

    pub fn ins_jump(&mut self, jump_label: &str, label: Option<&str>) {
        self.add_unresolved(
            UnresolvedIns::Jump(UnresolvedJump {
                label_name: jump_label.to_string(),
            }),
            label,
        );
    }

    pub fn ins_br(
        &mut self,
        src: ir::RegId,
        label_true: &str,
        label_false: &str,
        label: Option<&str>,
    ) {
        self.add_unresolved(
            UnresolvedIns::Br(UnresolvedBr {
                ins: ir::InsBr::new(src, ir::LocalLabel(0), ir::LocalLabel(0)),
                true_name: label_true.to_string(),
                false_name: label_false.to_string(),
            }),
            label,
        );
    }

    pub fn ins_call_name(
        &mut self,
        dst: ir::RegId,
        fun: &str,
        args: Vec<ir::RegId>,
        label: Option<&str>,
    ) {
        self.add_unresolved(
            UnresolvedIns::Call(UnresolvedCall {
                ins: ir::InsCall::new(dst, ir::FunAddress(0), args),
                fn_name: fun.to_string(),
            }),
            label,
        );
    }

    pub fn ins_call_addr(
        &mut self,
        dst: ir::RegId,
        fun: ir::FunAddress,
        args: Vec<ir::RegId>,
        label: Option<&str>,
    ) {
        self.append_ins(ir::Ins::Call(ir::InsCall::new(dst, fun, args)), label);
    }

    pub fn ins_ret(&mut self, src: ir::RegId, label: Option<&str>) {
        self.append_ins(ir::Ins::Ret(ir::InsRet::new(src)), label);
    }

    fn add_unresolved(&mut self, ins: UnresolvedIns, label: Option<&str>) {
        let fun = self
            .current_fun
            .as_mut()
            .expect("Cannot insert instruction: no current function");
        fun.add_ins(ins, label);
    }

    fn gen_fun_addr(&mut self) -> ir::FunAddress {
        let res = ir::FunAddress(self.next_fun_addr);
        self.next_fun_addr += 1;
        return res;
    }

    fn gen_fun_name(&mut self) -> String {
        let res = "f".to_string() + &self.next_fun_name.to_string();
        self.next_fun_name += 1;
        res
    }
}
