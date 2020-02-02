use std::io::Write;

use crate::ir;
use crate::irnames;

pub trait CodePrintable {
    fn print_code(&self, writer: &mut dyn Write, names: Option<&irnames::ModuleNames>);
}

impl CodePrintable for ir::Module {
    fn print_code(&self, writer: &mut dyn Write, names: Option<&irnames::ModuleNames>) {
        let mut gen_names;

        let names = match names {
            Some(names) => names,
            None => {
                gen_names = Some(irnames::ModuleNames::new());
                let names = gen_names.as_mut().unwrap();
                names.complete_undefined(self);
                gen_names.as_ref().unwrap()
            }
        };

        let mut printer = IRPrinter::new(self, names);
        printer.print_mod(writer);
    }
}

struct IRPrinter<'a> {
    module: &'a ir::Module,
    names: &'a irnames::ModuleNames,
    fun: Option<&'a ir::Function>,
    fun_names: Option<&'a irnames::FunctionNames>,
}

impl<'a> IRPrinter<'a> {
    fn new(module: &'a ir::Module, names: &'a irnames::ModuleNames) -> Self {
        IRPrinter {
            module,
            names,
            fun: None,
            fun_names: None,
        }
    }

    fn print_mod(&mut self, writer: &mut dyn Write) {
        for fun in self.module.funs() {
            self.fun = Some(fun);
            self.fun_names = Some(self.names.get_function(fun.id()).unwrap());
            self.print_fun(writer);
            write!(writer, "\n").unwrap();
        }
    }

    fn print_fun(&self, writer: &mut dyn Write) {
        let fun = self.fun.unwrap();
        let fun_id = fun.id();

        let define_kw = if fun.is_extern() { "declare" } else { "define" };

        write!(
            writer,
            ".{} {} {} ",
            define_kw,
            fun_id.0,
            self.names.get_function_name(fun_id).unwrap()
        )
        .unwrap();

        if !fun.is_extern() {
            write!(writer, "\n").unwrap();
            self.print_body(writer);
        }
        write!(writer, "\n").unwrap();
    }

    fn print_body(&self, writer: &mut dyn Write) {
        let fun = self.fun.unwrap();
        let fun_names = self.fun_names.unwrap();

        for bb_id in fun.basic_blocks_list() {
            let bb = fun.get_basic_block(*bb_id);
            write!(
                writer,
                "{}:\n",
                fun_names.get_basic_block_name(*bb_id).unwrap()
            )
            .unwrap();
            self.print_bb(bb, writer);
            write!(writer, "\n").unwrap();
        }
    }

    fn print_bb(&self, bb: &ir::BasicBlock, writer: &mut dyn Write) {
        for ins in bb.iter() {
            write!(writer, "  ").unwrap();
            self.print_ins(ins, writer);
            write!(writer, "\n").unwrap();
        }
    }

    fn print_ins(&self, ins: &ir::Ins, writer: &mut dyn Write) {
        match ins {
            ir::Ins::Movi(ins) => self.print_ins_movi(&ins, writer),
            ir::Ins::Movr(ins) => self.print_ins_movr(&ins, writer),
            ir::Ins::Load(ins) => self.print_ins_load(&ins, writer),
            ir::Ins::Store(ins) => self.print_ins_store(&ins, writer),
            ir::Ins::Alloca(ins) => self.print_ins_alloca(&ins, writer),
            ir::Ins::Opbin(ins) => self.print_ins_opbin(&ins, writer),
            ir::Ins::Cmpbin(ins) => self.print_ins_cmpbin(&ins, writer),
            ir::Ins::Jump(ins) => self.print_ins_jump(&ins, writer),
            ir::Ins::Br(ins) => self.print_ins_br(&ins, writer),
            ir::Ins::Call(ins) => self.print_ins_call(&ins, writer),
            ir::Ins::Ret(ins) => self.print_ins_ret(&ins, writer),
        }
    }

    fn print_ins_movi(&self, ins: &ir::InsMovi, writer: &mut dyn Write) {
        let fun_names = self.fun_names.unwrap();
        let dst = fun_names.get_register_name(ins.dst()).unwrap();
        write!(writer, "movi %{}, {}", dst, ins.const_val()).unwrap();
    }

    fn print_ins_movr(&self, ins: &ir::InsMovr, writer: &mut dyn Write) {
        let fun_names = self.fun_names.unwrap();
        let src = fun_names.get_register_name(ins.src()).unwrap();
        let dst = fun_names.get_register_name(ins.dst()).unwrap();
        write!(writer, "movr %{}, %{}", dst, src).unwrap();
    }

