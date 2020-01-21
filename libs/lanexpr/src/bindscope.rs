use crate::bindfun::BindFunId;
use crate::bindvar::BindVarId;

use std::collections::HashMap;

pub struct BindScope {
    funs: Vec<HashMap<String, BindFunId>>,
    vars: Vec<HashMap<String, BindVarId>>,
    fun_pos: Vec<usize>,
}

impl BindScope {
    pub fn new() -> Self {
        BindScope {
            funs: vec![],
            vars: vec![],
            fun_pos: vec![],
        }
    }

    pub fn add_fun(&mut self, name: &str, id: BindFunId) {
        //println!("scope: add_fun '{}'", name);
        assert!(self.find_fun_top(name).is_none());
        self.funs.last_mut().unwrap().insert(name.to_string(), id);
    }

    pub fn add_var(&mut self, name: &str, id: BindVarId) {
        //println!("scope: add_var '{}'", name);
        assert!(self.find_var_top(name).is_none());
        self.vars.last_mut().unwrap().insert(name.to_string(), id);
    }

    /// Open a new block scope
    pub fn open_scope(&mut self) {
        //println!("scope: open_scope");
        if self.fun_pos.len() == 0 {
            panic!("Cannot open a scope of variables without a function");
        }

        self.push_defs();
    }

    /// Close the most inner block scope
    pub fn close_scope(&mut self) {
        //println!("scope: close_scope");
        if self.funs.len() == 0 {
            panic!("Outer scope already closed");
        }
        if self.get_scope_id() == self.get_fun_scope_id() {
            panic!("Trying to close a scope of a function");
        }

        self.pop_defs();
    }

    /// Open a new function (automatically create a block scope)
    pub fn open_fun(&mut self) {
        //println!("scope: open_fun");
        self.push_defs();
        self.fun_pos.push(self.get_scope_id());
    }

    ///Close the most inner function (automatically close its block scope)
    pub fn close_fun(&mut self) {
        //println!("scope: close_fun");
        if self.funs.len() == 0 {
            panic!("Outer function already closed");
        }
        if self.get_scope_id() != self.get_fun_scope_id() {
            panic!("Trying to close a function with open block scopes left");
        }

        self.pop_defs();
        self.fun_pos.pop().unwrap();
    }

    pub fn nb_open_scopes(&self) -> usize {
        self.funs.len()
    }

    pub fn nb_open_funs(&self) -> usize {
        self.fun_pos.len()
    }

    pub fn find_fun(&self, name: &str) -> Option<BindFunId> {
        for scope in self.funs.iter().rev() {
            let item = scope.get(name);
            if item.is_some() {
                return item.copied();
            }
        }

        None
    }

    pub fn find_fun_top(&self, name: &str) -> Option<BindFunId> {
        self.funs.last().unwrap().get(name).copied()
    }

    pub fn find_var(&self, name: &str) -> Option<BindVarId> {
        //TODO: don't go below fun scope limit

        for scope in self.vars.iter().rev() {
            let item = scope.get(name);
            if item.is_some() {
                return item.copied();
            }
        }

        None
    }

    pub fn find_var_top(&self, name: &str) -> Option<BindVarId> {
        self.vars.last().unwrap().get(name).copied()
    }

    fn push_defs(&mut self) {
        self.funs.push(HashMap::new());
        self.vars.push(HashMap::new());
    }

    fn pop_defs(&mut self) {
        self.funs.pop().unwrap();
        self.vars.pop().unwrap();
    }

    fn get_scope_id(&self) -> usize {
        self.funs.len() - 1
    }

    fn get_fun_scope_id(&self) -> usize {
        *self.fun_pos.last().unwrap()
    }
}
