use bnf::Grammar;

fn main() {
    let input = "<file>     ::= <token> | <token> <file>
                <token>    ::= <ignore> | <seq>
                <ignore>   ::= <whitespace> | <comment> <newline>
                <seq>      ::= <header> <molecule> <newline>
                <header>   ::= \">\" <arbitrary text> <newline>
                <molecule> ::= <mol-line> | <mol-line> <molecule>
                <mol-line> ::= <nucl-line> | <prot-line>
                <nucl-line>::= \"^[ACGTURYKMSWBDHVNX-]+$\"
                <prot-line>::= \"^[ABCDEFGHIKLMNOPQRSTUVWYZX*-]+$\"";
    let grammar: Grammar = input.parse().unwrap();
    let sentence = grammar.generate();
    match sentence {
        Ok(s) => println!("random sentence: {}", s),
        Err(e) => println!("something went wrong: {}!", e),
    }
}
