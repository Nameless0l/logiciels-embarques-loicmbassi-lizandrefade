# Préparation TP2 : Compilateur et Interpréteur Logo

## Partie 1 : Programme Logo dessinant un carré

Un carré de 100 unités de côté en Logo (grammaire complète) :

```logo
pendown
forward 100
right 90
forward 100
right 90
forward 100
right 90
forward 100
```

Avec la boucle `repeat` (grammaire étendue) :

```logo
pendown
repeat 4 [ forward 100 right 90 ]
```

---

## Partie 2 : Analyse lexicale et syntaxique

### Cargo.toml

```toml
[package]
name = "logo"
version = "0.1.0"
edition = "2021"

[dependencies]
santiago = "1.3.1"
svg_fmt = "0.4"
```

### Lexer (grammaire simplifiée)

```rust
use santiago::lexer::LexerRules;

fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "FORWARD"  = string "forward";
        "DEFAULT" | "BACKWARD" = string "backward";
        "DEFAULT" | "LEFT"     = string "left";
        "DEFAULT" | "RIGHT"    = string "right";
        "DEFAULT" | "NUMBER"   = pattern r"[0-9]+";
        "DEFAULT" | "WS"      = pattern r"\s+" => |lexer| lexer.skip();
    )
}
```

**Attention** : les règles `string` doivent être déclarées **avant** la règle `pattern` pour `NUMBER`, car Santiago prend le match le plus long en priorité, et en cas d'égalité, le premier déclaré. Les mots-clés (`forward`, `backward`, etc.) étant des textes fixes, ils ne risquent pas de conflit avec `[0-9]+`, mais l'ordre reste une bonne pratique.

### Test du lexer

```rust
fn main() {
    let input = "forward 100 left 90 forward 50";

    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
    println!("{:#?}", lexemes);
}
```

Sortie attendue :

```
[
    Lexeme { kind: "FORWARD",  raw: "forward", position: (1, 1) },
    Lexeme { kind: "NUMBER",   raw: "100",     position: (1, 9) },
    Lexeme { kind: "LEFT",     raw: "left",    position: (1, 13) },
    Lexeme { kind: "NUMBER",   raw: "90",      position: (1, 18) },
    Lexeme { kind: "FORWARD",  raw: "forward", position: (1, 21) },
    Lexeme { kind: "NUMBER",   raw: "50",      position: (1, 29) },
]
```

### Parser (grammaire simplifiée, sans AST)

```rust
use santiago::grammar::Grammar;

fn grammar() -> Grammar<()> {
    santiago::grammar!(
        "program" => rules "command" "program";
        "program" => empty;

        "command" => rules "order" "number";

        "order" => lexemes "FORWARD";
        "order" => lexemes "BACKWARD";
        "order" => lexemes "LEFT";
        "order" => lexemes "RIGHT";

        "number" => lexemes "NUMBER";
    )
}
```

### Test du parser

```rust
fn main() {
    let input = "forward 100";

    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();

    let grammar = grammar();
    let parse_trees = &santiago::parser::parse(&grammar, &lexemes)
        .expect("syntax error")[0];
    println!("{}", parse_trees);
}
```

---

## Partie 2c : AST et grammaire typée

### Définition de l'enum AST

```rust
#[derive(Debug, Clone)]
enum AST {
    Program(Vec<AST>),   // contient [Command, Program] ou []
    Command(Vec<AST>),   // contient [Order, Number]
    Forward,
    Backward,
    Left,
    Right,
    Number(i32),
    None,                // chaîne vide (programme vide)
}
```

### Grammaire avec construction de l'AST

```rust
fn grammar() -> Grammar<AST> {
    santiago::grammar!(
        "program" => rules "command" "program" => AST::Program;
        "program" => empty => AST::None;

        "command" => rules "order" "number" => AST::Command;

        "order" => lexemes "FORWARD"  => |_| AST::Forward;
        "order" => lexemes "BACKWARD" => |_| AST::Backward;
        "order" => lexemes "LEFT"     => |_| AST::Left;
        "order" => lexemes "RIGHT"    => |_| AST::Right;

        "number" => lexemes "NUMBER" => |lexemes| {
            let value = lexemes[0].raw.parse::<i32>().unwrap();
            AST::Number(value)
        };
    )
}
```

### Affichage de l'AST

```rust
fn main() {
    let input = "forward 100 left 90";

    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();

    let grammar = grammar();
    let parse_tree = &santiago::parser::parse(&grammar, &lexemes)
        .expect("syntax error")[0];

    let ast = parse_tree.as_abstract_syntax_tree();
    println!("{:?}", ast);
}
```

Sortie attendue :

```
Program([Command([Forward, Number(100)]), Program([Command([Left, Number(90)]), None])])
```

### Fonction eval

