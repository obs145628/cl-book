use crate::ast::ASTUid;
use crate::letype::Type;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct BindVarId {
    id: usize,
}

#[derive(Debug)]
pub struct BindVar {
    name: String,
    ty: Type,
    id: BindVarId,
    ast_id: ASTUid,
}

impl BindVar {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> Type {
        self.ty
    }

    pub fn id(&self) -> BindVarId {
        self.id
    }

    pub fn ast_id(&self) -> ASTUid {
        self.ast_id
    }

    pub fn dump_bindings(&self) {
        println!("[VAR][{}] {}: {:?}", self.id.id, self.name, self.ty);
    }
}

pub struct BindVarsList {
    list: Vec<BindVar>,
}

impl BindVarsList {
    pub fn new() -> Self {
        BindVarsList { list: vec![] }
    }

    pub fn add_var(&mut self, name: &str, ty: Type, ast_id: ASTUid) -> BindVarId {
        let id = BindVarId {
            id: self.list.len(),
        };
        self.list.push(BindVar {
            name: String::from(name),
            ty,
            id,
            ast_id,
        });
        id
    }

    pub fn get_var(&self, id: BindVarId) -> &BindVar {
        &self.list[id.id]
    }

    pub fn get_vars(&self) -> &Vec<BindVar> {
        &self.list
    }
}
