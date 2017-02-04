use lexer::{ Lexer, Token };

type GrammarFunction = fn(Token);

struct Rule {
    prefix: Option<GrammarFunction>,
    infix: Option<GrammarFunction>,
    postfix: Option<GrammarFunction>,
}

static rules: [Rule; 1] = [
    Rule { prefix: None, infix: None, postfix: None },
];

struct Parser {
    lexer: Lexer,
}

impl Parser {

}