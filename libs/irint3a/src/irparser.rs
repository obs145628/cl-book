// Module to parse an IR Input file and generate some valid code
//
// File Syntax:
// A file is a sequence of function definitions and declarations:
// We can add comments anywhere using ';' <text>
//
// Function declaration
// '.declare' <id@int> <name@str>
//
// Function definition:
// '.define' <id@int> <name@str> code
//
// code is a sequence of instructions
// each instruction may have a label
// A label means the beginning of a new basic block
//
// Instructions syntax:
// - movi: 'movi' %<dts-reg@str>, <val@int>
// - movr: 'movr' %<dst-reg@str>, %<src-reg@str>
// - load: 'load' %<dst-reg@str>, %<src-reg@str>
// - store: 'store' %<dst-reg@str>, %<src-reg@str>
// - alloca: 'alloca' %<dst-reg@str>
// - add: 'add' %<dst-reg@str>, %<src1-reg@str>, %<src2-reg@str>
// - sub: 'sub' %<dst-reg@str>, %<src1-reg@str>, %<src2-reg@str>
// - mul: 'mul' %<dst-reg@str>, %<src1-reg@str>, %<src2-reg@str>
// - div: 'div' %<dst-reg@str>, %<src1-reg@str>, %<src2-reg@str
// - mod: 'mod' %<dst-reg@str>, %<src1-reg@str>, %<src2-reg@str>
// - cmpeq: 'cmpeq' %<dst-reg@str>, %<src1-reg@str>, %<src2-reg@str>
// - cmplt: 'cmplt' %<dst-reg@str>, %<src1-reg@str>, %<src2-reg@str>
// - cmpgt: 'cmpgt' %<dst-reg@str>, %<src1-reg@str>, %<src2-reg@str>
// - jump: 'jump' <dst-bb@str>
// - br: 'br' %<src-reg@str>, <dst-true-bb@str>, <dst-false-bb@str>
// - call: 'call' %<dst-reg@str>, <fun@str> (, %<arg-i-reg@str>)*
// - ret: 'ret' %<src-reg@str>

use std::collections::HashSet;

use crate::ir;
use crate::irbuilder::IRBuilder;
use crate::irnames;
use crate::irvalidation;
use asmparser::parser::{Ins, InsArg, InsLabel, Item};

#[derive(Debug)]
struct ParsedBasicBlock {
    id: String,
    code: Vec<Ins>,
}

#[derive(Debug)]
struct ParsedFunction {
    id: ir::FunctionId,
    name: String,
    bbs: Option<Vec<ParsedBasicBlock>>,
}

pub struct Parser {
    ps: asmparser::parser::Parser,
    module: ir::Module,
    names: irnames::ModuleNames,
}

impl Parser {
    pub fn from_file(path: &str) -> Self {
        Parser::new(asmparser::parser::Parser::from_file(path))
    }

    pub fn from_str(s: &str) -> Self {
        Parser::new(asmparser::parser::Parser::from_str(s))
    }

    pub fn build(mut self) -> (ir::Module, irnames::ModuleNames) {
        let funs = self.r_file();
        self.add_module(&funs);
        irvalidation::validate_module(&self.module);
        (self.module, self.names)
    }