    fn print_ins_load(&self, ins: &ir::InsLoad, writer: &mut dyn Write) {
        let fun_names = self.fun_names.unwrap();
        let src = fun_names.get_register_name(ins.src()).unwrap();
        let dst = fun_names.get_register_name(ins.dst()).unwrap();
        write!(writer, "load %{}, %{}", dst, src).unwrap();
    }

    fn print_ins_store(&self, ins: &ir::InsStore, writer: &mut dyn Write) {
        let fun_names = self.fun_names.unwrap();
        let src = fun_names.get_register_name(ins.src()).unwrap();
        let dst = fun_names.get_register_name(ins.dst()).unwrap();
        write!(writer, "store %{}, %{}", dst, src).unwrap();
    }

    fn print_ins_alloca(&self, ins: &ir::InsAlloca, writer: &mut dyn Write) {
        let fun_names = self.fun_names.unwrap();
        let dst = fun_names.get_register_name(ins.dst()).unwrap();
        write!(writer, "alloca %{}", dst).unwrap();
    }

    fn print_ins_opbin(&self, ins: &ir::InsOpbin, writer: &mut dyn Write) {
        let fun_names = self.fun_names.unwrap();
        let src1 = fun_names.get_register_name(ins.src1()).unwrap();
        let src2 = fun_names.get_register_name(ins.src2()).unwrap();
        let dst = fun_names.get_register_name(ins.dst()).unwrap();

        let ins_name = match ins.kind() {
            ir::InsOpbinKind::Add => "add",
            ir::InsOpbinKind::Sub => "sub",
            ir::InsOpbinKind::Mul => "mul",
            ir::InsOpbinKind::Div => "div",
            ir::InsOpbinKind::Mod => "mod",
        };

        write!(writer, "{} %{}, %{}, %{}", ins_name, dst, src1, src2).unwrap();
    }

    fn print_ins_cmpbin(&self, ins: &ir::InsCmpbin, writer: &mut dyn Write) {
        let fun_names = self.fun_names.unwrap();
        let src1 = fun_names.get_register_name(ins.src1()).unwrap();
        let src2 = fun_names.get_register_name(ins.src2()).unwrap();
        let dst = fun_names.get_register_name(ins.dst()).unwrap();

        let ins_name = match ins.kind() {
            ir::InsCmpbinKind::Eq => "cmpeq",
            ir::InsCmpbinKind::Gt => "cmpgt",
            ir::InsCmpbinKind::Lt => "cmplt",
        };

        write!(writer, "{} %{}, %{}, %{}", ins_name, dst, src1, src2,).unwrap();
    }

    fn print_ins_jump(&self, ins: &ir::InsJump, writer: &mut dyn Write) {
        let fun_names = self.fun_names.unwrap();
        let dst_name = fun_names.get_basic_block_name(ins.dst()).unwrap();
        write!(writer, "jump {}", dst_name).unwrap();
    }

    fn print_ins_br(&self, ins: &ir::InsBr, writer: &mut dyn Write) {
        let fun_names = self.fun_names.unwrap();
        let src = fun_names.get_register_name(ins.src()).unwrap();
        let dst_true_name = fun_names.get_basic_block_name(ins.dst_true()).unwrap();
        let dst_false_name = fun_names.get_basic_block_name(ins.dst_false()).unwrap();

        write!(writer, "br %{}, {}, {}", src, dst_true_name, dst_false_name).unwrap();
    }

    fn print_ins_call(&self, ins: &ir::InsCall, writer: &mut dyn Write) {
        let fun_names = self.fun_names.unwrap();
        let dst = fun_names.get_register_name(ins.dst()).unwrap();
        let fun = self.names.get_function_name(ins.fun()).unwrap();

        write!(writer, "call %{}, {}", dst, fun).unwrap();
        for arg in ins.args() {
            let arg = fun_names.get_register_name(*arg).unwrap();
            write!(writer, ", %{}", arg).unwrap();
        }
    }

    fn print_ins_ret(&self, ins: &ir::InsRet, writer: &mut dyn Write) {
        let fun_names = self.fun_names.unwrap();
        let src = fun_names.get_register_name(ins.src()).unwrap();
        write!(writer, "ret %{}", src).unwrap();
    }
}
