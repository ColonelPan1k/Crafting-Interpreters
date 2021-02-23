class Lexer:
    keywords = { "and" : AND,
                 "class" : CLASS,
                 "else" : ELSE,
                 "false" : FALSE,
                 "for" : FOR,
                 "fun" : FUN,
                 "if" : IF,
                 "nil" : NIL,
                 "or" : OR,
                 "print" : PRINT,
                 "return" : RETURN,
                 "super" : SUPER,
                 "this" : THIS,
                 "true" : TRUE,
                 "var" : VAR,
                 "while" : WHILE}
    start = 0
    current = 0
    line = 1
    tokens = []
    source = ""

    def __init__(self, source):
        self.source = source

    def scanTokens(self):
        while (!isAtEnd()):
            start = current
            scanToken();
        tokens.append(create_token(EOF, "", None, line))
        return tokens

    def scanToken(self):
        c = advance()
