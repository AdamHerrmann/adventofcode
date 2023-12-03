#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}
pub struct Parser<I>
where
    I: Iterator<Item = char>,
{
    iter: I,
    line: u32,
    column: u32,
    next: Option<char>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(iter: I) -> Parser<I> {
        Parser {
            iter,
            line: 0,
            column: 0,
            next: None,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        if let None = self.next {
            self.next = self.iter.next();
        }

        self.next
    }

    pub fn loc(&self) -> Location {
        Location {
            line: self.line,
            column: self.column,
        }
    }

    pub fn consume(&mut self) {
        if let Some('\n') = self.next {
            self.line = self.line + 1;
            self.column = 0;
        } else {
            self.column = self.column + 1;
        }

        self.next = None;
    }
}
