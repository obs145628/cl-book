use crate::ast;
use crate::astcast::ASTStatic;
use oblexer::token::Token;

pub struct Parser {
    ps: obparser::parser::Parser,
}

impl Parser {
    fn kws() -> Vec<&'static str> {
        vec![
            "if", "then", "else", "let", "in", "while", "do", "var", "fun",
        ]
    }

    fn syms() -> Vec<&'static str> {
        vec![
            "=", "==", "<", ">", "+", "-", "*", "/", "%", "!", "(", ")", ",", ";", ":",
        ]
    }

    pub fn new_from_file(path: &str) -> Parser {
        Parser {
            ps: obparser::parser::Parser::new_from_file(path, Parser::kws(), Parser::syms()),
        }
    }

    pub fn new_from_str(path: &str) -> Parser {
        Parser {
            ps: obparser::parser::Parser::new_from_str(path, Parser::kws(), Parser::syms()),
        }
    }

    pub fn parse(&mut self) -> ast::ASTExprPtr {
        self.r_file()
    }

    // file: expr @eof
    fn r_file(&mut self) -> ast::ASTExprPtr {
        let res = self.r_expr();
        self.ps.eat_eof();
        res
    }

    //expr:  expr_if
    // | expr_let
    // | expr_while
    // | expr_val
    fn r_expr(&mut self) -> ast::ASTExprPtr {
        match self.ps.peek_token() {
            Token::Keyword(x) if x == "if" => self.r_expr_if(),
            Token::Keyword(x) if x == "let" => self.r_expr_let(),
            Token::Keyword(x) if x == "while" => self.r_expr_while(),
            _ => self.r_expr_val(),
        }
    }

    // expr_if: 'if' expr 'then' expr ['else' expr]
    fn r_expr_if(&mut self) -> ast::ASTExprPtr {
        self.ps.eat_keyword("if");
        let cond = self.r_expr();
        self.ps.eat_keyword("then");
        let if_val = self.r_expr();
        let else_val = if self.ps.try_eat_keyword("else") {
            self.r_expr()
        } else {
            ast::ASTExprBlock::new(vec![])
        };
        ast::ASTExprIf::new(cond, if_val, else_val)
    }

    // expr_let: 'let' def* 'in' expr
    fn r_expr_let(&mut self) -> ast::ASTExprPtr {
        self.ps.eat_keyword("let");
        let mut defs = vec![];
        while !self.ps.try_eat_keyword("in") {
            defs.push(self.r_def());
        }

        let val = self.r_expr();
        ast::ASTExprLet::new(defs, val)
    }

    // expr_while: 'while' expr 'do' expr
    fn r_expr_while(&mut self) -> ast::ASTExprPtr {
        self.ps.eat_keyword("while");
        let cond = self.r_expr();
        self.ps.eat_keyword("do");
        let body = self.r_expr();
        ast::ASTExprWhile::new(cond, body)
    }

    // expr_val: expr_v5
    fn r_expr_val(&mut self) -> ast::ASTExprPtr {
        self.r_expr_v5()
    }

    // expr_v5:  expr_v4
    //         | expr_v4 '=' expr_v5
    fn r_expr_v5(&mut self) -> ast::ASTExprPtr {
        let left = self.r_expr_v4();
        if !self.ps.try_eat_sym("=") {
            return left;
        }
        let right = self.r_expr_v5();
        ast::ASTExprCall::new("@op:set".to_string(), vec![left, right])
    }

    // expr_v4: expr_v3 ('==' expr_v3)*
    fn r_expr_v4(&mut self) -> ast::ASTExprPtr {
        let mut res = self.r_expr_v3();
        while self.ps.try_eat_sym("==") {
            let right = self.r_expr_v3();
            res = ast::ASTExprCall::new("@op:eq".to_string(), vec![res, right]);
        }
        res
    }

    // expr_v3: expr_v2 (('<' | '>') expr_v2)*
    fn r_expr_v3(&mut self) -> ast::ASTExprPtr {
        let mut res = self.r_expr_v2();

        loop {
            let fname = match self.ps.peek_token() {
                Token::Symbol(x) if x == "<" => Some("@op:lt"),
                Token::Symbol(x) if x == ">" => Some("@op:gt"),
                _ => None,
            };

            if fname.is_none() {
                break;
            }
            let fname = fname.unwrap();
            self.ps.get_token();
            let right = self.r_expr_v2();
            res = ast::ASTExprCall::new(fname.to_string(), vec![res, right]);
        }

        res
    }

    // expr_v2: expr_v1 (('+' | '-') expr_v1)*
    fn r_expr_v2(&mut self) -> ast::ASTExprPtr {
        let mut res = self.r_expr_v1();

        loop {
            let fname = match self.ps.peek_token() {
                Token::Symbol(x) if x == "+" => Some("@op:add"),
                Token::Symbol(x) if x == "-" => Some("@op:sub"),
                _ => None,
            };

            if fname.is_none() {
                break;
            }
            let fname = fname.unwrap();
            self.ps.get_token();
            let right = self.r_expr_v1();
            res = ast::ASTExprCall::new(fname.to_string(), vec![res, right]);
        }

        res
    }

    // expr_v1: expr_vunop (('*' | '/' | '%') expr_vunop)*
    fn r_expr_v1(&mut self) -> ast::ASTExprPtr {
        let mut res = self.r_expr_vunop();

        loop {
            let fname = match self.ps.peek_token() {
                Token::Symbol(x) if x == "*" => Some("@op:mul"),
                Token::Symbol(x) if x == "/" => Some("@op:div"),
                Token::Symbol(x) if x == "%" => Some("@op:mod"),
                _ => None,
            };

            if fname.is_none() {
                break;
            }
            let fname = fname.unwrap();
            self.ps.get_token();
            let right = self.r_expr_vunop();
            res = ast::ASTExprCall::new(fname.to_string(), vec![res, right]);
        }

        res
    }

    // expr_vunop:  expr_vprim
    //            | ('+' | '-' | '!') expr_vunop
    fn r_expr_vunop(&mut self) -> ast::ASTExprPtr {
        match self.ps.peek_token() {
            Token::Symbol(x) if x == "+" => {
                self.ps.get_token();
                self.r_expr_vunop()
            }

            Token::Symbol(x) if x == "-" => {
                self.ps.get_token();
                ast::ASTExprCall::new("@op:neg".to_string(), vec![self.r_expr_vunop()])
            }

            Token::Symbol(x) if x == "!" => {
                self.ps.get_token();
                ast::ASTExprCall::new("@op:not".to_string(), vec![self.r_expr_vunop()])
            }

            _ => self.r_expr_vprim(),
        }
    }

    // expr_vprim:  expr_vatom
    //            | expr_vprim '(' expr_list<','> ')'
    fn r_expr_vprim(&mut self) -> ast::ASTExprPtr {
        let mut res = self.r_expr_vatom();
        loop {
            match self.ps.peek_token() {
                Token::Symbol(x) if x == "(" => {
                    self.ps.get_token();
                    let args = self.r_expr_list(",");
                    self.ps.eat_sym(")");

                    let name = match ASTStatic::resolve(&res) {
                        ASTStatic::ExprId(x) => x,
                        _ => panic!("r:expr: callee must be an id"),
                    };

                    res = ast::ASTExprCall::new(name, args);
                }

                _ => break,
            }
        }

        res
    }

    // expr_vatom:  '(' expr_list<';'> ')'
    //            | @int
    //	          | @id
    fn r_expr_vatom(&mut self) -> ast::ASTExprPtr {
        let tok = self.ps.get_token();
        match &tok {
            Token::Symbol(x) if x == "(" => {
                let res = self.r_expr_list(";");
                self.ps.eat_sym(")");
                ast::ASTExprBlock::new(res)
            }

            Token::ValInt(x) => ast::ASTExprConst::new(*x as i32),
            Token::Id(x) => ast::ASTExprId::new(x.to_string()),
            _ => panic!("r:expr_vatom: Invalid token {:?}", tok),
        }
    }

    // expr_list<sep>:  expr (sep expr)*
    //                | @empty
    fn r_expr_list(&mut self, sep: &str) -> Vec<ast::ASTExprPtr> {
        let mut has_sep = false;
        let mut res = vec![];

        loop {
            match self.ps.peek_token() {
                Token::Symbol(x) if x == ")" => {
                    if has_sep {
                        panic!("Invalid end of expression list after ',' symbol");
                    }
                    break;
                }
                _ => {}
            }

            res.push(self.r_expr());
            has_sep = self.ps.try_eat_sym(sep);
        }

        res
    }

    // def:  def_var
    //     | def_fun
    fn r_def(&mut self) -> ast::ASTDefPtr {
        match self.ps.peek_token() {
            Token::Keyword(x) if x == "var" => self.r_def_var(),
            Token::Keyword(x) if x == "fun" => self.r_def_fun(),
            _ => panic!(
                "r:def: invalid keyword, expected var ou fun, got '{:?}'",
                self.ps.peek_token()
            ),
        }
    }

    // def_var: 'var' @id ':' type '=' expr
    fn r_def_var(&mut self) -> Box<ast::ASTDefVar> {
        self.ps.eat_keyword("var");
        let name = self.ps.eat_id();
        self.ps.eat_sym(":");
        let var_type = self.r_type();
        self.ps.eat_sym("=");
        let val = self.r_expr();
        ast::ASTDefVar::new(name, Some(var_type), val)
    }

    // def_fun: 'fun' @id '(' def_fun_args ')' ':' type '=' expr
    fn r_def_fun(&mut self) -> Box<ast::ASTDefFun> {
        self.ps.eat_keyword("fun");
        let name = self.ps.eat_id();
        self.ps.eat_sym("(");
        let args = self.r_def_fun_args();
        self.ps.eat_sym(")");
        self.ps.eat_sym(":");
        let ret_type = self.r_type();
        self.ps.eat_sym("=");
        let body = self.r_expr();
        ast::ASTDefFun::new(name, args, ret_type, body)
    }

    // def_fun_args:  def_fun_arg ( ',' def_fun_arg )*
    //              | @empty
    //
    // def_fun_arg: @id ':' type
    fn r_def_fun_args(&mut self) -> Vec<Box<ast::ASTDefArg>> {
        let mut has_sep = false;
        let mut res = vec![];

        loop {
            match self.ps.peek_token() {
                Token::Symbol(x) if x == ")" => {
                    if has_sep {
                        panic!("Invalid end of arguments list after symbol ','");
                    }
                    break;
                }
                _ => {}
            }

            let arg_name = self.ps.eat_id();
            self.ps.eat_sym(":");
            let arg_type = self.r_type();
            res.push(ast::ASTDefArg::new(arg_name, arg_type));
            has_sep = self.ps.try_eat_sym(",");
        }

        res
    }

    // type: @id
    fn r_type(&mut self) -> ast::ASTTypePtr {
        let name = self.ps.eat_id();
        ast::ASTTypeName::new(name)
    }
}
