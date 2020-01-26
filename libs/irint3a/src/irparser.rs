use oblexer::token::Token;

use crate::ir;
use crate::irbuilder::IRBuilder;

pub struct Parser {
    ps: obparser::parser::Parser,
    builder: IRBuilder,
}

impl Parser {
    fn kws() -> Vec<&'static str> {
        vec![
            "define", "extern", "movi", "movr", "load", "store", "alloca", "add", "sub", "mul",
            "div", "mod", "cmpeq", "cmplt", "cmpgt", "jump", "br", "call", "ret",
        ]
    }

    fn syms() -> Vec<&'static str> {
        vec!["%", ",", ":"]
    }

    pub fn new_from_file(path: &str) -> Parser {
        Parser {
            ps: obparser::parser::Parser::new_from_file(path, Parser::kws(), Parser::syms()),
            builder: IRBuilder::new(),
        }
    }

    pub fn new_from_str(path: &str) -> Parser {
        Parser {
            ps: obparser::parser::Parser::new_from_str(path, Parser::kws(), Parser::syms()),
            builder: IRBuilder::new(),
        }
    }

    pub fn parse(mut self) -> ir::ModuleExtended {
        self.r_file();
        self.builder.build()
    }

    //file: fundef* <<EOF>>
    fn r_file(&mut self) {
        while !self.ps.try_eat_eof() {
            self.r_fundef();
        }
    }

    // fundef:  'define' @int @id <body>
    //        | 'define' @int @id 'extern'
    fn r_fundef(&mut self) {
        self.ps.eat_keyword("define");
        let addr = ir::FunAddress(self.ps.eat_vint() as usize);
        let name = self.ps.eat_id();

        if self.ps.try_eat_keyword("extern") {
            self.builder.add_extern_fun(Some(&name), addr);
        } else {
            self.builder.begin_function(Some(&name), Some(addr));
            self.r_body();
            self.builder.end_function();
        }
    }

    fn r_body(&mut self) {
        loop {
            match self.ps.peek_token() {
                Token::Keyword(x) if x == "define" => break,
                Token::EOF => break,
                _ => {}
            };

            self.r_ins();
        }
    }

    // ins: [@id ':'] ins_sim
    //
    // ins_sim: ins_movi | ins_movr | etc
    fn r_ins(&mut self) {
        let label_str: Option<String> = self.ps.try_eat_id();
        if label_str.is_some() {
            self.ps.eat_sym(":");
        }
        let label: Option<&str> = match &label_str {
            Some(x) => Some(x),
            None => None,
        };

        match self.ps.peek_token() {
            Token::Keyword(x) if x == "movi" => self.r_movi(label),
            Token::Keyword(x) if x == "movr" => self.r_movr(label),
            Token::Keyword(x) if x == "load" => self.r_load(label),
            Token::Keyword(x) if x == "store" => self.r_store(label),
            Token::Keyword(x) if x == "alloca" => self.r_alloca(label),
            Token::Keyword(x) if x == "add" => self.r_add(label),
            Token::Keyword(x) if x == "sub" => self.r_sub(label),
            Token::Keyword(x) if x == "mul" => self.r_mul(label),
            Token::Keyword(x) if x == "div" => self.r_div(label),
            Token::Keyword(x) if x == "mod" => self.r_mod(label),
            Token::Keyword(x) if x == "cmpeq" => self.r_cmpeq(label),
            Token::Keyword(x) if x == "cmplt" => self.r_cmplt(label),
            Token::Keyword(x) if x == "cmpgt" => self.r_cmpgt(label),
            Token::Keyword(x) if x == "jump" => self.r_jump(label),
            Token::Keyword(x) if x == "br" => self.r_br(label),
            Token::Keyword(x) if x == "call" => self.r_call(label),
            Token::Keyword(x) if x == "ret" => self.r_ret(label),
            _ => panic!(
                "Trying to parse instruction: invalid token {:?}",
                self.ps.peek_token()
            ),
        }
    }

    // movi: 'movi' '%' @int ',' @int
    fn r_movi(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("movi");
        self.ps.eat_sym("%");
        let dst = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        let const_val = self.ps.eat_vint() as i32;
        self.builder.ins_movi(dst, const_val, label);
    }

    // movr: 'movr' '%' @int ','  '%' @int
    fn r_movr(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("movr");
        self.ps.eat_sym("%");
        let dst = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src = ir::RegId(self.ps.eat_vint() as usize);
        self.builder.ins_movr(dst, src, label);
    }

    // load: 'load' '%' @int ','  '%' @int
    fn r_load(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("load");
        self.ps.eat_sym("%");
        let dst = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src = ir::RegId(self.ps.eat_vint() as usize);
        self.builder.ins_load(dst, src, label);
    }

    // store: 'store' '%' @int ','  '%' @int
    fn r_store(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("store");
        self.ps.eat_sym("%");
        let dst = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src = ir::RegId(self.ps.eat_vint() as usize);
        self.builder.ins_store(dst, src, label);
    }

    // alloca: 'alloca' '%' @int
    fn r_alloca(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("alloca");
        self.ps.eat_sym("%");
        let dst = ir::RegId(self.ps.eat_vint() as usize);
        self.builder.ins_alloca(dst, label);
    }

    // add: 'add' '%' @int ','  '%' @int ','  '%' @int
    fn r_add(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("add");
        self.ps.eat_sym("%");
        let dst = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src1 = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src2 = ir::RegId(self.ps.eat_vint() as usize);
        self.builder.ins_add(dst, src1, src2, label);
    }

    // sub: 'sub' '%' @int ','  '%' @int ','  '%' @int
    fn r_sub(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("sub");
        self.ps.eat_sym("%");
        let dst = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src1 = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src2 = ir::RegId(self.ps.eat_vint() as usize);
        self.builder.ins_sub(dst, src1, src2, label);
    }

    // mul: 'mul' '%' @int ','  '%' @int ','  '%' @int
    fn r_mul(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("mul");
        self.ps.eat_sym("%");
        let dst = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src1 = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src2 = ir::RegId(self.ps.eat_vint() as usize);
        self.builder.ins_mul(dst, src1, src2, label);
    }

    // div: 'div' '%' @int ','  '%' @int ','  '%' @int
    fn r_div(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("div");
        self.ps.eat_sym("%");
        let dst = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src1 = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src2 = ir::RegId(self.ps.eat_vint() as usize);
        self.builder.ins_div(dst, src1, src2, label);
    }

    // mod: 'mod' '%' @int ','  '%' @int ','  '%' @int
    fn r_mod(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("mod");
        self.ps.eat_sym("%");
        let dst = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src1 = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src2 = ir::RegId(self.ps.eat_vint() as usize);
        self.builder.ins_mod(dst, src1, src2, label);
    }

    // cmpeq: 'cmpeq' '%' @int ','  '%' @int ','  '%' @int
    fn r_cmpeq(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("cmpeq");
        self.ps.eat_sym("%");
        let dst = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src1 = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src2 = ir::RegId(self.ps.eat_vint() as usize);
        self.builder.ins_cmpeq(dst, src1, src2, label);
    }

    // cmplt: 'cmplt' '%' @int ','  '%' @int ','  '%' @int
    fn r_cmplt(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("cmplt");
        self.ps.eat_sym("%");
        let dst = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src1 = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src2 = ir::RegId(self.ps.eat_vint() as usize);
        self.builder.ins_cmplt(dst, src1, src2, label);
    }

    // cmpgt: 'cmpgt' '%' @int ','  '%' @int ','  '%' @int
    fn r_cmpgt(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("cmpgt");
        self.ps.eat_sym("%");
        let dst = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src1 = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        self.ps.eat_sym("%");
        let src2 = ir::RegId(self.ps.eat_vint() as usize);
        self.builder.ins_cmpgt(dst, src1, src2, label);
    }

    // jump: 'jump' @id
    fn r_jump(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("jump");
        let jump_label = self.ps.eat_id();
        self.builder.ins_jump(&jump_label, label);
    }

    // br: 'br' '%' @int ',' @id ',' @id
    fn r_br(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("br");
        self.ps.eat_sym("%");
        let src = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        let label_true = self.ps.eat_id();
        self.ps.eat_sym(",");
        let label_false = self.ps.eat_id();
        self.builder.ins_br(src, &label_true, &label_false, label);
    }

    //call : 'call' '%' @int ','  @id (',' '%' @int)*
    fn r_call(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("call");
        self.ps.eat_sym("%");
        let dst = ir::RegId(self.ps.eat_vint() as usize);
        self.ps.eat_sym(",");
        let fun = self.ps.eat_id();

        let mut args = vec![];
        while self.ps.try_eat_sym(",") {
            self.ps.eat_sym("%");
            args.push(ir::RegId(self.ps.eat_vint() as usize));
        }

        self.builder.ins_call_name(dst, &fun, args, label);
    }

    //ret: 'ret' '%' @int
    fn r_ret(&mut self, label: Option<&str>) {
        self.ps.eat_keyword("ret");
        self.ps.eat_sym("%");
        let src = ir::RegId(self.ps.eat_vint() as usize);
        self.builder.ins_ret(src, label);
    }
}
