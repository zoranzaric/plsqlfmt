extern crate plsqlfmt;

fn main() {
    println!("{:?}", plsqlfmt::lexer::read_str("SELECT 1 FROM dual;"));
    println!("{:?}", plsqlfmt::lexer::read_str(r#"SELECT
1
 FROM dual;"#));
    println!("{:?}", plsqlfmt::lexer::read_str("SELECT 1 x FROM dual;"));
    println!("{:?}", plsqlfmt::lexer::read_str("SELECT 1 AS x FROM dual;"));
    println!("{:?}", plsqlfmt::lexer::read_str("SELECT 'Hi' FROM dual;"));
    println!("{:?}", plsqlfmt::lexer::read_str("INSERT INTO foo (baz, bar) VALUES (1, 'Hi');"));
}
