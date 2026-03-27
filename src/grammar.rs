use crate::types::AST;
use santiago::grammar::Grammar;

pub fn grammar() -> Grammar<AST> {
    santiago::grammar!(
        "program" => rules "command" "program"
            => |rules| AST::Program(rules);
        "program" => empty
            => |_| AST::None;

        "command" => rules "order" "number"
            => |rules| AST::Command(rules);

        "order" => lexemes "FORWARD"
            => |_| AST::Forward;
        "order" => lexemes "BACKWARD"
            => |_| AST::Backward;
        "order" => lexemes "LEFT"
            => |_| AST::Left;
        "order" => lexemes "RIGHT"
            => |_| AST::Right;

        "number" => lexemes "NUMBER"
            => |lexemes| AST::Number(lexemes[0].raw.parse().unwrap());
    )
}

pub fn eval(ast: &AST) {
    match ast {
        AST::Program(children) => children.iter().for_each(|c| eval(c)),
        AST::Command(children) => {
            if let AST::Number(n) = &children[1] {
                match &children[0] {
                    AST::Forward => println!("Avance de {} unités", n),
                    AST::Backward => println!("Recule de {} unités", n),
                    AST::Left => println!("Tourne à gauche de {} degrés", n),
                    AST::Right => println!("Tourne à droite de {} degrés", n),
                    _ => {}
                }
            }
        }
        AST::None => println!("Stop"),
        _ => {}
    }
}
