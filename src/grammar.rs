use crate::types::AST;
use santiago::grammar::Grammar;
//Grammar<T> contient : les règles, les transitions, les tables internes, les actions associées (les closures => |rules| ...), la logique de parsing
//Grammar<T> fait le taff de parser
//

pub fn grammar() -> Grammar<AST> {
    //La librairie santiago est extrèmement flexible
    //Le besoin de créer des commandes est très répondu et les besoins sont très très souvent les mêmes
    //(ex: besoin d'écrire  program ::= commandA commandC program)
    santiago::grammar!(
        //Pour chq command lue par le parser, il applique la règle command program (program impératif pour la récursion)
        //"program" => rules "command" "commandbis" "program"; serait possible et lirait 2 arguments par 2 arguments
        "program" => rules "command" "program"
        // cette ligne est ésuivalent à : rules = vec![AST_Command(...), AST_Program(...)]
        // avec itération on finit avec : vec![AST::Command(...), AST::Command(...), AST::Command(...), AST::Program(...)]
            => |rules| AST::Program(rules);
        "program" => empty
            => |_| AST::None;
        //la chaîne de fin de chaîne de caractère va essayer d'être interprété en tant que commande et va 'fail'

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
        //lexemes sert à définir quelles séquences de tokens sont acceptées pour un non‑terminal donné
        //Les symboles terminaux sont utilisés "FORWARD","BACKWARD","100" pour définir commande (symbole non)
        "command" => rules "repeat";

        "repeat" => rules "repeat_kw" "number" "block"
            => |rules| AST::Repeat(rules);
        "repeat_kw" => lexemes "REPEAT"
            => |_| AST::None;

        "block" => rules "lbracket" "program" "rbracket"
            => |rules| AST::Block(vec![rules[1].clone()]); //Pour encapsulation
        "lbracket" => lexemes "LBRACKET"
            => |_| AST::None;
        "rbracket" => lexemes "RBRACKET"
            => |_| AST::None;
        "command" => lexemes "PENUP"
            => |_| AST::PenUp;
        "command" => lexemes "PENDOWN"
            => |_| AST::PenDown;

    )
}

pub fn eval(ast: &AST) {
    //test pour savoir comment les commandes réagissent (sert à début)
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
