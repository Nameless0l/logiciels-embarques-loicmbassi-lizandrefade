use logo::grammar::grammar;
use logo::lexer::lexer_rules;
use logo::types::Logo;
fn main() {
    let input =
        "forward 100 right 90 forward 100 right 90 forward 100 right 90 forward 100 right 90";
    let lexemes = santiago::lexer::lex(&lexer_rules(), &input).unwrap();
    let grammar = grammar();
    let parse_trees = &santiago::parser::parse(&grammar, &lexemes).expect("syntax error")[0];
    let ast = parse_trees.as_abstract_syntax_tree();
    let mut logo = Logo::new();
    let svg = logo.compile(&ast);
    std::fs::write("output.svg", &svg).unwrap();
    println!("{}", svg);
    // logo::grammar::eval(&ast);
}
