

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    chr: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 1,
            chr: '0',
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.chr = '0';
        } else {
            self.chr = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position;
    }
}
