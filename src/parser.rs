use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    context::Context,
    node::{FunctionDeclaration, FunctionExpression, Identifier, Position, Program},
    string::ReadonlyString,
    token::*,
};

lazy_static! {
    static ref REG_IDENTIFIER: Regex = Regex::new("[0-9a-zA-Z$_]").unwrap();
    static ref REG_STRING_BOUNDARY: Regex = Regex::new("['\"]").unwrap();
    static ref REG_NUMBERIC: Regex = Regex::new(r"[0-9]").unwrap();
    static ref REG_OPERATOR: Regex = Regex::new(r"[=!<>+\-*/%?=(){}\[\];:]").unwrap();
    static ref REG_WHITESPACE: Regex = Regex::new(r"\s").unwrap();
    static ref REG_LINE_BREAK: Regex = Regex::new(r"[\n\r]").unwrap();
}
fn get_char(src: &ReadonlyString, position: usize) -> &str {
    src.slice(position, position + 1)
}

fn get_operator_by_chars(chars: &str) -> Option<Token> {
    match chars {
        "==" => Some(Token::Comparation(Comparation::DoubleE)),
        "===" => Some(Token::Comparation(Comparation::TripleE)),
        "!=" => Some(Token::Comparation(Comparation::DoubleNE)),
        "!==" => Some(Token::Comparation(Comparation::TripleNE)),
        "<" => Some(Token::Comparation(Comparation::LT)),
        "<=" => Some(Token::Comparation(Comparation::LTE)),
        ">" => Some(Token::Comparation(Comparation::GT)),
        ">=" => Some(Token::Comparation(Comparation::GTE)),

        "+" => Some(Token::Arithmetic(Arithmetic::Plus)),
        "-" => Some(Token::Arithmetic(Arithmetic::Minus)),
        "*" => Some(Token::Arithmetic(Arithmetic::Multiple)),
        "/" => Some(Token::Arithmetic(Arithmetic::Divide)),
        "%" => Some(Token::Arithmetic(Arithmetic::Modulo)),

        "=" => Some(Token::Assign(Assign::Normal)),
        "+=" => Some(Token::Assign(Assign::Addition)),
        "-=" => Some(Token::Assign(Assign::Subtraction)),
        "*=" => Some(Token::Assign(Assign::Multiplication)),
        "/=" => Some(Token::Assign(Assign::Division)),
        "??=" => Some(Token::Assign(Assign::NullishCoalescing)),

        "(" => Some(Token::ParenL),
        ")" => Some(Token::ParenR),
        "[" => Some(Token::BracketL),
        "]" => Some(Token::BracketR),
        "{" => Some(Token::BraceL),
        "}" => Some(Token::BraceR),

        "." => Some(Token::Dot),
        "?." => Some(Token::QuestionDot),
        ";" => Some(Token::Semi),
        "," => Some(Token::Comma),
        ":" => Some(Token::Colon),
        "?" => Some(Token::Question),
        "??" => Some(Token::NullishCoalesce),
        "!" => Some(Token::LogicalInversion),
        "~" => Some(Token::BitwiseInversion),
        "||" => Some(Token::LogicalOR),
        "|" => Some(Token::BitwiseOR),
        "&&" => Some(Token::LogicalAND),
        "&" => Some(Token::BitwiseAND),
        "++" => Some(Token::Increment),
        "--" => Some(Token::Decrement),
        "=>" => Some(Token::Arrow),

        _ => None,
    }
}

fn read_string(
    src: &ReadonlyString,
    position: &mut usize,
    tokens: &mut Vec<Token>,
    line: usize,
    column: &mut usize,
) {
    let start = *position;
    let boundary = get_char(src, start); // ' or "

    // read content
    *position += 1;

    // escape context flag
    let mut esc = false;

    // read until boundary or line break
    let mut current_char = get_char(src, *position);
    while *position < src.length
        && (current_char != boundary || esc)
        && !REG_LINE_BREAK.is_match(current_char)
    {
        if esc {
            esc = false;
        } else if current_char == "\\" {
            esc = true;
        }

        *position += 1;
        *column += 1;
        current_char = get_char(src, *position);
    }

    // unexpected boundary such as line break or ending of code
    if current_char != boundary {
        panic!(
            "Unexpected character '{}' at line:{}, column:{}.",
            match current_char {
                "\n" => "\\n",
                "\r" => "\\r",
                _ => "",
            },
            line,
            column
        );
    }

    // ready to read next token
    *position += 1;

    let raw = src.slice(start, *position);
    let content = utf8_slice::slice(raw, 1, utf8_slice::len(raw));
    tokens.push(Token::String(raw.to_string(), content.to_string()));
}

