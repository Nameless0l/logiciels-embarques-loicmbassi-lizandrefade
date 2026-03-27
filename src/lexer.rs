use santiago::lexer::LexerRules;

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "FORWARD"  = string "forward";
        "DEFAULT" | "BACKWARD" = string "backward";
        "DEFAULT" | "LEFT"     = string "left";
        "DEFAULT" | "RIGHT"    = string "right";
        "DEFAULT" | "NUMBER"   = pattern r"[0-9]+";
        "DEFAULT" | "WS"      = pattern r"\s+" => |lexer| lexer.skip();
    )
}
