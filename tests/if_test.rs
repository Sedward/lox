use std::path::Path;
use std::fs;

#[test]
fn lexer_tokenizes_if() {
    let path = Path::new("./if/if.lox");
    let _contents = fs::read_to_string(path)
        .expect("problem reading if.lox");

}