fn read_numberic(
    src: &ReadonlyString,
    position: &mut usize,
    tokens: &mut Vec<Token>,
    _line: usize,
    column: &mut usize,
) {
    let start = *position;
    let mut current_char = get_char(src, start);

    // find number system
    let system = match (current_char, get_char(src, start + 1)) {
        ("0", "b") => {
            *position += 2;
            NumberSystem::Binary
        }
        ("0", "x") => {
            *position += 2;
            NumberSystem::Hex
        }
        ("0", _) => {
            *position += 1;
            NumberSystem::Octal
        }
        _ => NumberSystem::Decimal,
    };

    // cannot use separator at the begining of numeric content
    current_char = get_char(src, *position);
    if current_char == "_" {
        panic!("Numeric separators are not allowed at the first of numeric literals");
    }

    let content_start = *position;

    // separator context flag
    let mut separate = false;

    // read until non-numeric except numeric separator
    while *position < src.length && (REG_NUMBERIC.is_match(current_char) || current_char == "_") {
        // cannot use separator constantly
        if current_char == "_" {
            if separate {
                panic!("Only one underscore is allowed as numeric separator");
            } else {
                separate = true;
            }
        } else {
            separate = false;
        }

        *position += 1;
        *column += 1;
        current_char = get_char(src, *position);
    }

    if current_char == "n" {
        // parse bigint
        let raw = src.slice(start, *position + 1);
        let content = src.slice(content_start, *position);
        tokens.push(Token::Bigint(
            raw.to_string(),
            system,
            content.parse::<i128>().unwrap(),
        ));
    } else {
        // parse number
        let raw = src.slice(start, *position);
        let content = src.slice(content_start, *position);
        tokens.push(Token::Number(
            raw.to_string(),
            system,
            content.parse::<f64>().unwrap(),
        ));
    }
}

fn read_reg_exp(
    src: &ReadonlyString,
    position: &mut usize,
    tokens: &mut Vec<Token>,
    line: usize,
    column: &mut usize,
) {
    let start = *position;

    // read content
    *position += 1;

    let mut esc = false;
    let mut current_char = get_char(src, *position);
    while *position < src.length
        && (current_char != "/" || esc)
        && !REG_LINE_BREAK.is_match(current_char)
    {
        if esc {
            esc = false;
        } else if current_char == "\\" {
            esc = true;
        }

        *position += 1;
        *column += 1;
        current_char = get_char(src, *position);
    }

    // unexpected boundary such as line break or ending of code
    if current_char != "/" && current_char != "i" && current_char != "g" {
        panic!(
            "Unexpected character '{}' at line:{}, column:{}.",
            match current_char {
                "\n" => "\\n",
                "\r" => "\\r",
                _ => current_char,
            },
            line,
            column
        );
    }

    *position += 1;

    let modifier = match src.slice(*position, *position + 1) {
        "i" => {
            *position += 1;
            Some(RegExpModifier::I)
        }
        "g" => {
            *position += 1;
            Some(RegExpModifier::G)
        }
        _ => None,
    };

    let raw = src.slice(start, *position);
    let content = src.slice(
        start + 1,
        match modifier {
            Some(_) => *position - 2,
            None => *position,
        },
    );
    tokens.push(Token::RegExp(
        raw.to_string(),
        content.to_string(),
        modifier,
    ));
}

fn read_private_name(
    src: &ReadonlyString,
    position: &mut usize,
    tokens: &mut Vec<Token>,
    _line: usize,
    column: &mut usize,
) {
    let start = *position;

    // read name
    *position += 1;

    while *position < src.length && REG_IDENTIFIER.is_match(get_char(src, *position)) {
        *position += 1;
        *column += 1;
    }

    let raw = src.slice(start, *position);
    let content = src.slice(start + 1, *position);
    tokens.push(Token::PrivateName(raw.to_string(), content.to_string()));
}

fn read_keyword_or_name(
    src: &ReadonlyString,
    context: &mut Context,
    position: &mut usize,
    tokens: &mut Vec<Token>,
    line: usize,
    column: &mut usize,
) {
    let start = *position;
    let startPos = Position::new(line, column);
    while *position < src.length && REG_IDENTIFIER.is_match(get_char(src, *position)) {
        *position += 1;
        *column += 1;
    }
    let identifier = src.slice(start, *position);
    let token = match identifier {
        "var" => Token::Var,
        "let" => Token::Let,
        "const" => Token::Const,

        "function" => Token::Function,
        "return" => Token::Return,

        "for" => Token::For,
        "of" => Token::Of,
        "do" => Token::Do,
        "while" => Token::While,
        "break" => Token::Break,
        "continue" => Token::Continue,

        "switch" => Token::Switch,
        "case" => Token::Case,

        "throw" => Token::Throw,
        "try" => Token::Try,
        "catch" => Token::Catch,
        "finally" => Token::Finally,

        "if" => Token::If,
        "else" => Token::Else,

        "new" => Token::New,
        "this" => Token::This,
        "super" => Token::Super,
        "delete" => Token::Delete,
        "class" => Token::Class,
        "extends" => Token::Extends,
        "instanceof" => Token::Instanceof,
        "typeof" => Token::Typeof,

        "import" => Token::Import,
        "export" => Token::Export,
        "default" => Token::Default,

        "null" => Token::Null,
        "undefined" => Token::Undefined,
        "true" => Token::True,
        "false" => Token::False,
        "void" => Token::Void,

        "in" => Token::In,

        _ => {
            context.is_function_identifier = false;
            Token::Name(identifier.to_string())
        }
    };

    // validate token
    match token {
        Token::Name(_) => {
            context.is_function_identifier = false;
            context
                .statements
                .push(FunctionDeclaration::new(Identifier::new(identifier)))
        }
        Token::Function => {
            if let Some(expressions) = &mut context.expressions {
                expressions.push(Box::new(FunctionExpression::new()));
            }
            context.is_function_identifier = true;
        }
        _ => {
            if context.is_function_identifier {
                panic!(
                    "Unexpected token '{}' at line:{}, column:{}",
                    identifier, line, column
                );
            }
        }
    };

    tokens.push(token);
}

