extern crate failure;

pub mod lexer {
    use failure;

    enum State {
        None,
        StringLiteral,
        Identifier,
        Number,
    }

    pub fn read_str(input: &str) -> Result<Vec<String>, failure::Error> {
        let mut state = State::None;
        let mut buffer = String::new();
        let mut result: Vec<String> = vec![];

        for c in input.chars() {
            match state {
                State::None => match c {
                    ' ' | '\n' | '\t' => {}
                    c @ 'a'...'z' | c @ 'A'...'Z' => {
                        state = State::Identifier;
                        buffer = String::new();
                        buffer.push(c);
                    }
                    c @ '\'' => {
                        state = State::StringLiteral;
                        buffer = String::new();
                        buffer.push(c);
                    }
                    c @ '0'...'9' => {
                        state = State::Number;
                        buffer = String::new();
                        buffer.push(c);
                    }
                    c @ '(' | c @ ')' | c @ ';' => {
                        result.push(format!("{}", c));
                    }
                    c @ _ => unimplemented!("State is None unknown char: {}", c),
                },
                State::Identifier => match c {
                    c @ 'a'...'z' | c @ 'A'...'Z' => {
                        buffer.push(c);
                    }
                    ' ' | '\n' | '\t' => {
                        state = State::None;
                        result.push(buffer.clone());
                    }
                    c @ ';' => {
                        state = State::None;
                        result.push(buffer.clone());
                        result.push(String::from(format!("{}", c)));
                    }
                    c @ '(' | c @ ')' | c @ ',' => {
                        state = State::None;
                        result.push(buffer.clone());
                        result.push(String::from(format!("{}", c)));
                    }
                    _ => {
                        println!("result: {:?}, buffer: {}, c: {}", result, buffer, c);
                        unimplemented!();
                    }
                },
                State::Number => match c {
                    c @ '0'...'9' => {
                        buffer.push(c);
                    }
                    ' ' | '\n' | '\t' => {
                        state = State::None;
                        result.push(buffer.clone());
                    }
                    c @ '(' | c @ ')' | c @ ',' => {
                        state = State::None;
                        result.push(buffer.clone());
                        result.push(String::from(format!("{}", c)));
                    }
                    _ => {
                        unimplemented!();
                    }
                },
                State::StringLiteral => match c {
                    c @ '\'' => {
                        state = State::None;
                        buffer.push(c);
                        result.push(buffer.clone());
                    }
                    c @ _ => {
                        buffer.push(c);
                    }
                },
            }
        }
        Ok(result)
    }

    #[cfg(test)]
    mod tests {
        use super::read_str;

        #[test]
        fn lexing_simple_stuff_works() {
            assert_eq!(
                vec!["SELECT", "1", "FROM", "dual", ";"],
                read_str("SELECT 1 FROM dual;").unwrap()
            );
            assert_eq!(
                vec!["SELECT", "1", "FROM", "dual", ";"],
                read_str(
                    r#"SELECT
1
FROM dual;"#,
                ).unwrap()
            );

            assert_eq!(
                vec!["SELECT", "1", "x", "FROM", "dual", ";"],
                read_str("SELECT 1 x FROM dual;").unwrap()
            );
            assert_eq!(
                vec!["SELECT", "1", "AS", "x", "FROM", "dual", ";"],
                read_str("SELECT 1 AS x FROM dual;").unwrap()
            );
            assert_eq!(
                vec!["SELECT", "'Hi'", "FROM", "dual", ";"],
                read_str("SELECT 'Hi' FROM dual;").unwrap()
            );
            assert_eq!(
                vec![
                    "INSERT", "INTO", "foo", "(", "baz", ",", "bar", ")", "VALUES", "(", "1", ",",
                    "'Hi'", ")", ";",
                ],
                read_str("INSERT INTO foo (baz, bar) VALUES (1, 'Hi');").unwrap()
            );
        }
    }
}
