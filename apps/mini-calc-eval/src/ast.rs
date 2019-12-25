use crate::value::Value;

pub trait AST: Visitable {
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

impl Visitable for ASTConst {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_const(self);
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

impl Visitable for ASTBinOp {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_binop(self);
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

impl Visitable for ASTUnop {
    fn accept(&self, v: &mut dyn ASTVisitor) {
        v.visit_unop(self);
    }
}

impl AST for ASTUnop {
    fn dump(&self) {
        print!("{}(", self.op);
        self.right.dump();
        print!(")");
    }
}

pub trait ASTVisitor {
    fn visit_const(&mut self, ast: &ASTConst);
    fn visit_binop(&mut self, ast: &ASTBinOp);
    fn visit_unop(&mut self, ast: &ASTUnop);
}

pub trait Visitable {
    fn accept(&self, v: &mut dyn ASTVisitor);
}

pub struct ASTEval {
    res: Value,
}

impl ASTEval {
    fn get_eval(&mut self, ast: &dyn AST) -> Value {
        ast.accept(self);
        self.res
    }

    pub fn eval(ast: &dyn AST) -> Value {
        (ASTEval {
            res: Value::VInt(0),
        })
        .get_eval(ast)
    }
}

impl ASTVisitor for ASTEval {
    fn visit_const(&mut self, ast: &ASTConst) {
        self.res = ast.val;
    }

    fn visit_binop(&mut self, ast: &ASTBinOp) {
        match ast.op {
            ASTBinopOp::Add => {
                self.res = Value::add(&self.get_eval(&*ast.left), &self.get_eval(&*ast.right))
            }
            ASTBinopOp::Sub => {
                self.res = Value::sub(&self.get_eval(&*ast.left), &self.get_eval(&*ast.right))
            }
            ASTBinopOp::Mul => {
                self.res = Value::mul(&self.get_eval(&*ast.left), &self.get_eval(&*ast.right))
            }
            ASTBinopOp::Div => {
                self.res = Value::div(&self.get_eval(&*ast.left), &self.get_eval(&*ast.right))
            }
        }
    }

    fn visit_unop(&mut self, ast: &ASTUnop) {
        match ast.op {
            '+' => self.res = self.get_eval(&*ast.right),
            '-' => self.res = Value::sub(&Value::VInt(0), &self.get_eval(&*ast.right)),
            _ => panic!("Cannot eval ASTUnop, invalid op '{}'", ast.op),
        }
    }
}
