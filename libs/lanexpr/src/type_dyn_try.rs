use crate::ast;

use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum TypeHandle {
    Missing,
    Invalid,
    Handle(usize),
}

enum TypeDef {
    Int,
    Void,
    Ref(TypeHandle),
}

struct TypeHeader {
    t_val: TypeHandle,
    t_ref: TypeHandle,

    def: TypeDef,
}

pub struct TypeBuilder {
    types: Vec<TypeHeader>,

    handle_int: TypeHandle,
    handle_void: TypeHandle,
}

impl TypeBuilder {
    /*
    fn add_type(&mut self, id: &str, ty: LEType) -> &'a LEType {
        self.types.push(Box::new(ty));
        let ty_ref: &LEType = self.types.last().unwrap();
        let ty_a_ref = unsafe { std::mem::transmute::<&LEType, &'a LEType>(ty_ref) };
        self.tmap.insert(id.to_string(), ty_a_ref);
        ty_a_ref
    }

    fn get_or_add_type(&mut self, id: &str, ty: LEType) -> &'a LEType {
        match self.tmap.get(id) {
            Some(ty) => ty,
            None => self.add_type(id, ty),
        }
    }

    pub fn build_int(&mut self) -> &'a LEType {
        let id = "int";
        let ty = LEType::Int;
        self.get_or_add_type(id, ty)
    }
     */

    /*
    fn handle2header<'a>(&'a self, handle: TypeHandle) -> &'a TypeHeader {
        let handle = match handle {
            TypeHandle::Handle(x) => x,
            _ => panic!("handle not resolved"),
        };

        let ty_ref: &TypeHeader = self.types.get(handle).unwrap();
        unsafe { std::mem::transmute::<&TypeHeader, &'a TypeHeader>(ty_ref) }
    }
     */

    fn add_type(&mut self, ty: TypeHeader) -> TypeHandle {
        self.types.push(ty);
        TypeHandle::Handle(self.types.len())
    }

    fn handle2header(&self, handle: TypeHandle) -> &TypeHeader {
        let handle = match handle {
            TypeHandle::Handle(x) => x,
            _ => panic!("handle not resolved"),
        };

        let ty_ref: &TypeHeader = self.types.get(handle).unwrap();
        ty_ref
        //unsafe { std::mem::transmute::<&TypeHeader, &'a TypeHeader>(ty_ref) }
    }

    fn handle2type(&self, handle: TypeHandle) -> &TypeDef {
        &self.handle2header(handle).def
    }

    pub fn build_int(&mut self) -> TypeHandle {
        match self.handle_int {
            TypeHandle::missing => {}
        }

        self.handle_int
    }
}

//type LETypePtr = Box<LEType>;

//type LETypePtr = &'static LEType;

pub struct TypeChecker {}
