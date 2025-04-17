use crate::custom_types::types::{Scanner, TokenType, Token};


 impl<T>  Scanner<Token<TokenType, T>> {
    pub fn new(source: &str, current: u16, token: Token<TokenType, T> ) -> Self{
       Self{
        source: source.to_string(),
        current,
        tokens: vec![token]
        }
       } 

    }
