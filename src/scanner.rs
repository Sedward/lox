

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum TokenType {
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
    //Slash, 
    Star,
    //// One or two character tokens.
    Bang, 
    BangEqual,
    Equal,
    EqualEqual,
    //Greater, 
    //GreaterEqual,
    //Less, 
    //LessEqual,
    //// Literals.
    //Identifier,
    //String, 
    //Number,

    //// Keywords.
    //And,
    //Class,
    //Else,
    //False, Fun, For, If, Nil, Or,
    //Print, Return, Super, This, True, Var, While,
    EOF
}

//pub struct Token {
    ////token_type: TokenType,
    //lexeme: String, 
//}

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    //line: usize
}

impl<'a> Scanner<'a> {

    pub fn new(source: &'a str) -> Self {
        Scanner{source: source, 
            start: 0, 
            current: 0, 
            //line: 1
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<TokenType>{

        let mut tokens: Vec<TokenType> = Vec::new();
        while !self.is_end() {

            self.start = self.current;
            let token = self.scan_token();
            tokens.push(token.unwrap());
            //TODO unwarp and skip none? 
        }
        tokens
    }

    fn scan_token(&mut self) -> Option<TokenType> {
        let c: char = self.advance();
        let token_type: Option<TokenType> = match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen), 
            '{' => Some(TokenType::LeftBrace), 
            '}' => Some(TokenType::RightBrace), 
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot), 
            '-' => Some(TokenType::Minus), 
            '+' => Some(TokenType::Plus), 
            ';' => Some(TokenType::Semicolon), 
            '*' => Some(TokenType::Star),
            '!' => if self.cond_advance('='){
                        Some(TokenType::BangEqual);
                    else {
                        Some(TokenType::Bang)
                    }
                },
            ' ' => None,
            _ => Some(TokenType::EOF),
        };
        token_type
    }


    fn is_end(&self) -> bool{
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c: char = self.source[self.current..].chars().next().unwrap();
        self.current += 1;
        c
    }
    //TODO refactor this to use Result and a less complicated match above
    fn cond_advance(&mut self, expected: char) -> bool {
        let current: char = self.source[self.current..].chars().next().unwrap();
        if self.is_end() || current != expected {
            false
        } else {

            self.current +=1;
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scanner_can_advance() {
        let mut scanner = Scanner::new("abc");
        assert_eq!(scanner.advance(), 'a');
        assert_eq!(scanner.advance(), 'b');
        assert_eq!(scanner.advance(), 'c');
    }

    #[test]
    fn scan_single_token(){
        let mut scanner = Scanner::new("(())");
        let t = scanner.scan_token();
        assert_eq!(t.unwrap(), TokenType::LeftParen);
    }
    #[test]
    fn scanner_can_tokenize_single_lexemes(){
        let mut scanner = Scanner::new("(())");
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 4)
    }

    #[test]
    fn scanner_can_conditionally_advance(){
        let mut scanner = Scanner::new("==");
        let tokens: Vec<TokenType> = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1);
    }
}