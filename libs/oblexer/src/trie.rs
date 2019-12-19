
pub struct Trie
{
    root: Box<TrieNode>,
    ready: bool,
    nodes: Vec<OptiNode>,
    cur_node: usize,
}

struct TrieNode
{
    c: char,
    is_word: bool,
    children: Vec<Box<TrieNode>>,
}

struct OptiNode
{
    c: char,
    is_word: bool,
    children: Vec<usize>
}


impl Trie {

    pub fn new() -> Trie {
	Trie {
	    root: TrieNode::new(0 as char, false),
	    ready: false,
	    nodes: Vec::new(),
	    cur_node: std::usize::MAX,
	}
    }

    pub fn from_words(words: Vec<&str>) -> Trie {
	let mut res = Trie::new();
	for w in words {
	    res.add_word(w);
	}
	res.prepare();
	res.state_reset();
	res
    }

    pub fn add_word(&mut self, w: &str) {
	assert!(!self.ready);
	self.root.add_word(w);
    }

    /// Compile the trie, ready to accept new requests
    pub fn prepare(&mut self) {
	assert!(!self.ready);
	self.ready = true;
	Trie::create_opti_node(&mut self.nodes, &self.root);
    }

    pub fn can_start_with(&self, c: char) -> bool {
	let node = &self.nodes.last().unwrap();
	for child in &node.children {
	    if self.nodes[*child].c == c {
		return true;
	    }
	}

	false
    }

    /// Reset the actual state token to the root
    pub fn state_reset(&mut self) {
	assert!(self.ready);
	self.cur_node = self.nodes.len() - 1;
    }

    /// Check if current state left the trie
    pub fn state_in_trie(&self) -> bool {
	assert!(self.ready);
	self.cur_node != std::usize::MAX
    }

    /// Return if the current state token forms a word
    pub fn state_is_word(&self) -> bool {
	assert!(self.ready);
	if !self.state_in_trie() {
	    return false;
	};
	self.nodes[self.cur_node].is_word
    }

    /// Change current state by adding a char
    pub fn state_consume(&mut self, c: char) {
	assert!(self.ready);
	if !self.state_in_trie() {
	    return;
	};

	let node = &self.nodes[self.cur_node];
	for child in &node.children {
	    if self.nodes[*child].c == c {
		self.cur_node = *child;
		return;
	    }
	};

	self.cur_node = std::usize::MAX;
    }

    fn create_opti_node(res: &mut Vec<OptiNode>, node: &Box<TrieNode>) -> usize {
	let mut onode = OptiNode {
	    c: node.c,
	    is_word: node.is_word,
	    children: Vec::new()
	};
	
	for child in &node.children {
	   onode.children.push(Trie::create_opti_node(res, &child));
	};
	let idx = res.len();
	res.push(onode);
	idx
    }

}


impl TrieNode
{

    fn new(c: char, is_word: bool) -> Box<TrieNode> {
	Box::new(TrieNode {
	    c,
	    is_word,
	    children: Vec::new(),
	})
    }
    

    fn get_child(&mut self, c: char) -> &mut Box<TrieNode> {

	let mut id: usize = std::usize::MAX;
	
	for (i, child) in self.children.iter().enumerate() {
	    if child.c == c {
		id = i;
		break;
	    }
	}

	if id == std::usize::MAX
	{
	    id = self.children.len();
	    self.children.push(TrieNode::new(c, false));
	}

	&mut self.children[id]
    }

    fn add_word(&mut self, w: &str)
    {

	match w.chars().next() {
	    None => {
		self.is_word = true;
	    }

	    Some(c) => {
		let child = self.get_child(c);
		let rest = &w[c.len_utf8()..];
		child.add_word(rest);
	    }
	}
    }
    
}
