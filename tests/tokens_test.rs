

#[cfg(test)]
mod tests {
    use coremath::tokens::{
        Token,
        Tokens,
    };
    use coremath::operators::Operator;

    #[test]
    fn test_tokens() {
        let mut tokens = Tokens::new();

        let token_1 = Token(10.0, Operator::Addition);
        let token_2 = Token(2.0,  Operator::Subtraction);
        let token_3 = Token(2.0,  Operator::Multiplication);
        let token_4 = Token(2.0,  Operator::Division);

        tokens.tokens.push(token_1);
        tokens.tokens.push(token_2);
        tokens.tokens.push(token_3);
        tokens.tokens.push(token_4);

        let answer = 8.0;

        assert_eq!(answer, tokens.eval());
    }
}
