use lurlene::Index;

fn main() {
    let mut index = Index::new();
    index.add("foo", "i love emacs");
    let res = index.search("emacs");
    println!("Index: {:?}, Results: {:?}", index, res);
}
