//In the IR, basic blocks, functions and registers are all represented by ids with are unique integers
//This file Contains structs to give identifier names to all these ids

use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

use crate::ir;
use crate::registers::GetRegistersUse;

struct IdsMapper<T> {
    //TODO: this implem need to clone strings
    id2name: HashMap<T, String>,
    name2id: HashMap<String, T>,
    count_gen: usize,
}

impl<T: Clone + Copy + Eq + PartialEq + Hash> IdsMapper<T> {
    fn new() -> Self {
        IdsMapper {
            id2name: HashMap::new(),
            name2id: HashMap::new(),
            count_gen: 0,
        }
    }

    fn insert(&mut self, id: T, name: String) {
        assert!(self.id2name.get(&id).is_none());
        assert!(self.name2id.get(&name).is_none());

        self.id2name.insert(id, name.clone());
        self.name2id.insert(name, id);
    }

    fn id2name(&self, id: T) -> Option<&str> {
        match self.id2name.get(&id) {
            Some(x) => Some(&x),
            None => None,
        }
    }

    fn name2id(&self, name: &str) -> Option<T> {
        self.name2id.get(name).copied()
    }

    fn gen_name(&mut self, prefix: &str) -> String {
        loop {
            let name = format!("{}{}", prefix, self.count_gen);
            self.count_gen += 1;
            if self.name2id(&name).is_none() {
                return name;
            }
        }
    }
}

pub struct FunctionNames {
    id: ir::FunctionId,
    regs: IdsMapper<ir::RegId>,
    bbs: IdsMapper<ir::BasicBlockId>,
}

impl FunctionNames {
    pub fn new(id: ir::FunctionId) -> Self {
        FunctionNames {
            id,
            regs: IdsMapper::new(),
            bbs: IdsMapper::new(),
        }
    }

    pub fn id(&self) -> ir::FunctionId {
        self.id
    }

    pub fn add_register(&mut self, id: ir::RegId, name: String) {
        self.regs.insert(id, name);
    }

    pub fn get_register_id(&self, name: &str) -> Option<ir::RegId> {
        self.regs.name2id(name)
    }

    pub fn get_register_name(&self, id: ir::RegId) -> Option<&str> {
        self.regs.id2name(id)
    }

    pub fn add_basic_block(&mut self, id: ir::BasicBlockId, name: String) {
        self.bbs.insert(id, name);
    }

    pub fn get_basic_block_id(&self, name: &str) -> Option<ir::BasicBlockId> {
        self.bbs.name2id(name)
    }

    pub fn get_basic_block_name(&self, id: ir::BasicBlockId) -> Option<&str> {
        self.bbs.id2name(id)
    }

    // Give generic names to all unnamed basic blocks and registers
    pub fn complete_undefined(&mut self, fun: &ir::Function) {
        assert!(fun.id() == self.id);
        assert!(!fun.is_extern());

        // 1) Find all registers
        let mut all_regs = HashSet::new();
        for bb_id in fun.basic_blocks_list() {
            let bb = fun.get_basic_block(*bb_id);
            for ins in bb.iter() {
                ins.get_register_use(&mut all_regs);
            }
        }

        // 2) Name all registers in order
        let mut all_regs: Vec<_> = all_regs.into_iter().collect();
        all_regs.sort();
        for reg in all_regs {
            if self.get_register_name(reg).is_some() {
                continue;
            }
            let reg_name = self.regs.gen_name("r");
            self.add_register(reg, reg_name);
        }

        // 3) Name all basic blocks in order
        let mut all_bbs: Vec<_> = fun.basic_blocks_list().iter().map(|x| *x).collect();
        all_bbs.sort();
        for bb_id in all_bbs {
            if self.get_basic_block_name(bb_id).is_some() {
                continue;
            }
            let bb_name = self.bbs.gen_name("L");
            self.add_basic_block(bb_id, bb_name);
        }
    }
}

pub struct ModuleNames {
    funs: IdsMapper<ir::FunctionId>,
    funs_names: HashMap<ir::FunctionId, FunctionNames>,
}

impl ModuleNames {
    pub fn new() -> Self {
        ModuleNames {
            funs: IdsMapper::new(),
            funs_names: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, id: ir::FunctionId, name: String) {
        self.funs.insert(id, name);
        self.funs_names.insert(id, FunctionNames::new(id));
    }

    pub fn get_function_id(&self, name: &str) -> Option<ir::FunctionId> {
        self.funs.name2id(name)
    }

    pub fn get_function_name(&self, id: ir::FunctionId) -> Option<&str> {
        self.funs.id2name(id)
    }

    pub fn get_function(&self, id: ir::FunctionId) -> Option<&FunctionNames> {
        self.funs_names.get(&id)
    }

    pub fn get_function_mut(&mut self, id: ir::FunctionId) -> Option<&mut FunctionNames> {
        self.funs_names.get_mut(&id)
    }

    // Give generic names to all unnamed functions, basic blocks and registers
    pub fn complete_undefined(&mut self, module: &ir::Module) {
        // 1) Find and give name to all local funstions
        let mut all_funs: Vec<_> = module.funs().iter().map(|fun| fun.id()).collect();
        all_funs.sort();
        for fun_id in all_funs {
            if self.get_function_name(fun_id).is_some() {
                continue;
            }

            let fun_name = self.funs.gen_name("f");
            self.add_function(fun_id, fun_name);
        }

        // 2) Complere functions: basic blocks, registers
        for fun in module.funs() {
            if fun.is_extern() {
                continue;
            }
            let fun_names = self.get_function_mut(fun.id()).unwrap();
            fun_names.complete_undefined(fun);
        }
    }
}
