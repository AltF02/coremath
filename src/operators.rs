pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Unknown(char),
}

pub fn is_operator(symbol: &char) -> bool {
    match symbol {
        '+' | '-' | '/' | '*' => true,
        _ => false,
    }
}

pub fn as_operator(symbol: &char) -> Operator {
    match symbol {
        '+' => Operator::Addition,
        '-' => Operator::Subtraction,
        '/' => Operator::Division,
        '*' => Operator::Multiplication,
        symbol => Operator::Unknown(symbol.clone()),
    }
}

