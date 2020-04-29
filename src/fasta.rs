use bnf::Grammar;

fn main() {
   let input = "
<fastq>   :=  <block>+
<block>   :=  @<seqname>\n<seq>\n+[<seqname>]\n<qual>\n
<seqname> :=  [A-Za-z0-9]+
<seq>     :=  [A-Za-z]+
<qual>    :=  [!-~\n]+
";
    let grammar: Grammar = input.parse().unwrap();
    let sentence = grammar.generate();

    match sentence {
        Ok(s) => println!("random sentence: {}", s),
        Err(e) => println!("something went wrong: {}!", e)
    }
}