```rust
fn eval(ast: &AST) {
    match ast {
        AST::Program(children) => {
            if children.is_empty() {
                println!("Stop");
                return;
            }
            // children[0] = command, children[1] = program (suite)
            eval(&children[0]);
            eval(&children[1]);
        }
        AST::Command(children) => {
            // children[0] = order, children[1] = number
            let order = &children[0];
            let number = match &children[1] {
                AST::Number(n) => *n,
                _ => unreachable!(),
            };
            match order {
                AST::Forward  => println!("Avance de {} unités", number),
                AST::Backward => println!("Recule de {} unités", number),
                AST::Left     => println!("Tourne à gauche de {} degrés", number),
                AST::Right    => println!("Tourne à droite de {} degrés", number),
                _ => unreachable!(),
            }
        }
        AST::None => {
            println!("Stop");
        }
        _ => {}
    }
}
```

---

## Partie 3 : Compilateur Logo vers SVG

### Test de svg_fmt (bin séparé)

On peut créer un fichier `src/bin/test_svg.rs` :

```rust
use svg_fmt::*;

fn main() {
    println!("{}", BeginSvg { w: 300.0, h: 300.0 });

    // Carré via des lignes SVG brutes (svg_fmt n'a pas de "line")
    // On peut utiliser la fonction path() ou écrire du SVG brut
    println!(r#"  <path d="M 100 100 L 200 100" stroke="red" fill="none"/>"#);
    println!(r#"  <path d="M 200 100 L 200 200" stroke="red" fill="none"/>"#);
    println!(r#"  <path d="M 200 200 L 100 200" stroke="red" fill="none"/>"#);
    println!(r#"  <path d="M 100 200 L 100 100" stroke="red" fill="none"/>"#);

    println!("{}", EndSvg);
}
```

Dans `Cargo.toml`, déclarer le bin :

```toml
[[bin]]
name = "test_svg"
path = "src/bin/test_svg.rs"
```

On peut aussi générer du SVG directement sans svg_fmt, avec des `format!()`.

### Structure Logo et compilation

```rust
use std::f64::consts::PI;

struct Logo {
    x: f64,
    y: f64,
    angle: f64,       // en degrés, 0 = vers le haut (nord)
    pen_down: bool,
    svg_content: String,
}

impl Logo {
    fn new() -> Self {
        Logo {
            x: 150.0,         // position initiale au centre
            y: 150.0,
            angle: 0.0,       // orienté vers le haut
            pen_down: true,
            svg_content: String::new(),
        }
    }

    fn compile(&mut self, ast: &AST) -> String {
        self.walk(ast);

        format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="300" height="300">
{}
</svg>"#,
            self.svg_content
        )
    }

    fn walk(&mut self, ast: &AST) {
        match ast {
            AST::Program(children) => {
                if children.is_empty() {
                    return;
                }
                self.walk(&children[0]); // command
                self.walk(&children[1]); // program (suite)
            }
            AST::Command(children) => {
                let order = &children[0];
                let number = match &children[1] {
                    AST::Number(n) => *n as f64,
                    _ => unreachable!(),
                };
                match order {
                    AST::Forward  => self.move_forward(number),
                    AST::Backward => self.move_forward(-number),
                    AST::Left     => self.angle -= number,
                    AST::Right    => self.angle += number,
                    _ => unreachable!(),
                }
            }
            AST::None => {}
            _ => {}
        }
    }

    fn move_forward(&mut self, distance: f64) {
        // Conversion : angle 0 = nord (vers le haut)
        // En SVG, l'axe Y est inversé (vers le bas)
        let angle_rad = self.angle * PI / 180.0;
        let new_x = self.x + distance * angle_rad.sin();
        let new_y = self.y - distance * angle_rad.cos();

        if self.pen_down {
            self.svg_content.push_str(&format!(
                "  <path d=\"M {:.1} {:.1} L {:.1} {:.1}\" stroke=\"black\" fill=\"none\"/>\n",
                self.x, self.y, new_x, new_y
            ));
        }

        self.x = new_x;
        self.y = new_y;
    }
}
```

### Utilisation dans main

```rust
use std::fs;

fn main() {
    let input = "forward 100 right 90 forward 100 right 90 forward 100 right 90 forward 100";

    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();

    let grammar = grammar();
    let parse_tree = &santiago::parser::parse(&grammar, &lexemes)
        .expect("syntax error")[0];

    let ast = parse_tree.as_abstract_syntax_tree();

    let mut logo = Logo::new();
    let svg = logo.compile(&ast);

    fs::write("output.svg", &svg).expect("Impossible d'écrire le fichier SVG");
    println!("Fichier output.svg généré !");
    println!("{}", svg);
}
```

---

## Partie 4 (Bonus) : Grammaire étendue

### Lexer étendu

