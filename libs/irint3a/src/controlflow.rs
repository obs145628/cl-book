use crate::digraph::Digraph;
use crate::ir;

fn add_edge_once(g: &mut Digraph, v: usize, w: usize) {
    if !g.adj(v).any(|x| *x == w) {
        g.add_edge(v, w);
    }
}

// Build a control flow graph (CFG)
// Directed graph G=(N,E)
// Each Node n_i in N is a basic block
// Each vertex e = (n_i, n_j) in E corresponds to a possible transfer of control
// from block n_i to block n_j
pub fn build_cfg(fun: &ir::Function) -> Digraph {
    let bbs = fun.basic_blocks_list();
    let mut g = Digraph::new(bbs.len());

    for bb_id in bbs {
        let bb = fun.get_basic_block(*bb_id);
        let bb_v = bb_id.0;
        for ins in bb.iter() {
            match ins {
                ir::Ins::Jump(ins) => add_edge_once(&mut g, bb_v, ins.dst().0),
                ir::Ins::Br(ins) => {
                    add_edge_once(&mut g, bb_v, ins.dst_true().0);
                    add_edge_once(&mut g, bb_v, ins.dst_false().0);
                }
                _ => {}
            }
        }
    }

    g
}
