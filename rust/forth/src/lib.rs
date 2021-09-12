pub type Value = i32;
pub type Word = String;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
    WrongExpression,
}

#[derive(Debug)]
pub struct Forth {
    words: Vec<WordDef>,
    stack: Vec<Value>,
}

impl Forth {
    fn pop(&mut self) -> Result<Value> {
        Ok(self.stack.pop().ok_or(Error::StackUnderflow)?)
    }

    fn word_pos(&self, name: &str, env_pos: usize) -> Option<usize> {
        self.words
            .iter()
            .rev()
            .skip(env_pos)
            .position(|w| w.name == name)
            .map(|p| p + env_pos + 1)
    }

    pub fn new() -> Forth {
        Self {
            words: vec![
                WordDef {
                    name: "+".to_string(),
                    def: vec![Plus],
                },
                WordDef {
                    name: "-".to_string(),
                    def: vec![Minus],
                },
                WordDef {
                    name: "*".to_string(),
                    def: vec![Mul],
                },
                WordDef {
                    name: "/".to_string(),
                    def: vec![Div],
                },
                WordDef {
                    name: "dup".to_string(),
                    def: vec![Dup],
                },
                WordDef {
                    name: "drop".to_string(),
                    def: vec![Drop],
                },
                WordDef {
                    name: "swap".to_string(),
                    def: vec![Swap],
                },
                WordDef {
                    name: "over".to_string(),
                    def: vec![Over],
                },
            ],
            stack: Vec::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn eval_no_defs_tokens(&mut self, tokens: &[Token], env_pos: usize) -> Result<()> {
        //dbg!(&tokens);
        for t in tokens {
            match t {
                Plus => {
                    let x2 = self.pop()?;
                    let x1 = self.pop()?;
                    self.stack.push(x1 + x2);
                }
                Minus => {
                    let x2 = self.pop()?;
                    let x1 = self.pop()?;
                    self.stack.push(x1 - x2);
                }
                Mul => {
                    let x2 = self.pop()?;
                    let x1 = self.pop()?;
                    self.stack.push(x1 * x2);
                }
                Div => {
                    let x2 = self.pop()?;
                    if x2 == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    let x1 = self.pop()?;
                    self.stack.push(x1 / x2);
                }
                Dup => {
                    let x = self.pop()?;
                    self.stack.push(x);
                    self.stack.push(x);
                }
                Drop => {
                    dbg!(&self);
                    self.pop()?;
                }
                Swap => {
                    let x2 = self.pop()?;
                    let x1 = self.pop()?;
                    self.stack.push(x2);
                    self.stack.push(x1);
                }
                Over => {
                    let x2 = self.pop()?;
                    let x1 = self.pop()?;
                    self.stack.push(x1);
                    self.stack.push(x2);
                    self.stack.push(x1);
                }
                ValueToken(val) => self.stack.push(*val),
                WordToken(word) => {
                    let word_pos = self.word_pos(word, env_pos).ok_or(Error::UnknownWord)?;
                    let word_tokens = self.words[self.words.len() - word_pos].def.clone();
                    self.eval_no_defs_tokens(&word_tokens, word_pos)?;
                }
                _ => unimplemented!(),
            }
        }
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result<()> {
        let mut tokens = input.split_ascii_whitespace().map(Token::from_str);

        let mut inside_definition = false;

        while let Some(t) = tokens.next() {
            let t = t?;

            if inside_definition {
                let word_name = word_from_token(t)?;
                let mut word_def = Vec::new();
                while let Some(nt) = tokens.next() {
                    let nt = nt?;
                    if nt == Semicolon {
                        inside_definition = false;
                        break;
                    }
                    word_def.push(nt);
                }

                for t in word_def.iter() {
                    match t {
                        WordToken(word) => {
                            if self.word_pos(&word, 0).is_none() {
                                return Err(Error::UnknownWord);
                            }
                        }
                        // word def can't include other word def
                        Colon => return Err(Error::WrongExpression),
                        _ => {}
                    }
                }
                self.words.push(WordDef {
                    name: dbg!(word_name),
                    def: dbg!(word_def),
                });

                continue;
            }

            match t {
                Colon => {
                    inside_definition = true;
                }
                Semicolon => return Err(Error::WrongExpression),
                t => {
                    let mut tokens_wo_defs = vec![t];
                    while let Some(nt) = tokens.next() {
                        let nt = nt?;
                        if nt == Colon {
                            inside_definition = true;
                            break;
                        }
                        tokens_wo_defs.push(nt);
                    }
                    self.eval_no_defs_tokens(&tokens_wo_defs, 0)?;
                }
            }
        }

        if inside_definition {
            Err(Error::InvalidWord)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
struct WordDef {
    name: Word,
    def: Vec<Token>,
}

use std::str::FromStr;

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
    WordToken(Word),
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(if let Ok(val) = s.parse::<Value>() {
            ValueToken(val)
        } else {
            match s {
                ":" => Colon,
                ";" => Semicolon,
                s => WordToken(s.to_lowercase()),
            }
        })
    }
}

//  "word is a sequence of one or more letters, digits, symbols or punctuation that is not a number"
// now only checking for alphanumeric
fn word_from_token(t: Token) -> Result<Word> {
    match t {
        WordToken(word) => Ok(word),
        _ => Err(Error::InvalidWord),
    }
}