```rust
fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "FORWARD"  = string "forward";
        "DEFAULT" | "BACKWARD" = string "backward";
        "DEFAULT" | "LEFT"     = string "left";
        "DEFAULT" | "RIGHT"    = string "right";
        "DEFAULT" | "PENUP"    = string "penup";
        "DEFAULT" | "PENDOWN"  = string "pendown";
        "DEFAULT" | "REPEAT"   = string "repeat";
        "DEFAULT" | "LBRACKET" = string "[";
        "DEFAULT" | "RBRACKET" = string "]";
        "DEFAULT" | "NUMBER"   = pattern r"[0-9]+";
        "DEFAULT" | "WS"      = pattern r"\s+" => |lexer| lexer.skip();
    )
}
```

### AST étendu

```rust
#[derive(Debug, Clone)]
enum AST {
    Program(Vec<AST>),
    Command(Vec<AST>),
    Block(Vec<AST>),        // contient le program intérieur
    Repeat(Vec<AST>),       // contient [Number, Command/Block]
    Forward,
    Backward,
    Left,
    Right,
    PenUp,
    PenDown,
    Number(i32),
    None,
}
```

### Grammaire étendue avec AST

```rust
fn grammar() -> Grammar<AST> {
    santiago::grammar!(
        "program" => rules "command" "program" => AST::Program;
        "program" => empty => AST::None;

        "command" => rules "action" "number" => AST::Command;
        "command" => rules "state";
        "command" => rules "loop";
        "command" => rules "block";

        "block" => rules "lbracket" "program" "rbracket" => AST::Block;

        "loop" => rules "repeat" "number" "command" => AST::Repeat;

        "action" => lexemes "FORWARD"  => |_| AST::Forward;
        "action" => lexemes "BACKWARD" => |_| AST::Backward;
        "action" => lexemes "LEFT"     => |_| AST::Left;
        "action" => lexemes "RIGHT"    => |_| AST::Right;

        "state" => lexemes "PENUP"   => |_| AST::PenUp;
        "state" => lexemes "PENDOWN" => |_| AST::PenDown;

        "number" => lexemes "NUMBER" => |lexemes| {
            AST::Number(lexemes[0].raw.parse::<i32>().unwrap())
        };

        "repeat"   => lexemes "REPEAT";
        "lbracket" => lexemes "LBRACKET";
        "rbracket" => lexemes "RBRACKET";
    )
}
```

### Compilation étendue (ajouts dans walk)

```rust
fn walk(&mut self, ast: &AST) {
    match ast {
        AST::Program(children) => {
            if children.is_empty() { return; }
            self.walk(&children[0]);
            self.walk(&children[1]);
        }
        AST::Command(children) => {
            let order = &children[0];
            let number = match &children[1] {
                AST::Number(n) => *n as f64,
                _ => unreachable!(),
            };
            match order {
                AST::Forward  => self.move_forward(number),
                AST::Backward => self.move_forward(-number),
                AST::Left     => self.angle -= number,
                AST::Right    => self.angle += number,
                _ => unreachable!(),
            }
        }
        AST::Block(children) => {
            // children[1] est le program intérieur (entre les crochets)
            self.walk(&children[1]);
        }
        AST::Repeat(children) => {
            // children[1] = number, children[2] = command/block
            let count = match &children[1] {
                AST::Number(n) => *n,
                _ => unreachable!(),
            };
            for _ in 0..count {
                self.walk(&children[2]);
            }
        }
        AST::PenUp   => self.pen_down = false,
        AST::PenDown => self.pen_down = true,
        AST::None => {}
        _ => {}
    }
}
```

Exemple de programme étendu : `repeat 4 [ forward 100 right 90 ]` dessine un carré.

---

## Récapitulatif : fichier main.rs complet (grammaire simplifiée)

