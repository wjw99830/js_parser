#[derive(Debug)]
pub enum Comparation {
    DoubleE,
    TripleE,
    DoubleNE,
    TripleNE,
    LT,
    LTE,
    GT,
    GTE,
}

#[derive(Debug)]
pub enum Arithmetic {
    Plus,
    Minus,
    Multiple,
    Divide,
    Modulo,
}

#[derive(Debug)]
pub enum Assign {
    Normal,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    NullishCoalescing,
}

pub type Raw = String;

pub type Number = f64;

#[derive(Debug)]
pub enum NumberSystem {
    Binary,
    Octal,
    Decimal,
    Hex,
}

#[derive(Debug)]
pub enum RegExpModifier {
    I,
    G,
}

#[derive(Debug)]
pub enum Token {
    Number(Raw, NumberSystem, Number),
    Bigint(Raw, NumberSystem, i128),
    RegExp(Raw, String, Option<RegExpModifier>),
    String(Raw, String),
    Name(String),
    PrivateName(Raw, String),

    Var,
    Let,
    Const,

    Function,
    Return,

    For,
    Of,
    Do,
    While,
    Break,
    Continue,

    Switch,
    Case,

    Throw,
    Try,
    Catch,
    Finally,

    If,
    Else,

    New,
    This,
    Super,
    Delete,
    Class,
    Extends,
    Instanceof,
    Typeof,

    Import,
    Export,
    Default,

    Null,
    Undefined,
    True,
    False,
    Void,

    In,

    Comparation(Comparation),
    Arithmetic(Arithmetic),
    Assign(Assign),

    ParenL,
    ParenR,
    BracketL,
    BracketR,
    BraceL,
    BraceR,

    Dot,
    QuestionDot,
    Semi,
    Comma,
    Colon,
    Question,
    NullishCoalesce,
    LogicalInversion,
    BitwiseInversion,
    LogicalOR,
    BitwiseOR,
    LogicalAND,
    BitwiseAND,
    Increment,
    Decrement,
    Arrow,
}
