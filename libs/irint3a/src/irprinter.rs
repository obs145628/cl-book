use crate::ir;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;

pub trait CodePrintable {
    fn print_code(&self, writer: &mut dyn Write);
}

impl CodePrintable for ir::Module {
    fn print_code(&self, writer: &mut dyn Write) {
        let mut printer = IRPrinter::new();
        printer.print_mod(self, writer);
    }
}

pub struct IRPrinter {
    local_labels: HashMap<usize, usize>, //label (val) for each instruction index (key)
    fn_names: HashMap<usize, usize>,     //label name (val) for each function address (key)
}

impl IRPrinter {
    fn new() -> Self {
        IRPrinter {
            local_labels: HashMap::new(),
            fn_names: HashMap::new(),
        }
    }

    fn print_mod(&mut self, m: &ir::Module, writer: &mut dyn Write) {
        self.prepare_mod(m);
        for fun in m.defs() {
            self.print_fun(fun, writer);
            write!(writer, "\n").unwrap();
        }
    }

    fn prepare_mod(&mut self, m: &ir::Module) {
        // 1) Create label for all functions
        for (label, fun) in m.defs().iter().enumerate() {
            self.fn_names.insert(fun.addr().0, label);
        }
    }

    fn prepare_fun(&mut self, fun: &ir::DefFun) {
        // 1) create labels for all instructions we can jump to
        let mut ins_targets: HashSet<usize> = HashSet::new();
        for ins in fun.body().unwrap().iter() {
            match ins {
                ir::Ins::Jump(ins) => {
                    ins_targets.insert(ins.label().0);
                }
                ir::Ins::Br(ins) => {
                    ins_targets.insert(ins.label_true().0);
                    ins_targets.insert(ins.label_false().0);
                }
                _ => {}
            }
        }

        self.local_labels = HashMap::new();
        for (label_id, ins_id) in ins_targets.iter().enumerate() {
            self.local_labels.insert(*ins_id, label_id);
        }
    }

    fn print_fun(&mut self, f: &ir::DefFun, writer: &mut dyn Write) {
        self.prepare_fun(f);
        let body = f.body();
        let fn_id = f.addr().0;
        let fn_name = self.fn_names.get(&fn_id).unwrap();
        write!(writer, "define {} F{} ", fn_id, fn_name).unwrap();
        match body {
            Some(body) => {
                write!(writer, "{{\n").unwrap();
                self.print_body(body, writer);
                write!(writer, "}}\n").unwrap();
            }
            _ => write!(writer, "extern\n").unwrap(),
        }
    }

    fn print_body(&mut self, body: &Vec<ir::Ins>, writer: &mut dyn Write) {
        for (id, ins) in body.iter().enumerate() {
            write!(writer, "  ").unwrap();
            match self.local_labels.get(&id) {
                Some(ins_label) => write!(writer, "L{}: ", ins_label).unwrap(),
                _ => {}
            }

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
            ir::InsCmpbinKind::Eq => "compeq",
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
        let label_id = self.local_labels.get(&ins.label().0).unwrap();
        write!(writer, "jump L{}", label_id).unwrap();
    }

    fn print_ins_br(&self, ins: &ir::InsBr, writer: &mut dyn Write) {
        let label_true = self.local_labels.get(&ins.label_true().0).unwrap();
        let label_false = self.local_labels.get(&ins.label_false().0).unwrap();
        write!(
            writer,
            "br {}, L{}, L{}",
            ins.src().0,
            label_true,
            label_false
        )
        .unwrap();
    }

    fn print_ins_call(&self, ins: &ir::InsCall, writer: &mut dyn Write) {
        let fn_name = self.fn_names.get(&ins.fun().0).unwrap();
        write!(writer, "call %{}, F{}", ins.dst().0, fn_name).unwrap();
        for arg in ins.args() {
            write!(writer, ", %{}", arg.0).unwrap();
        }
    }

    fn print_ins_ret(&self, ins: &ir::InsRet, writer: &mut dyn Write) {
        write!(writer, "ret %{}", ins.src().0).unwrap();
    }
}
