use crate::letype::{FnType, LeType, LeTypePtr};
use std::collections::HashMap;

struct ScopeDefs {
    tys: HashMap<String, LeTypePtr>,
    fns: HashMap<String, FnType>,
    vars: HashMap<String, LeTypePtr>,
}

impl ScopeDefs {
    fn new() -> ScopeDefs {
        ScopeDefs {
            tys: HashMap::new(),
            fns: HashMap::new(),
            vars: HashMap::new(),
        }
    }
}

pub struct DefsTable {
    scopes: Vec<ScopeDefs>,
}

impl DefsTable {
    pub fn size(&self) -> usize {
        self.scopes.len()
    }

    pub fn new() -> DefsTable {
        let mut res = DefsTable { scopes: vec![] };
        res.open_scope();
        res.set_native_defs();
        res
    }

    pub fn add_type(&mut self, id: &str, ty: LeTypePtr) {
        assert!(self.get_top_type(id).is_none());
        self.scopes
            .last_mut()
            .unwrap()
            .tys
            .insert(id.to_string(), ty);
    }

    pub fn add_fun(&mut self, id: &str, ty: FnType) {
        assert!(self.get_top_fun(id).is_none());
        self.scopes
            .last_mut()
            .unwrap()
            .fns
            .insert(id.to_string(), ty);
    }

    pub fn add_var(&mut self, id: &str, ty: LeTypePtr) {
        assert!(self.get_top_var(id).is_none());
        self.scopes
            .last_mut()
            .unwrap()
            .vars
            .insert(id.to_string(), ty);
    }

    pub fn get_top_type(&self, id: &str) -> Option<&LeTypePtr> {
        self.scopes.last().unwrap().tys.get(id)
    }

    pub fn get_top_fun(&self, id: &str) -> Option<&FnType> {
        self.scopes.last().unwrap().fns.get(id)
    }

    pub fn get_top_var(&self, id: &str) -> Option<&LeTypePtr> {
        self.scopes.last().unwrap().vars.get(id)
    }

    pub fn get_type(&self, id: &str) -> Option<&LeTypePtr> {
        for scope in self.scopes.iter().rev() {
            let item = scope.tys.get(id);
            if item.is_some() {
                return item;
            }
        }

        None
    }

    pub fn get_fun(&self, id: &str) -> Option<&FnType> {
        for scope in self.scopes.iter().rev() {
            let item = scope.fns.get(id);
            if item.is_some() {
                return item;
            }
        }

        None
    }

    pub fn get_var(&self, id: &str) -> Option<&LeTypePtr> {
        for scope in self.scopes.iter().rev() {
            let item = scope.vars.get(id);
            if item.is_some() {
                return item;
            }
        }

        None
    }

    pub fn open_scope(&mut self) {
        self.scopes.push(ScopeDefs::new());
    }

    pub fn close_scope(&mut self) {
        if self.scopes.len() <= 1 {
            panic!("Trying to close native defs scope");
        }
        self.scopes.pop().unwrap();
    }

    fn set_native_defs(&mut self) {
        self.add_type("int", LeType::new_int());
        self.add_type("void", LeType::new_void());
    }
}
