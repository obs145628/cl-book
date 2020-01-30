use std::collections::HashMap;

use crate::ir;
use crate::irbuilder::IRBuilder;
use asmparser::parser::{Ins, InsArg, InsLabel, Item};

struct ParsedBasicBlock {
    id: usize,
    code: Vec<Ins>,
}

struct ParsedFunction {
    id: ir::FunctionRef,
    bbs: Option<Vec<ParsedBasicBlock>>,
}

pub struct Parser {
    ps: asmparser::parser::Parser,
    builder: IRBuilder,
    mapping_bb: HashMap<usize, ir::BasicBlockRef>,
}

impl Parser {
    pub fn from_file(path: &str) -> Self {
        Parser::new(asmparser::parser::Parser::from_file(path))
    }

    pub fn from_str(s: &str) -> Self {
        Parser::new(asmparser::parser::Parser::from_str(s))
    }

    pub fn build(mut self) -> ir::Module {
        let funs = self.r_file();
        self.add_module(&funs);
        self.builder.finish()
    }

    fn new(ps: asmparser::parser::Parser) -> Self {
        Parser {
            ps,
            builder: IRBuilder::new(),
            mapping_bb: HashMap::new(),
        }
    }

    fn r_file(&mut self) -> Vec<ParsedFunction> {
        let mut res = vec![];

        loop {
            if self.ps.peek().is_none() {
                break;
            }

            res.push(self.r_fun());
        }

        res
    }

    fn r_fun(&mut self) -> ParsedFunction {
        let def = match self.ps.next() {
            Some(Item::Def(def)) => def,
            _ => panic!("Expected definition"),
        };

        let (is_extern, def_id) = if def.line.starts_with("declare") {
            (true, &def.line[7..])
        } else if def.line.starts_with("define") {
            (false, &def.line[6..])
        } else {
            panic!("Invalid definition {}", def.line);
        };

        let def_id = def_id
            .trim()
            .parse::<usize>()
            .expect("function id must be a number");
        let def_id = ir::FunctionRef::new(def_id);

        if is_extern {
            return ParsedFunction {
                id: def_id,
                bbs: None,
            };
        }

        let mut bbs = vec![];

        loop {
            match self.ps.peek() {
                None | Some(Item::Def(_)) => break,
                _ => {}
            }
            bbs.push(self.r_bb());
        }

        ParsedFunction {
            id: def_id,
            bbs: Some(bbs),
        }
    }

    fn r_bb(&mut self) -> ParsedBasicBlock {
        let mut code = vec![];
        let mut id = None;

        loop {
            let ins = match self.ps.peek() {
                Some(Item::Ins(ins)) => ins,
                _ => {
                    break;
                }
            };

            if code.len() == 0 {
                match ins.label {
                    Some(InsLabel::IdInt(label)) => {
                        id = Some(label as usize);
                    }
                    Some(InsLabel::IdName(_)) => panic!("Label cannot be a name"),
                    None => panic!("First instruction in a basic block must have a label"),
                }
            } else {
                match ins.label {
                    Some(InsLabel::IdInt(_)) => {
                        break;
                    }
                    Some(InsLabel::IdName(_)) => panic!("Label cannot be a name"),
                    None => {}
                }
            }

            let ins = match self.ps.next() {
                Some(Item::Ins(ins)) => ins,
                _ => unreachable!(),
            };
            code.push(ins);
        }

        if code.len() == 0 {
            panic!("Basic block cannot be empty");
        }

        ParsedBasicBlock {
            id: id.unwrap(),
            code,
        }
    }

    fn add_module(&mut self, funs: &Vec<ParsedFunction>) {
        self.add_fun_defs(funs);
        for fun in funs {
            self.add_fun(fun);
        }
    }

    fn add_fun(&mut self, fun: &ParsedFunction) {
        let bbs = match fun.bbs.as_ref() {
            Some(bbs) => bbs,
            None => return,
        };
        self.add_bbs_defs(fun);

        for bb in bbs {
            self.add_bb(bb);
        }
    }

    fn add_fun_defs(&mut self, funs: &Vec<ParsedFunction>) {
        for fun in funs {
            self.builder.create_function(Some(fun.id));
        }
    }

    fn add_bbs_defs(&mut self, fun: &ParsedFunction) {
        self.mapping_bb.clear();
        for bb in fun.bbs.as_ref().unwrap() {
            let bb_id = self.builder.create_basic_block(fun.id);
            self.mapping_bb.insert(bb.id, bb_id);
        }
    }

    fn add_bb(&mut self, bb: &ParsedBasicBlock) {
        let bb_id = *self.mapping_bb.get(&bb.id).unwrap();
        self.builder.set_insert_point(bb_id);
        for ins in &bb.code {
            self.add_ins(ins);
        }
    }

