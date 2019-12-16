
mod stream;
mod trie;
use stream::Stream;
use trie::Trie;

fn main() {

    let mut is = Stream::new("ex/p1.txt");

    while !is.eof() {
	println!("[{}]", is.get_char());
	is.next_char();
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
