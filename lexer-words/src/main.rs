
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
    t.add_word("hello");
    t.add_word("pop");
    t.add_word("heli");
    t.add_word("hel");
}
