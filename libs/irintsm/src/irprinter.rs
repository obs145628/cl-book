use std::collections::HashMap;
use std::io::Write;

use crate::ir;

pub trait CodePrintable {
    fn print_code(&self, writer: &mut dyn Write);
}

impl CodePrintable for ir::Module {
    fn print_code(&self, writer: &mut dyn Write) {
        let mut printer = IRPrinter::new(self);
        printer.print_mod(writer);
    }
}

struct IRPrinter<'a> {
    module: &'a ir::Module,
    fun: Option<&'a ir::Function>,
    bb: Option<&'a ir::BasicBlock>,
    bb_mapping: HashMap<ir::BasicBlockRef, usize>,
}

impl<'a> IRPrinter<'a> {
    pub fn new(module: &'a ir::Module) -> Self {
        IRPrinter {
            module,
            fun: None,
            bb: None,
            bb_mapping: HashMap::new(),
        }
    }

    pub fn print_mod(&mut self, writer: &mut dyn Write) {
        for fun in self.module.fun_list() {
            self.fun = Some(fun);
            self.print_fun(writer);
            write!(writer, "\n").unwrap();
        }
    }

    pub fn print_fun(&mut self, writer: &mut dyn Write) {
        let fun = self.fun.unwrap();
        if fun.is_extern() {
            write!(writer, ".declare {}\n", fun.id()).unwrap();
            return;
        }

        write!(writer, ".define {}\n", fun.id()).unwrap();

        self.bb_mapping.clear();
        for (bb_idx, bb) in fun.bb_list().iter().enumerate() {
            self.bb_mapping.insert(bb.id(), bb_idx);
        }

        for bb in fun.bb_list() {
            self.bb = Some(bb);
            self.print_bb(writer);
            write!(writer, "\n").unwrap();
        }
    }

    pub fn print_bb(&mut self, writer: &mut dyn Write) {
        let bb = self.bb.unwrap();
        let bb_idx = *self.bb_mapping.get(&bb.id()).unwrap();
        write!(writer, "{}:\n", bb_idx).unwrap();

        for ins in bb.ins_list() {
            write!(writer, "  ").unwrap();
            self.print_ins(*ins, writer);
            write!(writer, "\n").unwrap();
        }
    }

    fn print_ins(&self, ins: ir::Ins, writer: &mut dyn Write) {
        match ins {
            ir::Ins::Pop(ins) => self.print_ins_pop(ins, writer),
            ir::Ins::Const(ins) => self.print_ins_const(ins, writer),
            ir::Ins::Load(ins) => self.print_ins_load(ins, writer),
            ir::Ins::Store(ins) => self.print_ins_store(ins, writer),
            ir::Ins::Opbin(ins) => self.print_ins_opbin(ins, writer),
            ir::Ins::Cmpbin(ins) => self.print_ins_cmpbin(ins, writer),
            ir::Ins::Jump(ins) => self.print_ins_jump(ins, writer),
            ir::Ins::Br(ins) => self.print_ins_br(ins, writer),
            ir::Ins::Call(ins) => self.print_ins_call(ins, writer),
            ir::Ins::Ret(ins) => self.print_ins_ret(ins, writer),
        }
    }

    fn print_ins_pop(&self, _ins: ir::InsPop, writer: &mut dyn Write) {
        write!(writer, "pop").unwrap();
    }

    fn print_ins_const(&self, ins: ir::InsConst, writer: &mut dyn Write) {
        write!(writer, "const {}", ins.val()).unwrap();
    }

    fn print_ins_load(&self, ins: ir::InsLoad, writer: &mut dyn Write) {
        write!(writer, "load {}", ins.src()).unwrap();
    }

    fn print_ins_store(&self, ins: ir::InsStore, writer: &mut dyn Write) {
        write!(writer, "const {}", ins.dst()).unwrap();
    }

    fn print_ins_opbin(&self, ins: ir::InsOpbin, writer: &mut dyn Write) {
        write!(
            writer,
            "{}",
            match ins {
                ir::InsOpbin::Add => "add",
                ir::InsOpbin::Sub => "sub",
                ir::InsOpbin::Mul => "mul",
                ir::InsOpbin::Div => "div",
                ir::InsOpbin::Rem => "rem",
            }
        )
        .unwrap();
    }

    fn print_ins_cmpbin(&self, ins: ir::InsCmpbin, writer: &mut dyn Write) {
        write!(
            writer,
            "{}",
            match ins {
                ir::InsCmpbin::Eq => "cmpeq",
                ir::InsCmpbin::Lt => "cmplt",
                ir::InsCmpbin::Gt => "cmpgt",
            }
        )
        .unwrap();
    }

    fn print_ins_jump(&self, ins: ir::InsJump, writer: &mut dyn Write) {
        write!(writer, "jump %{}", ins.dst()).unwrap();
    }

    fn print_ins_br(&self, ins: ir::InsBr, writer: &mut dyn Write) {
        write!(writer, "br %{}, %{}", ins.dst_true(), ins.dst_false()).unwrap();
    }

    fn print_ins_call(&self, ins: ir::InsCall, writer: &mut dyn Write) {
        write!(writer, "call %{}, {}", ins.fun(), ins.nb_args()).unwrap();
    }

    fn print_ins_ret(&self, _ins: ir::InsRet, writer: &mut dyn Write) {
        write!(writer, "ret").unwrap();
    }
}