    fn new(ps: asmparser::parser::Parser) -> Self {
        Parser {
            ps,
            module: ir::Module::new(),
            names: irnames::ModuleNames::new(),
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

        let def_id = def_id.trim();
        let sep_pos = def_id
            .find(' ')
            .expect("Invalid def syntax: must be .define <fn-id> <fn-name>");
        let def_name = &def_id[sep_pos + 1..];
        let def_id = &def_id[0..sep_pos];

        let def_id = def_id
            .trim()
            .parse::<usize>()
            .expect("function id must be a number");
        let def_id = ir::FunctionId(def_id);

        if is_extern {
            return ParsedFunction {
                id: def_id,
                name: def_name.to_string(),
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
            name: def_name.to_string(),
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
                match &ins.label {
                    Some(InsLabel::IdName(label)) => {
                        id = Some(label.clone());
                    }
                    Some(InsLabel::IdInt(_)) => panic!("Label cannot be a int"),
                    None => panic!("First instruction in a basic block must have a label"),
                }
            } else {
                match ins.label {
                    Some(InsLabel::IdName(_)) => {
                        break;
                    }
                    Some(InsLabel::IdInt(_)) => panic!("Label cannot be a int"),
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
        // 1) Ignore extern functions
        if fun.bbs.is_none() {
            return;
        }

        // 2) Register All basic blocs and register names
        self.add_bbs_defs(fun);
        self.add_regs_defs(fun);

        // 3) Call FunParser to build all the code
        let fun_names = self.names.get_function(fun.id).unwrap();
        let mut fun_ir = self.module.get_fun_mut(fun.id).unwrap();
        let builder = IRBuilder::new(&mut fun_ir);
        let mut fun_ps = FunctionParser::new(builder, &self.names, fun_names);
        fun_ps.parse(fun);
    }

    fn add_fun_defs(&mut self, funs: &Vec<ParsedFunction>) {
        for fun in funs {
            self.names.add_function(fun.id, fun.name.clone());
            if fun.bbs.is_none() {
                self.module.create_extern_function(fun.id);
            } else {
                self.module.create_function(Some(fun.id));
            }
        }
    }

    fn add_bbs_defs(&mut self, fun: &ParsedFunction) {
        let fun_names = self.names.get_function_mut(fun.id).unwrap();
        let fun_ir = self.module.get_fun_mut(fun.id).unwrap();

        for bb in fun.bbs.as_ref().unwrap() {
            let bb_name = bb.id.to_string();
            let bb_id = fun_ir.create_basic_block();
            fun_names.add_basic_block(bb_id, bb_name);
        }
    }

    fn add_regs_defs(&mut self, fun: &ParsedFunction) {
        // 1) Make the list of all registers in the function
        let mut all_regs: HashSet<&str> = HashSet::new();
        for bb in fun.bbs.as_ref().unwrap() {
            for ins in &bb.code {
                for arg in &ins.args {
                    if let InsArg::IdName(rname) = arg {
                        all_regs.insert(rname);
                    }
                }
            }
        }

        // 2) Sort all registers by names

        let mut all_regs: Vec<_> = all_regs.iter().collect();
        all_regs.sort_by(|a, b| cmp_reg_keys(a, b));

        // 3) Add registers
        let fun_names = self.names.get_function_mut(fun.id).unwrap();
        for (reg_id, reg_name) in all_regs.iter().enumerate() {
            let reg_id = ir::RegId(reg_id);
            fun_names.add_register(reg_id, reg_name.to_string());
        }
    }
}

fn reg_key_to_id(s: &str) -> Option<usize> {
    match s.chars().nth(0) {
        Some(c) if c == 'r' => s[1..].parse().ok(),
        _ => None,
    }
}

fn cmp_reg_keys(a: &str, b: &str) -> std::cmp::Ordering {
    let a_id = reg_key_to_id(a);
    let b_id = reg_key_to_id(b);
    if a_id.is_some() && a_id.is_some() {
        a_id.unwrap().cmp(&b_id.unwrap())
    } else if a_id.is_some() {
        std::cmp::Ordering::Less
    } else if b_id.is_some() {
        std::cmp::Ordering::Greater
    } else {
        a.cmp(b)
    }
}

struct FunctionParser<'a> {
    builder: IRBuilder<'a>,
    module_names: &'a irnames::ModuleNames,
    names: &'a irnames::FunctionNames,
}

impl<'a> FunctionParser<'a> {
    pub fn new(
        builder: IRBuilder<'a>,
        module_names: &'a irnames::ModuleNames,
        names: &'a irnames::FunctionNames,
    ) -> Self {
        FunctionParser {
            builder,
            module_names,
            names,
        }
    }

    pub fn parse(&mut self, fun: &ParsedFunction) {
        for bb in fun.bbs.as_ref().unwrap() {
            self.parse_bb(bb);
        }
    }

    pub fn parse_bb(&mut self, bb: &ParsedBasicBlock) {
        let bb_id = self.names.get_basic_block_id(&bb.id).unwrap();
        self.builder.set_insert_point(bb_id);
        for ins in &bb.code {
            self.add_ins(ins);
        }
    }

    fn add_ins(&mut self, ins: &Ins) {
        match ins.name.as_str() {
            "movi" => self.add_ins_movi(&ins.args),
            "movr" => self.add_ins_movr(&ins.args),
            "load" => self.add_ins_load(&ins.args),
            "store" => self.add_ins_store(&ins.args),
            "alloca" => self.add_ins_alloca(&ins.args),
            "add" => self.add_ins_add(&ins.args),
            "sub" => self.add_ins_sub(&ins.args),
            "mul" => self.add_ins_mul(&ins.args),
            "div" => self.add_ins_div(&ins.args),
            "mod" => self.add_ins_mod(&ins.args),
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

    fn check_args_count(&self, name: &str, args: &[InsArg], exp_len: usize) {
        if args.len() != exp_len {
            panic!(
                "Instruction {} expected {} arguments, got {}",
                name,
                exp_len,
                args.len()
            );
        }
    }

    fn check_arg_const(&self, name: &str, args: &[InsArg], id: usize) -> usize {
        match &args[id] {
            InsArg::Const(val) => *val as usize,
            _ => panic!("Instruction {}: arg #{} must a constant", name, id + 1),
        }
    }

    fn check_arg_label(&self, name: &str, args: &[InsArg], id: usize) -> ir::BasicBlockId {
        let bb = match &args[id] {
            InsArg::Name(name) => name,
            _ => panic!("Instruction {}: arg #{} must a label", name, id + 1),
        };
        self.names.get_basic_block_id(&bb).unwrap()
    }

    fn check_arg_function(&self, name: &str, args: &[InsArg], id: usize) -> ir::FunctionId {
        let fun = match &args[id] {
            InsArg::Name(name) => name,
            _ => panic!("Instruction {}: arg #{} must a label", name, id + 1),
        };
        self.module_names.get_function_id(&fun).unwrap()
    }

    fn check_arg_reg(&self, name: &str, args: &[InsArg], id: usize) -> ir::RegId {
        let reg = match &args[id] {
            InsArg::IdName(name) => name,
            _ => panic!("Instruction {}: arg #{} must a register", name, id + 1),
        };
        self.names.get_register_id(&reg).unwrap()
    }

    fn check_args_l(&self, name: &str, args: &[InsArg]) -> ir::BasicBlockId {
        self.check_args_count(name, args, 1);
        self.check_arg_label(name, args, 0)
    }

    fn check_args_r(&self, name: &str, args: &[InsArg]) -> ir::RegId {
        self.check_args_count(name, args, 1);
        self.check_arg_reg(name, args, 0)
    }

    fn check_args_rll(
        &self,
        name: &str,
        args: &[InsArg],
    ) -> (ir::RegId, ir::BasicBlockId, ir::BasicBlockId) {
        self.check_args_count(name, args, 3);
        let r = self.check_arg_reg(name, args, 0);
        let l1 = self.check_arg_label(name, args, 1);
        let l2 = self.check_arg_label(name, args, 2);
        (r, l1, l2)
    }

    fn check_args_rr(&self, name: &str, args: &[InsArg]) -> (ir::RegId, ir::RegId) {
        self.check_args_count(name, args, 2);
        let r1 = self.check_arg_reg(name, args, 0);
        let r2 = self.check_arg_reg(name, args, 1);
        (r1, r2)
    }

    fn check_args_rrr(&self, name: &str, args: &[InsArg]) -> (ir::RegId, ir::RegId, ir::RegId) {
        self.check_args_count(name, args, 3);
        let r1 = self.check_arg_reg(name, args, 0);
        let r2 = self.check_arg_reg(name, args, 1);
        let r3 = self.check_arg_reg(name, args, 2);
        (r1, r2, r3)
    }

    fn check_args_rc(&self, name: &str, args: &[InsArg]) -> (ir::RegId, usize) {
        self.check_args_count(name, args, 2);
        let r1 = self.check_arg_reg(name, args, 0);
        let r2 = self.check_arg_const(name, args, 1);
        (r1, r2)
    }

    fn add_ins_movi(&mut self, args: &[InsArg]) {
        let (dst, val) = self.check_args_rc("movi", args);
        self.builder.ins_movi(dst, val as i32);
    }

    fn add_ins_movr(&mut self, args: &[InsArg]) {
        let (dst, src) = self.check_args_rr("movr", args);
        self.builder.ins_movr(dst, src);
    }

    fn add_ins_load(&mut self, args: &[InsArg]) {
        let (dst, src) = self.check_args_rr("load", args);
        self.builder.ins_load(dst, src);
    }

    fn add_ins_store(&mut self, args: &[InsArg]) {
        let (dst, src) = self.check_args_rr("store", args);
        self.builder.ins_store(dst, src);
    }

    fn add_ins_alloca(&mut self, args: &[InsArg]) {
        let dst = self.check_args_r("alloca", args);
        self.builder.ins_alloca(dst);
    }

    fn add_ins_add(&mut self, args: &[InsArg]) {
        let (dst, src1, src2) = self.check_args_rrr("add", args);
        self.builder.ins_add(dst, src1, src2);
    }

    fn add_ins_sub(&mut self, args: &[InsArg]) {
        let (dst, src1, src2) = self.check_args_rrr("sub", args);
        self.builder.ins_sub(dst, src1, src2);
    }

    fn add_ins_mul(&mut self, args: &[InsArg]) {
        let (dst, src1, src2) = self.check_args_rrr("mul", args);
        self.builder.ins_mul(dst, src1, src2);
    }

    fn add_ins_div(&mut self, args: &[InsArg]) {
        let (dst, src1, src2) = self.check_args_rrr("div", args);
        self.builder.ins_div(dst, src1, src2);
    }

    fn add_ins_mod(&mut self, args: &[InsArg]) {
        let (dst, src1, src2) = self.check_args_rrr("mod", args);
        self.builder.ins_mod(dst, src1, src2);
    }

    fn add_ins_cmpeq(&mut self, args: &[InsArg]) {
        let (dst, src1, src2) = self.check_args_rrr("cmpeq", args);
        self.builder.ins_cmpeq(dst, src1, src2);
    }

    fn add_ins_cmplt(&mut self, args: &[InsArg]) {
        let (dst, src1, src2) = self.check_args_rrr("cmplt", args);
        self.builder.ins_cmplt(dst, src1, src2);
    }

    fn add_ins_cmpgt(&mut self, args: &[InsArg]) {
        let (dst, src1, src2) = self.check_args_rrr("cmpgt", args);
        self.builder.ins_cmpgt(dst, src1, src2);
    }

    fn add_ins_jump(&mut self, args: &[InsArg]) {
        let dst = self.check_args_l("jump", args);
        self.builder.ins_jump(dst);
    }

    fn add_ins_br(&mut self, args: &[InsArg]) {
        let (src, dst_true, dst_false) = self.check_args_rll("br", args);
        self.builder.ins_br(src, dst_true, dst_false);
    }

    fn add_ins_call(&mut self, args: &[InsArg]) {
        if args.len() < 2 {
            println!(
                "call instructions expected at least 2 arguments, got {}",
                args.len()
            );
        }
        let dst = self.check_arg_reg("call", args, 0);
        let fun = self.check_arg_function("call", args, 1);
        let args: Vec<_> = (2..args.len())
            .map(|id| self.check_arg_reg("call", args, id))
            .collect();
        self.builder.ins_call(dst, fun, args);
    }

    fn add_ins_ret(&mut self, args: &[InsArg]) {
        let src = self.check_args_r("ret", args);
        self.builder.ins_ret(src);
    }
}
