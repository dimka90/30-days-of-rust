use custom_types::types::{Scanner, TokenType, Token};

mod compiler;
mod custom_types;
fn main() {

// let token: Token<TokenType> = Token { token_type: TokenType::COMMA, lexeme: ",".to_string(), literal: ",".to_string(), line_number: 1 };
// //    let scanner :Scanner<Token<TokenType>> =  custom_types::types::Scanner::new("num:u16 = 1", 10, } 
//    let scanner2 :Scanner<Token<TokenType>> =  custom_types::types::Scanner::new("Dimka,",1, token);

// //    println!("{:?}", scanner);
//    println!("{:?}", scanner2);


let token_type = TokenType::COMMA;
let token:Token<TokenType, Option<String>>= Token {
   token_type: token_type,
   lexeme: String::from(","),
   literal: None,
   line_number: 1,
};

println!("{:?}", token);
}