fn read_identifier(
    src: &ReadonlyString,
    context: &mut Context,
    position: &mut usize,
    tokens: &mut Vec<Token>,
    line: usize,
    column: &mut usize,
) {
    let start = *position;
    let first_char = src.slice(start, start + 1);
    if REG_NUMBERIC.is_match(first_char) {
        read_numberic(src, position, tokens, line, column);
    } else if first_char == "/" {
        read_reg_exp(src, position, tokens, line, column);
    } else if first_char == "#" {
        read_private_name(src, position, tokens, line, column);
    } else {
        read_keyword_or_name(src, context, position, tokens, line, column);
    }
}

fn read_operator(
    src: &ReadonlyString,
    position: &mut usize,
    tokens: &mut Vec<Token>,
    line: usize,
    column: &mut usize,
) {
    let char3 = src.slice(*position, *position + 3);

    match get_operator_by_chars(char3) {
        Some(token) => {
            *position += 3;
            *column += 3;
            tokens.push(token);
        }
        None => {
            let char2 = src.slice(*position, *position + 2);
            match get_operator_by_chars(char2) {
                Some(token) => {
                    *position += 2;
                    *column += 2;
                    tokens.push(token);
                }
                None => {
                    let char1 = get_char(src, *position);
                    *position += 1;
                    *column += 1;
                    tokens.push(
                        get_operator_by_chars(char1).expect(
                            format!(
                                "Unexpected character '{}' at line:{}, column:{}.",
                                char1, line, column,
                            )
                            .as_str(),
                        ),
                    );
                }
            }
        }
    }
}

fn find_prev_char_ignore_whitespace(str: &ReadonlyString, start: usize) -> &str {
    if start == 0 {
        return "";
    }

    let mut i: isize = (start - 1) as isize;
    let mut current_char = get_char(str, i as usize);
    while i >= 0 && REG_WHITESPACE.is_match(current_char) {
        i -= 1;
        current_char = get_char(str, i as usize);
    }
    match i {
        -1 => "",
        _ => current_char,
    }
}

fn validate_token(context: &Context, char: &str, line: usize, column: usize) {
    if context.is_function_identifier {
        panic!(
            "Unexpected character '{}' at line:{}, column:{}",
            char, line, column
        );
    }
}

pub fn parse(src: &str) -> Program {
    let readonly_string = ReadonlyString::new(src);
    let mut position: usize = 0;
    let mut line: usize = 1;
    let mut column: usize = 0;
    let mut tokens: Vec<Token> = vec![];
    let mut program = Program::new(line, column);
    let mut context = Context::new(&mut program.body);

    while position < readonly_string.length {
        let char = get_char(&readonly_string, position);

        if REG_WHITESPACE.is_match(char) {
            position += 1;
            if REG_LINE_BREAK.is_match(char) {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        } else if char == "/" {
            validate_token(&context, char, line, column);
            let prev_char = find_prev_char_ignore_whitespace(&readonly_string, position);
            if REG_IDENTIFIER.is_match(prev_char) || prev_char == ")" || prev_char == "]" {
                read_operator(
                    &readonly_string,
                    &mut position,
                    &mut tokens,
                    line,
                    &mut column,
                );
            } else {
                read_reg_exp(
                    &readonly_string,
                    &mut position,
                    &mut tokens,
                    line,
                    &mut column,
                );
            }
        } else if REG_STRING_BOUNDARY.is_match(char) {
            validate_token(&context, char, line, column);
            read_string(
                &readonly_string,
                &mut position,
                &mut tokens,
                line,
                &mut column,
            );
        } else if char == "#" {
            validate_token(&context, char, line, column);
            read_private_name(
                &readonly_string,
                &mut position,
                &mut tokens,
                line,
                &mut column,
            );
        } else if REG_IDENTIFIER.is_match(char) {
            read_identifier(
                &readonly_string,
                &mut context,
                &mut position,
                &mut tokens,
                line,
                &mut column,
            );
        } else {
            validate_token(&context, char, line, column);
            read_operator(
                &readonly_string,
                &mut position,
                &mut tokens,
                line,
                &mut column,
            );
        }
    }
    program
}
