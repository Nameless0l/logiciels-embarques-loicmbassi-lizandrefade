use logo::grammar::grammar;
use logo::lexer::lexer_rules;
use logo::types::Logo;
fn main() {
    //triangle equilatéral
    // let input = "right 30 forward 100 right 120 forward 100  right 120 forward 100 right 120";
    // let fade = "\
    //         forward 60 right 90 forward 40 backward 40 right 90 forward 30 left 90 forward 25 \
    //         penup backward 25 left 90 backward 30 right 90 forward 55 left 90 pendown \
    //         forward 60 right 90 forward 40 right 90 forward 60 backward 30 right 90 forward 40 \
    //         penup backward 40 right 90 backward 30 right 90 forward 55 left 90 pendown \
    //         forward 60 right 90 forward 40 right 90 forward 60 right 90 forward 40 \
    //         penup right 90 right 90 forward 55 left 90 pendown \
    //         forward 60 right 90 forward 40 backward 40 right 90 forward 30 left 90 forward 25 backward 25 right 90 forward 30 left 90 forward 40";
//loic
    let input = "\
            forward 60 right 90 forward 10 right 90 forward 30 left 90 forward 10 left 90 forward 30 right 90 forward 10 right 90 forward 60 \
            penup left 90 forward 15 left 90 pendown \
            forward 60 right 90 forward 30 right 90 forward 30 right 90 forward 30 penup left 90 left 90 forward 30 pendown right 90 forward 30 right 90 forward 30 \
            penup left 90 left 90 forward 45 left 90 pendown \
            forward 60 right 90 forward 30 right 90 forward 60 penup backward 30 right 90 pendown forward 30 \
            penup left 90 forward 30 left 90 forward 45 left 90 pendown \
            right 90 forward 30 left 90 forward 30 left 90 forward 30 right 90 forward 30 right 90 forward 30 \
            penup right 90 forward 60 left 90 forward 15 left 90 pendown \
            right 90 forward 30 left 90 forward 30 left 90 forward 30 right 90 forward 30 right 90 forward 30 \
            penup right 90 forward 60 left 90 forward 15 left 90 pendown \
            right 90 forward 30 penup backward 15 left 90 pendown forward 60 penup right 90 backward 15 pendown forward 30";
    let lexemes = santiago::lexer::lex(&lexer_rules(), &input).unwrap();
    let grammar = grammar();
    let parse_trees = &santiago::parser::parse(&grammar, &lexemes).expect("syntax error")[0];
    let ast = parse_trees.as_abstract_syntax_tree();
    let mut logo = Logo::new();
    let svg = logo.compile(&ast);
    std::fs::write("loic.svg", &svg).unwrap();
    println!("{}", svg);
    // logo::grammar::eval(&ast);
}
