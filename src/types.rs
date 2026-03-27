#[derive(Debug, Clone)]
pub enum AST {
    //L'énoncé nous demande de rassembler ainsi
    //Pour être plus clair on aurait pu imaginé une séparation en enum Args et enum Commands
    Program(Vec<AST>),
    Command(Vec<AST>),
    Forward,
    Backward,
    Left,
    Right,
    Number(i32),
    None,
}

pub struct Logo {
    x: f64,
    y: f64,
    angle: f64,
    pen_down: bool, //pour activer le mode écriture - On va réutiliser move_forward au lieu de recréer des fts
    svg_content: String, //porte l'instruction html pour modifier ?(à voir)
}

impl Logo {
    pub fn new() -> Self {
        Self {
            x: 150.0,
            y: 150.0,
            angle: 0.0,
            pen_down: true,
            svg_content: String::new(),
        }
    }

    pub fn compile(&mut self, ast: &AST) -> String {
        self.walk(ast);
        format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"500\" height=\"500\">\n{}</svg>\n",
            self.svg_content
        ) //Nous envoie à un site où le carré est déjà dessiné - le but est de vérifier que les instructions fonctionne pour l'instant
    }

    pub fn walk(&mut self, ast: &AST) {
        match ast {
            AST::Program(children) => children.iter().for_each(|c| self.walk(c)),
            AST::Command(children) => {
                if let AST::Number(n) = &children[1] {
                    match &children[0] {
                        AST::Forward => self.move_forward(*n as f64), //<---- Ici on rétulise move_forwardAttention au pen_down
                        AST::Backward => self.move_forward(-(*n as f64)),
                        AST::Left => self.angle -= *n as f64,
                        AST::Right => self.angle += *n as f64,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    pub fn move_forward(&mut self, dist: f64) {
        let rad = self.angle.to_radians();
        let (nx, ny) = (self.x + dist * rad.sin(), self.y - dist * rad.cos());
        if self.pen_down {
            self.svg_content.push_str(&format!(
                "  <path d=\"M {:.1} {:.1} L {:.1} {:.1}\" stroke=\"red\" fill=\"none\"/>\n",
                self.x, self.y, nx, ny
            )); // {:.1} permet de garder un chiffre après la virgule
        }
        self.x = nx;
        self.y = ny;
    }
}
