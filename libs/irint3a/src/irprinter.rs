use crate::ir;

use std::io::Write;

pub trait CodePrintable {
    fn print_code(&self, writer: &mut dyn Write);
}

impl CodePrintable for ir::ModuleExtended {
    fn print_code(&self, writer: &mut dyn Write) {
        let mut printer = IRPrinter::new(self);
        printer.print_mod(writer);
    }
}

struct IRPrinter<'a> {
    module_ex: &'a ir::ModuleExtended,
    module: &'a ir::Module,
    fun: Option<&'a ir::DefFun>,
    fun_ex: Option<&'a ir::FunExtended>,
}

impl<'a> IRPrinter<'a> {
    fn new(module_ex: &'a ir::ModuleExtended) -> Self {
        IRPrinter {
            module_ex,
            module: module_ex.module(),
            fun: None,
            fun_ex: None,
        }
    }

    fn print_mod(&mut self, writer: &mut dyn Write) {
        for fun in self.module.defs() {
            let fun_ex = self.module_ex.get_fun(fun.addr());
            self.fun = Some(fun);
            self.fun_ex = Some(fun_ex);
            self.print_fun(writer);
            write!(writer, "\n").unwrap();
        }
    }

    fn print_fun(&self, writer: &mut dyn Write) {
        let f = self.fun.unwrap();
        let f_ex = self.fun_ex.unwrap();
        let fn_id = f.addr();

        write!(writer, "define {} {} ", fn_id.0, f_ex.name()).unwrap();

        if f.body().is_some() {
            write!(writer, "\n").unwrap();
            self.print_body(writer);
            write!(writer, "\n").unwrap();
        } else {
            write!(writer, "extern\n").unwrap();
        }
    }

    fn print_body(&self, writer: &mut dyn Write) {
        let f = self.fun.unwrap();
        let f_ex = self.fun_ex.unwrap();

        for (id, ins) in f.body().unwrap().iter().enumerate() {
            if let Some(ins_label) = f_ex.label_of(ir::LocalLabel(id)) {
                if id != 0 {
                    write!(writer, "\n").unwrap();
                }

                write!(writer, "{}:\n", ins_label).unwrap();
            }

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
        write!(writer, "movi %{}, {}", ins.dst().0, ins.const_val()).unwrap();
    }

    fn print_ins_movr(&self, ins: &ir::InsMovr, writer: &mut dyn Write) {
        write!(writer, "movr %{}, %{}", ins.dst().0, ins.src().0).unwrap();
    }

    fn print_ins_load(&self, ins: &ir::InsLoad, writer: &mut dyn Write) {
        write!(writer, "load %{}, %{}", ins.dst().0, ins.src().0).unwrap();
    }

    fn print_ins_store(&self, ins: &ir::InsStore, writer: &mut dyn Write) {
        write!(writer, "store %{}, %{}", ins.dst().0, ins.src().0).unwrap();
    }

    fn print_ins_alloca(&self, ins: &ir::InsAlloca, writer: &mut dyn Write) {
        write!(writer, "alloca %{}", ins.dst().0).unwrap();
    }

    fn print_ins_opbin(&self, ins: &ir::InsOpbin, writer: &mut dyn Write) {
        let ins_name = match ins.kind() {
            ir::InsOpbinKind::Add => "add",
            ir::InsOpbinKind::Sub => "sub",
            ir::InsOpbinKind::Mul => "mul",
            ir::InsOpbinKind::Div => "div",
            ir::InsOpbinKind::Mod => "mod",
        };

        write!(
            writer,
            "{} %{}, %{}, %{}",
            ins_name,
            ins.dst().0,
            ins.src1().0,
            ins.scr2().0
        )
        .unwrap();
    }

    fn print_ins_cmpbin(&self, ins: &ir::InsCmpbin, writer: &mut dyn Write) {
        let ins_name = match ins.kind() {
            ir::InsCmpbinKind::Eq => "cmpeq",
            ir::InsCmpbinKind::Gt => "cpmgt",
            ir::InsCmpbinKind::Lt => "cmplt",
        };

        write!(
            writer,
            "{} %{}, %{}, %{}",
            ins_name,
            ins.dst().0,
            ins.src1().0,
            ins.scr2().0
        )
        .unwrap();
    }

    fn print_ins_jump(&self, ins: &ir::InsJump, writer: &mut dyn Write) {
        let f_ex = self.fun_ex.unwrap();
        let label_name = f_ex.label_of(ins.label()).unwrap();
        write!(writer, "jump {}", label_name).unwrap();
    }

    fn print_ins_br(&self, ins: &ir::InsBr, writer: &mut dyn Write) {
        let f_ex = self.fun_ex.unwrap();
        let label_true = f_ex.label_of(ins.label_true()).unwrap();
        let label_false = f_ex.label_of(ins.label_false()).unwrap();
        write!(
            writer,
            "br %{}, {}, {}",
            ins.src().0,
            label_true,
            label_false
        )
        .unwrap();
    }

    fn print_ins_call(&self, ins: &ir::InsCall, writer: &mut dyn Write) {
        let fn_name = self.module_ex.get_fun(ins.fun()).name();
        write!(writer, "call %{}, {}", ins.dst().0, fn_name).unwrap();
        for arg in ins.args() {
            write!(writer, ", %{}", arg.0).unwrap();
        }
    }

    fn print_ins_ret(&self, ins: &ir::InsRet, writer: &mut dyn Write) {
        write!(writer, "ret %{}", ins.src().0).unwrap();
    }
}
