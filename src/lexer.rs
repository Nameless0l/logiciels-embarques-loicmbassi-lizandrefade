use santiago::lexer::LexerRules;

//Un parseur (la grammaire) ne travaille jamais sur du texte brut.
//Il travaille sur une suite de tokens. Santiago ne fait pas de lexing automatiquement.
//Il a besoin de savoir comment découper le texte brut en tokens. D'où programme ci-dessous.
pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "FORWARD"  = string "forward"; //pour ignorer la casse on pourrais écrire pattern r"(?i)forward"
        "DEFAULT" | "BACKWARD" = string "backward";
        "DEFAULT" | "LEFT"     = string "left";
        "DEFAULT" | "RIGHT"    = string "right";
        "DEFAULT" | "NUMBER"   = pattern r"[0-9]+"; // regex test qui scanne l'expression et renvoie PAS un bool mais l'objet testé
        "DEFAULT" | "WS"       = pattern r"\s+" => |lexer| lexer.skip();
        "DEFAULT" | "REPEAT"   = string "repeat";
        "DEFAULT" | "LBRACKET" = string "[";
        "DEFAULT" | "RBRACKET" = string "]";
        "DEFAULT" | "PENUP"   = string "penup";
        "DEFAULT" | "PENDOWN"  = string "pendown";
    )
}
