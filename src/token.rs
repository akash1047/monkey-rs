#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    ILLEGAL,
    EOF,

    IDENT,
    INTEGER,

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    INC,         // ++
    PLUS_ASSIGN, // +=

    LT,
    GT,
    EQ,
    NEQ,
    LEQ,
    GEQ,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FN,
    LET,
    RETURN,
    IF,
    ELSE,
    TRUE,
    FALSE,
}
