use crate::ir;
use crate::irvalidation;

// basic block in construction
struct BasicBlockBuilder {
    id: ir::BasicBlockRef,
    fun: ir::FunctionRef,
    ins: Vec<ir::Ins>,
    complete: bool,
}

impl BasicBlockBuilder {
    fn new(id: ir::BasicBlockRef, fun: ir::FunctionRef) -> Self {
        BasicBlockBuilder {
            id,
            fun,
            ins: vec![],
            complete: false,
        }
    }

    fn add_ins(&mut self, ins: ir::Ins) {
        if self.complete {
            panic!("Basic block already completed");
        }
        self.ins.push(ins);
    }

    fn finish(self) -> ir::BasicBlock {
        if !self.complete {
            panic!("Basic block not completed yet");
        }
        ir::BasicBlock::new(self.id, self.ins)
    }
}

// function in construction
struct FunctionBuilder {
    id: ir::FunctionRef,
    bbs: Vec<BasicBlockBuilder>,
}

impl FunctionBuilder {
    fn new(id: ir::FunctionRef) -> Self {
        FunctionBuilder { id, bbs: vec![] }
    }

    fn finish(mut self) -> ir::Function {
        let mut bbs = vec![];
        std::mem::swap(&mut bbs, &mut self.bbs);
        let bbs: Vec<_> = bbs.into_iter().map(|x| x.finish()).collect();
        let bbs = if bbs.len() == 0 { None } else { Some(bbs) };

        ir::Function::new(self.id, bbs)
    }
}