```rust
use santiago::grammar::Grammar;
use santiago::lexer::LexerRules;
use std::f64::consts::PI;
use std::fs;

// ─── AST ──────────────────────────────

#[derive(Debug, Clone)]
enum AST {
    Program(Vec<AST>),
    Command(Vec<AST>),
    Forward,
    Backward,
    Left,
    Right,
    Number(i32),
    None,
}

// ─── LEXER ────────────────────────────

fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "FORWARD"  = string "forward";
        "DEFAULT" | "BACKWARD" = string "backward";
        "DEFAULT" | "LEFT"     = string "left";
        "DEFAULT" | "RIGHT"    = string "right";
        "DEFAULT" | "NUMBER"   = pattern r"[0-9]+";
        "DEFAULT" | "WS"      = pattern r"\s+" => |lexer| lexer.skip();
    )
}

// ─── GRAMMAR ──────────────────────────

fn grammar() -> Grammar<AST> {
    santiago::grammar!(
        "program" => rules "command" "program" => AST::Program;
        "program" => empty => AST::None;
        "command" => rules "order" "number" => AST::Command;
        "order" => lexemes "FORWARD"  => |_| AST::Forward;
        "order" => lexemes "BACKWARD" => |_| AST::Backward;
        "order" => lexemes "LEFT"     => |_| AST::Left;
        "order" => lexemes "RIGHT"    => |_| AST::Right;
        "number" => lexemes "NUMBER" => |lexemes| {
            AST::Number(lexemes[0].raw.parse::<i32>().unwrap())
        };
    )
}

// ─── EVAL ─────────────────────────────

fn eval(ast: &AST) {
    match ast {
        AST::Program(children) => {
            if children.is_empty() {
                println!("Stop");
                return;
            }
            eval(&children[0]);
            eval(&children[1]);
        }
        AST::Command(children) => {
            let number = match &children[1] {
                AST::Number(n) => *n,
                _ => unreachable!(),
            };
            match &children[0] {
                AST::Forward  => println!("Avance de {} unités", number),
                AST::Backward => println!("Recule de {} unités", number),
                AST::Left     => println!("Tourne à gauche de {} degrés", number),
                AST::Right    => println!("Tourne à droite de {} degrés", number),
                _ => unreachable!(),
            }
        }
        AST::None => println!("Stop"),
        _ => {}
    }
}

// ─── COMPILATEUR SVG ──────────────────

struct Logo {
    x: f64,
    y: f64,
    angle: f64,
    pen_down: bool,
    svg_content: String,
}

impl Logo {
    fn new() -> Self {
        Logo {
            x: 150.0,
            y: 150.0,
            angle: 0.0,
            pen_down: true,
            svg_content: String::new(),
        }
    }

    fn compile(&mut self, ast: &AST) -> String {
        self.walk(ast);
        format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
             <svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" \
             width=\"300\" height=\"300\">\n\
             {}</svg>\n",
            self.svg_content
        )
    }

    fn walk(&mut self, ast: &AST) {
        match ast {
            AST::Program(children) => {
                if children.is_empty() { return; }
                self.walk(&children[0]);
                self.walk(&children[1]);
            }
            AST::Command(children) => {
                let n = match &children[1] {
                    AST::Number(v) => *v as f64,
                    _ => unreachable!(),
                };
                match &children[0] {
                    AST::Forward  => self.move_forward(n),
                    AST::Backward => self.move_forward(-n),
                    AST::Left     => self.angle -= n,
                    AST::Right    => self.angle += n,
                    _ => unreachable!(),
                }
            }
            AST::None => {}
            _ => {}
        }
    }

    fn move_forward(&mut self, distance: f64) {
        let angle_rad = self.angle * PI / 180.0;
        let new_x = self.x + distance * angle_rad.sin();
        let new_y = self.y - distance * angle_rad.cos();

        if self.pen_down {
            self.svg_content.push_str(&format!(
                "  <path d=\"M {:.1} {:.1} L {:.1} {:.1}\" \
                 stroke=\"black\" fill=\"none\"/>\n",
                self.x, self.y, new_x, new_y
            ));
        }

        self.x = new_x;
        self.y = new_y;
    }
}

// ─── MAIN ─────────────────────────────

fn main() {
    // Carré de 100 unités
    let input = "forward 100 right 90 forward 100 right 90 forward 100 right 90 forward 100";

    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
    println!("=== Lexemes ===");
    for lex in &lexemes {
        println!("  {:?}", lex);
    }

    let grammar = grammar();
    let parse_tree = &santiago::parser::parse(&grammar, &lexemes)
        .expect("syntax error")[0];

    let ast = parse_tree.as_abstract_syntax_tree();
    println!("\n=== AST ===");
    println!("{:?}", ast);

    println!("\n=== Eval ===");
    eval(&ast);

    println!("\n=== Compilation SVG ===");
    let mut logo = Logo::new();
    let svg = logo.compile(&ast);
    fs::write("output.svg", &svg).expect("Erreur écriture SVG");
    println!("Fichier output.svg généré avec succès !");
}
```

---

## Points clés à retenir pour le TP

**Santiago** utilise un parseur Earley, capable de gérer toute grammaire context-free (y compris ambiguë).

**Trigonométrie de la tortue** : l'angle 0 pointe vers le haut (nord). En SVG, l'axe Y est inversé. Donc : `new_x = x + dist * sin(angle)` et `new_y = y - dist * cos(angle)`.

**L'AST est récursif** : `Program` contient `[Command, Program]`, ce qui forme une liste chaînée terminée par `None`.

**Pour la grammaire étendue** (Partie 4), les variantes `AST::Block` et `AST::Repeat` nécessitent d'accéder à des `children` dont l'index dépend de la construction Santiago (les terminaux comme `[`, `]`, `repeat` sont aussi dans le Vec).