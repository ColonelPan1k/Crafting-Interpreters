/*
This isn't really meant to be a working lexer, I'm just playing around with how parts of a lexer
would be implemented in Rust if I were to write one in Rust.
*/

enum TokenType {

}

struct Lexer {
    source : String;
    tokens : Vec::new();
    start  : u64;
    current : u64;
    line    : u64;
}

fn scanToken(){
    c : char = advance();
    match(c){
        '(' => addToken(LEFT_PAREN);
        ')' => addToken(RIGHT_PAREN);
        '{' => addToken(LEFT_BRACE);
        '}' => addToken(RIGHT_BRACE);
        ',' => addToken(COMMA);
        '.' => addToken(DOT);
        '+' => addToken(PLUS);
        ';' => addToken(SEMICOLON);
        '*' => addToken(STAR);
        // add !, =, <, >, /, and their individual cases
        '"' => string();
        '\n' => line += 1;
    }
}
