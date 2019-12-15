
pub struct Trie
{
    root: Box<TrieNode>,
    ready: bool,
}

struct TrieNode
{
    c: char,
    is_word: bool,
    children: Vec<Box<TrieNode>>,
}

impl Trie {

    pub fn new() -> Trie {
	Trie {
	    root: TrieNode::new(0 as char, false),
	    ready: false
	}
    }

    pub fn add_word(&mut self, w: &str) {
	assert!(!self.ready);
	self.root.add_word(w);
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

	println!("add word '{}'", w);

	match w.chars().next() {
	    None => {
		self.is_word = true;
	    }

	    Some(c) => {
		let child = self.get_child(c);
		let rest = &w[1..];
		child.add_word(rest);
	    }
	}
    }
    
}
