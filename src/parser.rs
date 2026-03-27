use santiago::grammar::Grammar;

pub fn grammar() -> Grammar<()> {
    //La librairie santiago est extrèmement flexible
    //Le besoin de créer des commandes est très répondu et les besoins sont très très souvent les mêmes
    //(ex: besoin d'écrire  program ::= commandA commandC program)
    santiago::grammar!(
        "program" => rules "command" "program"; //Pour chq command lue par le parser, il applique la règle command program (program impératif pour la récursion)
        //"program" => rules "command" "commandbis" "program"; serait possible et lirait 2 arguments par 2 arguments
        "program" => empty;                     //la chaîne de fin de chaîne de caractère va essayer d'être interprété en tant que commande et vaFail

        "command" => rules "order" "number";

        "order" => lexemes "FORWARD";
        "order" => lexemes "BACKWARD";
        "order" => lexemes "LEFT";
        "order" => lexemes "RIGHT";
        //lexemes sert à définir quelles séquences de tokens sont acceptées pour un non‑terminal donné
        //Les symboles terminaux sont utilisés "FORWARD","BACKWARD","100" pour définir commande (symbole non)
        "number" => lexemes "NUMBER";
    )
}
