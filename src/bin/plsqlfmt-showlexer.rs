extern crate plsqlfmt;

fn try_lexer(input: &str) {
    println!("'{}' -> {:?}", input, plsqlfmt::lexer::read_str(input));

}

fn main() {
    println!("Lexing:");
    
    try_lexer("SELECT 1 FROM dual;");
    try_lexer(r#"SELECT
1
 FROM dual;"#);
    try_lexer("SELECT 1 x FROM dual;");
    try_lexer("SELECT 1 AS x FROM dual;");
    try_lexer("SELECT 'Hi' FROM dual;");
    try_lexer("INSERT INTO foo (baz, bar) VALUES (1, 'Hi');");
}
