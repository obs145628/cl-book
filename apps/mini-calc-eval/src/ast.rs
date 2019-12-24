use crate::value::Value;

pub trait AST {
    fn dump(&self);
}

pub type ASTPtr = Box<dyn AST>;

pub struct ASTConst {
    val: Value,
}

impl ASTConst {
    pub fn new(val: Value) -> Box<ASTConst> {
        Box::new(ASTConst { val })
    }
}

impl AST for ASTConst {
    fn dump(&self) {
        match self.val {
            Value::VInt(x) => print!("{}", x),
            Value::VFloat(x) => print!("{}", x),
        }
    }
}

pub enum ASTBinopOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl ASTBinopOp {
    fn toc(&self) -> char {
        match self {
            ASTBinopOp::Add => '+',
            ASTBinopOp::Sub => '-',
            ASTBinopOp::Mul => '*',
            ASTBinopOp::Div => '/',
        }
    }

    fn fromc(c: char) -> ASTBinopOp {
        match c {
            '+' => ASTBinopOp::Add,
            '-' => ASTBinopOp::Sub,
            '*' => ASTBinopOp::Mul,
            '/' => ASTBinopOp::Div,
            _ => panic!("Invalid char for binop '{}'.", c),
        }
    }
}

pub struct ASTBinOp {
    op: ASTBinopOp,
    left: ASTPtr,
    right: ASTPtr,
}

impl ASTBinOp {
    pub fn new(op: ASTBinopOp, left: ASTPtr, right: ASTPtr) -> Box<ASTBinOp> {
        Box::new(ASTBinOp { op, left, right })
    }
}

impl AST for ASTBinOp {
    fn dump(&self) {
        print!("(");
        self.left.dump();
        print!(") {} (", self.op.toc());
        self.right.dump();
        print!(")");
    }
}

pub struct ASTUnop {
    op: char,
    right: ASTPtr,
}

impl ASTUnop {
    pub fn new(op: char, right: ASTPtr) -> Box<ASTUnop> {
        Box::new(ASTUnop { op, right })
    }
}

impl AST for ASTUnop {
    fn dump(&self) {
        print!("{}(", self.op);
        self.right.dump();
        print!(")");
    }
}
