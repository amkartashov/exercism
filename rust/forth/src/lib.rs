pub type Value = i32;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug)]
pub struct Forth {
    env: env::Env,
    stack: Vec<Value>,
}

impl Forth {
    pub fn new() -> Forth {
        let mut env = env::Env::new();
        env.add_word("+".to_string(), vec![Plus]).ok();
        env.add_word("-".to_string(), vec![Minus]).ok();
        env.add_word("*".to_string(), vec![Mul]).ok();
        env.add_word("/".to_string(), vec![Div]).ok();
        env.add_word("dup".to_string(), vec![Dup]).ok();
        env.add_word("drop".to_string(), vec![Drop]).ok();
        env.add_word("swap".to_string(), vec![Swap]).ok();
        env.add_word("over".to_string(), vec![Over]).ok();

        Self {
            env,
            stack: Vec::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result<()> {
        fn collect_word_definition(
            tokens: &mut impl Iterator<Item = Token>,
        ) -> Result<(String, Vec<Token>)> {
            if let Some(word_token) = tokens.next() {
                let word = word_token.word()?;
                let mut definition_tokens = Vec::new();
                let mut definition_is_malformed = true;
                while let Some(token) = tokens.next() {
                    if token == Semicolon {
                        definition_is_malformed = false;
                        break;
                    }
                    definition_tokens.push(token);
                }
                if definition_is_malformed {
                    Err(Error::InvalidWord)
                } else {
                    Ok((word, definition_tokens))
                }
            } else {
                Err(Error::InvalidWord)
            }
        }

        let mut tokens = input.split_ascii_whitespace().map(Token::from_str);

        while let Some(token) = tokens.next() {
            match token {
                // word definition started
                Colon => {
                    let (word, definition) = collect_word_definition(&mut tokens)?;
                    self.env.add_word(word, definition)?;
                }

                // closing semicolon is consumed by collect_word_definition(),
                // so this must be standalone semicolon and this is a error
                Semicolon => return Err(Error::InvalidWord),

                // unfold word to simple tokens and evaluate each
                WordToken(word) => {
                    for token in self.env.word_tokens_iter(&word)? {
                        token.eval(&mut self.stack)?
                    }
                }

                // this must be simple token which can be evaluated
                _ => {
                    token.eval(&mut self.stack)?;
                }
            }
        }

        Ok(())
    }
}

mod env {
    use super::Error;
    use super::Result;
    use super::Token;

    #[derive(Debug)]
    struct WordDef {
        name: String,
        def: Vec<Token>,
    }
    #[derive(Debug)]
    pub(crate) struct Env {
        words: Vec<WordDef>,
    }

    impl Env {
        pub fn new() -> Self {
            Env { words: Vec::new() }
        }

        pub fn add_word(&mut self, word: String, tokens: Vec<Token>) -> Result<()> {
            // verify that all word tokens refer to previously defined words
            if !tokens.iter().all(|token| match token {
                Token::WordToken(word) => self
                    .words
                    .iter()
                    .position(|w| w.name == word.as_str())
                    .is_some(),
                _ => true,
            }) {
                Err(Error::UnknownWord)
            } else {
                self.words.push(WordDef {
                    name: word,
                    def: tokens,
                });
                Ok(())
            }
        }

        pub fn word_tokens_iter(&self, word: &str) -> Result<WordTokensIterator> {
            if self.words.iter().position(|w| w.name == word).is_some() {
                Ok(WordTokensIterator::new(word, &self.words[..]))
            } else {
                Err(Error::UnknownWord)
            }
        }
    }

    #[derive(Debug)]
    pub(crate) struct WordTokensIterator<'a> {
        env: &'a [WordDef],
        tokens: core::slice::Iter<'a, Token>,
        next_word_iterator: Option<Box<WordTokensIterator<'a>>>,
    }

    impl<'a> WordTokensIterator<'a> {
        fn new(word: &'_ str, env: &'a [WordDef]) -> Self {
            let word_pos = env.iter().rposition(|w| w.name == word).unwrap();
            Self {
                env: &env[..word_pos],
                tokens: env[word_pos].def.iter(),
                next_word_iterator: None,
            }
        }

        fn next_from_tokens(&mut self) -> Option<&'a Token> {
            if let Some(token) = self.tokens.next() {
                match token {
                    Token::WordToken(word) => {
                        self.next_word_iterator =
                            Some(Box::new(WordTokensIterator::new(word, self.env)));
                        self.next_from_next_word()
                    }
                    _ => Some(token),
                }
            } else {
                None
            }
        }

        fn next_from_next_word(&mut self) -> Option<&'a Token> {
            if let Some(token) = self.next_word_iterator.as_mut().unwrap().next() {
                Some(token)
            } else {
                self.next_word_iterator = None;
                self.next_from_tokens()
            }
        }
    }

    impl<'a> Iterator for WordTokensIterator<'a> {
        type Item = &'a Token;

        fn next(&mut self) -> Option<Self::Item> {
            if self.next_word_iterator.is_some() {
                self.next_from_next_word()
            } else {
                self.next_from_tokens()
            }
        }
    }
}

use Token::*;
#[derive(Debug, PartialEq, Clone)]
enum Token {
    Colon,
    Semicolon,
    Plus,
    Minus,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    ValueToken(Value),
    WordToken(String),
}

impl Token {
    fn from_str(s: &str) -> Self {
        if let Ok(val) = s.parse::<Value>() {
            ValueToken(val)
        } else {
            match s {
                ":" => Colon,
                ";" => Semicolon,
                s => WordToken(s.to_lowercase()),
            }
        }
    }

    fn word(self) -> Result<String> {
        match self {
            WordToken(word) => Ok(word),
            _ => Err(Error::InvalidWord),
        }
    }

    // eval any token except for Word, Colon, Semicolon
    fn eval(&self, stack: &mut Vec<Value>) -> Result<()> {
        fn pop(stack: &mut Vec<Value>) -> Result<Value> {
            Ok(stack.pop().ok_or(Error::StackUnderflow)?)
        }
        match self {
            Plus => {
                let x2 = pop(stack)?;
                let x1 = pop(stack)?;
                stack.push(x1 + x2);
            }
            Minus => {
                let x2 = pop(stack)?;
                let x1 = pop(stack)?;
                stack.push(x1 - x2);
            }
            Mul => {
                let x2 = pop(stack)?;
                let x1 = pop(stack)?;
                stack.push(x1 * x2);
            }
            Div => {
                let x2 = pop(stack)?;
                if x2 == 0 {
                    return Err(Error::DivisionByZero);
                }
                let x1 = pop(stack)?;
                stack.push(x1 / x2);
            }
            Dup => {
                let x = pop(stack)?;
                stack.push(x);
                stack.push(x);
            }
            Drop => {
                pop(stack)?;
            }
            Swap => {
                let x2 = pop(stack)?;
                let x1 = pop(stack)?;
                stack.push(x2);
                stack.push(x1);
            }
            Over => {
                let x2 = pop(stack)?;
                let x1 = pop(stack)?;
                stack.push(x1);
                stack.push(x2);
                stack.push(x1);
            }
            ValueToken(val) => stack.push(*val),
            _ => unimplemented!(),
        }

        Ok(())
    }
}
