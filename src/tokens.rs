#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ingredient,
    Taste,
    TasteAgain,
    Tasteless,
    Layer,
    Simmer,
    Cook,
    Burnt,
    Serve,
    Plate,
    Recipe,
    Fresh,
    Spoiled,
    Mix,
    Separate,
    Number(isize),
    Identifier(String),
    StringLiteral(String),
    LeftBracket,
    RightBracket,
    Is,
    ArrayNumber(Vec<isize>),
    ArrayString(Vec<String>),
    Hotter,
    Cooler,
    Same,
    NotSame,
    HotOrSame,
    CoolOrSame,
    In,
    Space,
    Block(Vec<Token>),
}
