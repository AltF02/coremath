use crate::operators::Operator;


pub struct Token(pub f64, pub Operator);

pub struct Tokens {
    pub tokens: Vec<Token>,
}

impl Tokens {
    pub fn new() -> Tokens {
        Tokens {
            tokens: vec!(),
        }
    }

    pub fn eval(&self) -> f64 {
        let mut result: f64 = 0.0;
        let mut i = 0;        
        
        while i < self.tokens.len() {
            let token = &self.tokens[i];

            match token.1 {
                Operator::Addition => {
                    result += token.0;
                },

                Operator::Subtraction => {
                    result -= token.0;
                },

                Operator::Multiplication => {
                    result *= token.0;
                },

                Operator::Division => {
                    result /= token.0;
                },

                _ => continue,
            }

            i += 1;
        }


        return result;
    }
}

