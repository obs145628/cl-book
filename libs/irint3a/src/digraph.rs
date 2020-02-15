use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// Directed Graph, based on an adjency-list representation
// Fixed number of vertices, can only add edges
// Doesn't check for self loop and parallel edges
pub struct Digraph {
    vcount: usize,
    ecount: usize,
    adjs: Vec<Vec<usize>>,
}

impl Digraph {
    // Create a new graph with `vcount` vertices and no edge
    pub fn new(vcount: usize) -> Self {
        Digraph {
            vcount,
            ecount: 0,
            adjs: vec![vec![]; vcount],
        }
    }

    // Returns the number of vertices in the graph
    pub fn vcount(&self) -> usize {
        self.vcount
    }

    // Returns the number of edges in the graph
    pub fn ecount(&self) -> usize {
        self.ecount
    }

    // Add a new directed edge v -> w
    pub fn add_edge(&mut self, v: usize, w: usize) {
        assert!(v < self.vcount);
        assert!(w < self.vcount);
        self.adjs[v].push(w);
        self.ecount += 1;
    }

    // Returns an iterator over all vertices adjacent to vertex `v`
    pub fn adj(&self, v: usize) -> std::slice::Iter<usize> {
        assert!(v < self.vcount);
        let alist = &self.adjs[v];
        alist.as_slice().iter()
    }

    // Save the graph to dot format in the file `path`
    // `gname` optional graph name, g otherwhise
    // `vnames` optional map of names for every vertices.
    // If none, or not defined for one vertex, name is simply the index
    pub fn write_dot(
        &self,
        path: &str,
        gname: Option<&str>,
        vnames: Option<&HashMap<usize, String>>,
    ) {
        let gname = gname.unwrap_or("g");
        let base_names = HashMap::new();
        let vnames = vnames.unwrap_or(&base_names);
        let vnames: Vec<String> = (0..self.vcount())
            .map(|x| match vnames.get(&x) {
                Some(name) => name.to_string(),
                None => format!("{}", x),
            })
            .collect();

        let mut os = File::create(path).expect("Failed to create output dot file");

        write!(os, "digraph {} {{\n", gname).unwrap();

        for v in 0..self.vcount() {
            for w in self.adj(v) {
                write!(os, "  {} -> {};\n", vnames[v], vnames[*w]).unwrap();
            }
        }

        write!(os, "}}\n").unwrap();
    }
}
