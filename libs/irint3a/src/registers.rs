/// Utils to compute informations about Instructions and registers
use std::collections::HashSet;

use crate::ir;

/// Know the registers used (as src or dst) by any instructions
pub trait GetRegistersUse {
    fn get_register_use(&self, out_regs: &mut HashSet<ir::RegId>);
}

impl GetRegistersUse for ir::Ins {
    fn get_register_use(&self, out_regs: &mut HashSet<ir::RegId>) {
        match self {
            ir::Ins::Movi(ins) => ins.get_register_use(out_regs),
            ir::Ins::Movr(ins) => ins.get_register_use(out_regs),
            ir::Ins::Load(ins) => ins.get_register_use(out_regs),
            ir::Ins::Store(ins) => ins.get_register_use(out_regs),
            ir::Ins::Alloca(ins) => ins.get_register_use(out_regs),
            ir::Ins::Opbin(ins) => ins.get_register_use(out_regs),
            ir::Ins::Cmpbin(ins) => ins.get_register_use(out_regs),
            ir::Ins::Jump(ins) => ins.get_register_use(out_regs),
            ir::Ins::Br(ins) => ins.get_register_use(out_regs),
            ir::Ins::Call(ins) => ins.get_register_use(out_regs),
            ir::Ins::Ret(ins) => ins.get_register_use(out_regs),
        }
    }
}

impl GetRegistersUse for ir::InsMovi {
    fn get_register_use(&self, out_regs: &mut HashSet<ir::RegId>) {
        out_regs.insert(self.dst());
    }
}

impl GetRegistersUse for ir::InsMovr {
    fn get_register_use(&self, out_regs: &mut HashSet<ir::RegId>) {
        out_regs.insert(self.src());
        out_regs.insert(self.dst());
    }
}

impl GetRegistersUse for ir::InsLoad {
    fn get_register_use(&self, out_regs: &mut HashSet<ir::RegId>) {
        out_regs.insert(self.src());
        out_regs.insert(self.dst());
    }
}

impl GetRegistersUse for ir::InsStore {
    fn get_register_use(&self, out_regs: &mut HashSet<ir::RegId>) {
        out_regs.insert(self.src());
        out_regs.insert(self.dst());
    }
}

impl GetRegistersUse for ir::InsAlloca {
    fn get_register_use(&self, out_regs: &mut HashSet<ir::RegId>) {
        out_regs.insert(self.dst());
    }
}

impl GetRegistersUse for ir::InsOpbin {
    fn get_register_use(&self, out_regs: &mut HashSet<ir::RegId>) {
        out_regs.insert(self.src1());
        out_regs.insert(self.src2());
        out_regs.insert(self.dst());
    }
}

impl GetRegistersUse for ir::InsCmpbin {
    fn get_register_use(&self, out_regs: &mut HashSet<ir::RegId>) {
        out_regs.insert(self.src1());
        out_regs.insert(self.src2());
        out_regs.insert(self.dst());
    }
}

impl GetRegistersUse for ir::InsJump {
    fn get_register_use(&self, _out_regs: &mut HashSet<ir::RegId>) {}
}

impl GetRegistersUse for ir::InsBr {
    fn get_register_use(&self, out_regs: &mut HashSet<ir::RegId>) {
        out_regs.insert(self.src());
    }
}

impl GetRegistersUse for ir::InsCall {
    fn get_register_use(&self, out_regs: &mut HashSet<ir::RegId>) {
        for arg in self.args() {
            out_regs.insert(*arg);
        }
        out_regs.insert(self.dst());
    }
}

impl GetRegistersUse for ir::InsRet {
    fn get_register_use(&self, out_regs: &mut HashSet<ir::RegId>) {
        out_regs.insert(self.src());
    }
}
