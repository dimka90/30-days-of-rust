#[derive(Debug)]
pub struct Scanner<Token>{
    pub  source: String,
    pub current: u16,
    pub tokens: Vec<Token>
}


#[derive(Debug)]
pub struct Token<TokenType, T> {
    pub  token_type: TokenType,
    pub lexeme: String,
    pub literal: T,
    pub line_number: u64,

}


#[derive(Debug)]
pub  enum  TokenType{
    // Single token
    COMMA,
    IDENT,
    EQUAL,

}


