use bnf::Grammar;

fn main() {
    let input = "<postal-address> ::= <name-part> <street-address> <zip-part>

            <name-part> ::= <personal-part> <last-name> <opt-suffix-part> <EOL>
                            | <personal-part> <name-part>

        <personal-part> ::= <initial> \".\" | <first-name>

        <street-address> ::= <house-num> <street-name> <opt-apt-num> <EOL>

            <zip-part> ::= <town-name> \",\" <state-code> <ZIP-code> <EOL>

        <opt-suffix-part> ::= \"Sr.\" | \"Jr.\" | <roman-numeral> | \"\"
            <opt-apt-num> ::= <apt-num> | \"\"";

    let grammar: Result<Grammar, _> = input.parse();
    match grammar {
        Ok(g) => println!("{:#?}", g),
        Err(e) => println!("Failed to make grammar from String: {}", e),
    }
}
