use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    /// The input of the Lexer. This will start by having the entire file loaded into the string,
    /// but will shrink as the characters get lexed.
    pub input: &'a str,
    /// Peekable lookahead to make decisions about multi-character lexemes
    pub lookahead: Peekable<Chars<'a>>,
    /// Holds the position of the next unread character
    // TODO: Convert this to keep track of line/column
    pub position: usize,
}

impl<'a> Lexer<'a> {
    /// Construct a new [`Lexer`], but disallow empty inputs for now.
    pub fn new(input: &'a str) -> Self {
        if input.is_empty() {
            panic!("Can't lex an empty input. For now.");
        }

        Self {
            input,
            lookahead: input.chars().peekable(),
            position: 0,
        }
    }

    /// Get the next character to lex. This should put the [`Lexer`] in a state
    /// where it is only holding the remaining input, and return `None` when done.
    pub fn next_char(&mut self) -> Option<char> {
        let chr = self.lookahead.next()?;
        let len = chr.len_utf8();

        self.input = &self.input[len..];
        self.position += len;

        Some(chr)
    }

    /// Get the next `n` characters, returning the entire remaining input if `self.input.len() <=
    /// n`
    pub fn next_chars(&mut self, n: usize) -> &'a str {
        if self.input.len() <= n {
            let out = self.input;
            self.position += self.input.len();
            self.input = "";
            return out;
        }

        self.position += n;
        // Split the input at the specified size
        // TODO: Check if we split into a multi-byte character
        let (accumulated, rest) = self.input.split_at(n);
        // Consume the accumulated characters
        self.input = rest;
        // Return the output
        accumulated
    }

    /// Accumulate through the Lexer's input while a given predicate is true.
    pub fn accumulate_while(&mut self, predicate: &dyn Fn(char) -> bool) -> &'a str {
        let mut size = 0;

        while let Some(&chr) = self.lookahead.peek() {
            // we want to continue looping while the predicate is true, if it
            // is false, we will break from the loop.
            if !predicate(chr) {
                break;
            }

            // increment the size by the utf-8 length, otherwise sometimes we
            // can index 'half-way' into a character, which could have weird
            // consequences.
            size += chr.len_utf8();
            self.lookahead.next();
        }

        // Increase the position
        self.position += size;
        // Split the input at the specified size
        let (accumulated, rest) = self.input.split_at(size);
        // Consume the accumulated characters
        self.input = rest;
        // Return the output
        accumulated
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::is_valid_id;

    use super::*;

    #[test]
    #[should_panic]
    fn lexing_invalid_input() {
        let _ = Lexer::new("");
    }

    #[test]
    fn lex_next_char() {
        let input = "+-";
        let mut lx = Lexer::new(input);
        let chr = lx.next_char();

        assert_eq!(chr, Some('+'));
        assert_eq!(lx.input, "-");
        assert_eq!(lx.position, 1);
    }

    #[test]
    fn lex_multibyte_char() {
        let input = "😊+";
        let mut lx = Lexer::new(input);

        let chr = lx.next_char();
        assert_eq!(chr, Some('😊'));
        assert_eq!(lx.input, "+");
        assert_eq!(lx.position, 4);

        // ensure we correctly read the next character
        let chr = lx.next_char();
        assert_eq!(chr, Some('+'));
        assert_eq!(lx.input, "");
        assert_eq!(lx.position, 5);
    }

    #[test]
    fn nop_accumulate_while() {
        let input = "    ";
        let mut lx = Lexer::new(input);
        let out = lx.accumulate_while(&is_valid_id);

        assert_eq!(out, "");
        assert_eq!(lx.input, input);
        assert_eq!(lx.position, 0);
    }

    #[test]
    fn test_next_chars() {
        let input = "abcd";
        let mut lx = Lexer::new(input);
        let out = lx.next_chars(2);

        assert_eq!(out, "ab");
        assert_eq!(lx.input, "cd");
        assert_eq!(lx.position, 2);
    }

    #[test]
    fn test_next_chars_beyond_input() {
        let input = "abcd";
        let mut lx = Lexer::new(input);
        let out = lx.next_chars(20);

        assert_eq!(out, "abcd");
        assert_eq!(lx.input, "");
        assert_eq!(lx.position, 4);
    }
}
