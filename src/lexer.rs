
use std::{iter::Peekable, str::Chars};



#[derive(Debug, PartialEq)]
pub enum Token {
    // Whitespace which will be ignored later
    Whitespace,
    Illegal,
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma, 
    Dot, 
    Minus,
    Plus,
    Semicolon,
    Slash, 
    Star,
    // One or two character tokens.
    Bang, 
    BangEqual,
    Equal,
    EqualEqual,
    Greater, 
    GreaterEqual,
    Less, 
    LessEqual,
    // Literals.
    String(String),
    Number(String),

    // Identifiers.
    And,
    Class,
    Else,
    False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
    // End of file
    //Eof
}

#[allow(dead_code)]
pub struct Lexer<'a>{
    source_iter: Peekable<Chars<'a>>,
    current: u32,
    line: u32
}

impl<'a> Lexer<'a> {

    pub fn new(source: &'a str) -> Self {
        Lexer{source_iter: source.chars().peekable(), line: 1, current: 1}
    }

    fn read_identifier(&mut self, c: char) -> String {
        let mut identifier = String::new();
        identifier.push(c);
        while let Some(&c) = self.source_iter.peek() {
            if c.is_alphabetic() {
                identifier.push(self.source_iter.next().unwrap());
                self.current += 1;
            } else {
                break;
            }
        }
        identifier
    }
    
    fn read_string(&mut self) -> String {
        let mut string = String::new();
        
        while let Some(&c) = self.source_iter.peek() {
            if c != '"'{
                string.push(self.source_iter.next().unwrap());
                self.current += 1;
            } else {
                self.source_iter.next().unwrap();
                self.current += 1
        }
    }
        string
    }
    
    fn read_number(&mut self, c: char) -> String {
        let mut number = String::new();
        number.push(c);
        while let Some(&c) = self.source_iter.peek() {
            if c.is_digit(10) {
                number.push(self.source_iter.next().unwrap());
                self.current += 1;
            } else {
                break;
            }
        }
        number
    }

    fn scan_token(&mut self) -> Option<Token> {

        if let Some(c) = self.source_iter.next(){
        
            self.current += 1;

            match c{
            ' ' | '\r' | '\t' => Some(Token::Whitespace), 
            '\n' => {
                self.line +=1;
                Some(Token::Whitespace)
            }
            '!' => {
                let next_char = self.source_iter.peek().unwrap();
                if next_char == &'='{
                    self.source_iter.next();
                    Some(Token::BangEqual)
                } else {
                    Some(Token::Bang)
                }   
            }
            '=' => {
                let next_char = self.source_iter.peek().unwrap();
                if next_char == &'='{
                    self.source_iter.next();
                    Some(Token::EqualEqual)
                } else {
                    Some(Token::Equal)
                }   
            }
            '<' => {
                let next_char = self.source_iter.peek().unwrap();
                if next_char == &'='{
                    self.source_iter.next();
                    Some(Token::LessEqual)
                } else {
                    Some(Token::Less)
                }   
            }
            '>' => {
                let next_char = self.source_iter.peek().unwrap();
                if next_char == &'='{
                    self.source_iter.next();
                    Some(Token::GreaterEqual)
                } else {
                    Some(Token::Greater)
                }   
            }
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen), 
            '{' => Some(Token::LeftBrace), 
            '}' => Some(Token::RightBrace), 
            '/' => Some(Token::Slash),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot), 
            '-' => Some(Token::Minus), 
            '+' => Some(Token::Plus), 
            ';' => Some(Token::Semicolon), 
            '*' => Some(Token::Star),
            '"' => {
                let string = self.read_string();
                Some(Token::String(string))
            }
            _ => {
                if c.is_alphabetic(){
                    let identifer = self.read_identifier(c);
                    lookup_identifier(&identifer)
                } else if c.is_digit(10) {
                    let number = self.read_number(c);
                    Some(Token::Number(number))
                }
                else {
                   Some(Token::Illegal) 
                }
            }
            }
        } else {
            None
        }
    }
}

fn lookup_identifier(identifer: &str) -> Option<Token>{
    match identifer {
        "and" => Some(Token::And), 
        "class" => Some(Token::Class),
        "else" => Some(Token::Else), 
        "false" => Some(Token::False), 
        "true" => Some(Token::True), 
        "fun" => Some(Token::Fun), 
        "for" => Some(Token::For), 
        "if" => Some(Token::If), 
        "nil" => Some(Token::Nil), 
        "or" => Some(Token::Or), 
        "print" => Some(Token::Print),
        "return" => Some(Token::Return),
        "super" => Some(Token::Super),
        "this" => Some(Token::This),
        "var" => Some(Token::Var),
        "while" => Some(Token::While),
        _ => Some(Token::Illegal)
    }
}


impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token>{
        self.scan_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_single_token(){
        let mut lexer = Lexer::new("()");
        let token1 = lexer.scan_token().unwrap();
        assert_eq!(token1, Token::LeftParen);
        let token2 = lexer.scan_token().unwrap();
        assert_eq!(token2, Token::RightParen);
    }

    #[test]
    fn lexer_implements_iterator(){
        let mut lexer = Lexer::new("()");
        let token1 = lexer.next().unwrap();
        assert_eq!(token1, Token::LeftParen);
    }
    
    #[test]
    fn lexer_scans_two_char_tokens(){
        let mut lexer = Lexer::new("!=");
        let token1 = lexer.next().unwrap();
        assert_eq!(token1, Token::BangEqual);
    }
    
    #[test]
    fn lexer_scans_string(){
        let mut lexer = Lexer::new("\"This is a string\"");
        let token1 = lexer.next().unwrap();
        println!("Did we get this string????? {:?}", token1);
        let expected = Token::String("This is a string".to_string());
        assert_eq!(token1, expected);
    }
}