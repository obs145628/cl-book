use crate::ast;
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
    pub fn r_file(&mut self) -> ast::ASTExprPtr {
        let res = self.r_expr();
        self.ps.eat_eof();
        res
    }

    //expr:  expr_if
    // | expr_let
    // | expr_while
    // | expr_val
    pub fn r_expr(&mut self) -> ast::ASTExprPtr {
        match self.ps.peek_token() {
            Token::Keyword(x) if x == "if" => self.r_expr_if(),
            Token::Keyword(x) if x == "let" => self.r_expr_let(),
            Token::Keyword(x) if x == "while" => self.r_expr_while(),
            _ => self.r_expr_val(),
        }
    }

    // expr_if: 'if' expr 'then' expr ['else' expr]
    pub fn r_expr_if(&mut self) -> ast::ASTExprPtr {
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
    pub fn r_expr_let(&mut self) -> ast::ASTExprPtr {
        self.ps.eat_keyword("let");
        let mut defs = vec![];
        while !self.ps.try_eat_keyword("in") {
            defs.push(self.r_def());
        }

        let val = self.r_expr();
        ast::ASTExprLet::new(defs, val)
    }

    // expr_while: 'while' expr 'do' expr
    pub fn r_expr_while(&mut self) -> ast::ASTExprPtr {
        self.ps.eat_keyword("while");
        let cond = self.r_expr();
        self.ps.eat_keyword("do");
        let body = self.r_expr();
        ast::ASTExprWhile::new(cond, body)
    }

    // expr_val: expr_v5
    pub fn r_expr_val(&mut self) -> ast::ASTExprPtr {
        self.r_expr_v5()
    }

    // expr_v5:  expr_v4
    //         | expr_v4 '=' expr_v5
    pub fn r_expr_v5(&mut self) -> ast::ASTExprPtr {
        let left = self.r_expr_v4();
        if !self.ps.try_eat_sym("=") {
            return left;
        }
        let right = self.r_expr_v5();
        ast::ASTExprCall::new("@op:set".to_string(), vec![left, right])
    }

    // expr_v4: expr_v3 ('==' expr_v3)*
    pub fn r_expr_v4(&mut self) -> ast::ASTExprPtr {
        let mut res = self.r_expr_v3();
        while self.ps.try_eat_sym("==") {
            let right = self.r_expr_v3();
            res = ast::ASTExprCall::new("@op:eq".to_string(), vec![res, right]);
        }
        res
    }

    // expr_v3: expr_v2 (('<' | '>') expr_v2)*
    pub fn r_expr_v3(&mut self) -> ast::ASTExprPtr {
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
    pub fn r_expr_v2(&mut self) -> ast::ASTExprPtr {
        unimplemented!();
    }

    // expr_v1: expr_vunop (('*' | '/' | '%') expr_vunop)*
    pub fn r_expr_v1(&mut self) -> ast::ASTExprPtr {
        unimplemented!();
    }

    // expr_vunop:  expr_vprim
    //            | ('+' | '-' | '!') expr_vunop
    pub fn r_expr_vunop(&mut self) -> ast::ASTExprPtr {
        unimplemented!();
    }

    // expr_vprim:  expr_vatom
    //            | expr_vprim '(' expr*<,> ')'
    pub fn r_expr_vprim(&mut self) -> ast::ASTExprPtr {
        unimplemented!();
    }

    // expr_vatom:  '(' expr*<;> ')'
    //            | @int
    //	          | @id
    pub fn r_expr_vatom(&mut self) -> ast::ASTExprPtr {
        unimplemented!();
    }

    // def:  def_var
    //     | def_fn
    pub fn r_def(&mut self) -> ast::ASTDefPtr {
        unimplemented!();
    }

    // def_var: 'var' @id ':' type '=' expr
    pub fn r_def_var(&mut self) -> Box<ast::ASTDefVar> {
        unimplemented!();
    }

    // def_fun: 'fun' @id '(' def_fun_args ')' ':' type '=' expr
    pub fn r_def_fun(&mut self) -> Box<ast::ASTDefFun> {
        unimplemented!();
    }

    // def_fun_args:  def_fun_arg ( ',' def_fun_arg )*
    //              | @empty
    //
    // def_fun_arg: @id ':' type
    pub fn r_def_fun_args(&mut self) -> Vec<(String, ast::ASTTypePtr)> {
        unimplemented!();
    }

    // type: @id
    pub fn r_type(&mut self) -> ast::ASTTypePtr {
        unimplemented!();
    }
}
