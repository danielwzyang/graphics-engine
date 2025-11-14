pub struct Token {
    value: String,
    token_type: TokenType
}

pub enum TokenType {
    Define,
    Command,
    Number,
    AxisOfRotation,
}