    fn add_ins(&mut self, ins: &Ins) {
        match ins.name.as_str() {
            "pop" => self.add_ins_pop(&ins.args),
            "const" => self.add_ins_const(&ins.args),
            "load" => self.add_ins_load(&ins.args),
            "store" => self.add_ins_store(&ins.args),
            "add" => self.add_ins_add(&ins.args),
            "sub" => self.add_ins_sub(&ins.args),
            "mul" => self.add_ins_mul(&ins.args),
            "div" => self.add_ins_div(&ins.args),
            "rem" => self.add_ins_rem(&ins.args),
            "cmpeq" => self.add_ins_cmpeq(&ins.args),
            "cmplt" => self.add_ins_cmplt(&ins.args),
            "cmpgt" => self.add_ins_cmpgt(&ins.args),
            "jump" => self.add_ins_jump(&ins.args),
            "br" => self.add_ins_br(&ins.args),
            "call" => self.add_ins_call(&ins.args),
            "ret" => self.add_ins_ret(&ins.args),
            _ => panic!("Unknow instruction {}", ins.name),
        }
    }

    fn add_ins_pop(&mut self, args: &[InsArg]) {
        if args.len() != 0 {
            panic!("pop instruction has no arguments");
        }
        self.builder.ins_pop();
    }

    fn add_ins_const(&mut self, args: &[InsArg]) {
        if args.len() != 1 {
            panic!("const instruction has 1 argument");
        }

        let val = match args[0] {
            InsArg::Const(val) => val as i32,
            _ => panic!("src argument must be a const"),
        };

        self.builder.ins_const(val);
    }

    fn add_ins_load(&mut self, args: &[InsArg]) {
        if args.len() != 1 {
            panic!("load instruction has 1 argument");
        }

        let src = match args[0] {
            InsArg::Const(id) => id,
            _ => panic!("src argument must be a const"),
        };

        let src = ir::LocalsIndex::new(src as usize);
        self.builder.ins_load(src);
    }

    fn add_ins_store(&mut self, args: &[InsArg]) {
        if args.len() != 1 {
            panic!("store instruction has 1 argument");
        }

        let dst = match args[0] {
            InsArg::Const(id) => id,
            _ => panic!("dst argument must be a const"),
        };

        let dst = ir::LocalsIndex::new(dst as usize);
        self.builder.ins_store(dst);
    }

    fn add_ins_add(&mut self, args: &[InsArg]) {
        if args.len() != 0 {
            panic!("add instruction has no arguments");
        }
        self.builder.ins_add();
    }

    fn add_ins_sub(&mut self, args: &[InsArg]) {
        if args.len() != 0 {
            panic!("sub instruction has no arguments");
        }
        self.builder.ins_sub();
    }

    fn add_ins_mul(&mut self, args: &[InsArg]) {
        if args.len() != 0 {
            panic!("mul instruction has no arguments");
        }
        self.builder.ins_mul();
    }

    fn add_ins_div(&mut self, args: &[InsArg]) {
        if args.len() != 0 {
            panic!("div instruction has no arguments");
        }
        self.builder.ins_div();
    }

    fn add_ins_rem(&mut self, args: &[InsArg]) {
        if args.len() != 0 {
            panic!("rem instruction has no arguments");
        }
        self.builder.ins_rem();
    }

    fn add_ins_cmpeq(&mut self, args: &[InsArg]) {
        if args.len() != 0 {
            panic!("cmpeq instruction has no arguments");
        }
        self.builder.ins_cmpeq();
    }

    fn add_ins_cmplt(&mut self, args: &[InsArg]) {
        if args.len() != 0 {
            panic!("cmplt instruction has no arguments");
        }
        self.builder.ins_cmplt();
    }

    fn add_ins_cmpgt(&mut self, args: &[InsArg]) {
        if args.len() != 0 {
            panic!("cmpgt instruction has no arguments");
        }
        self.builder.ins_cmpgt();
    }

    fn add_ins_jump(&mut self, args: &[InsArg]) {
        if args.len() != 1 {
            panic!("jump instruction has 1 argument");
        }

        let dst = match args[0] {
            InsArg::IdInt(id) => id,
            _ => panic!("dst argument must be an idint"),
        };
        let dst = ir::BasicBlockRef::new(dst as usize);

        self.builder.ins_jump(dst);
    }

    fn add_ins_br(&mut self, args: &[InsArg]) {
        if args.len() != 2 {
            panic!("br instruction has 2 arguments");
        }

        let dst_true = match args[0] {
            InsArg::IdInt(id) => id,
            _ => panic!("dst_true argument must be an idint"),
        };
        let dst_false = match args[1] {
            InsArg::IdInt(id) => id,
            _ => panic!("dst_false argument must be an idint"),
        };
        let dst_true = ir::BasicBlockRef::new(dst_true as usize);
        let dst_false = ir::BasicBlockRef::new(dst_false as usize);

        self.builder.ins_br(dst_true, dst_false);
    }

    fn add_ins_call(&mut self, args: &[InsArg]) {
        if args.len() != 2 {
            panic!("call instruction has 2 arguments");
        }

        let fun = match args[0] {
            InsArg::IdInt(id) => id,
            _ => panic!("fun argument must be an idint"),
        };
        let nb_args = match args[1] {
            InsArg::Const(id) => id as usize,
            _ => panic!("nb_args argument must be a const"),
        };
        let fun = ir::FunctionRef::new(fun as usize);

        self.builder.ins_call(fun, nb_args);
    }

    fn add_ins_ret(&mut self, args: &[InsArg]) {
        if args.len() != 0 {
            panic!("ret instruction has no arguments");
        }

        self.builder.ins_ret();
    }
}
