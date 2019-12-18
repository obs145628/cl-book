
mod stream;
mod trie;
mod token;
mod lexer;
use stream::Stream;
use trie::Trie;
use lexer::Lexer;

fn main() {

    let keywords = vec!["for"];
    let syms = vec!["+", "-", "*", "/"];
    let mut lex = Lexer::new(Stream::new("ex/p1.txt"), keywords, syms);
    loop {
	let tok = lex.get();
	println!("{:?}", tok);
	if tok.is_eof() {
	    break
	}
    }

    let mut t = Trie::new();
    t.add_word("abc");
    t.add_word("dg");
    t.prepare();

    t.state_reset();
    t.state_consume('a');
    println!("a: word={}, inTrie={}", t.state_is_word(), t.state_in_trie());
    t.state_consume('b');
    println!("ab: word={}, inTrie={}", t.state_is_word(), t.state_in_trie());
    t.state_consume('c');
    println!("abc: word={}, inTrie={}", t.state_is_word(), t.state_in_trie());
    t.state_consume('d');
    println!("abcd: word={}, inTrie={}", t.state_is_word(), t.state_in_trie());
}
