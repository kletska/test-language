use crate::inner_representation::token::{TokenKind};

pub const separating_symbols: &[&'static str] =
    &["$", ";", ":", "=", "'",
      "*", ",", "+", "|", "->", "~>", 
      "@", ".", "!", "(", ")", "[", "]", "{", "}"
    ];
pub const special_symbols: &[&'static str] = 
    &["include", "load", "$", ";", ":", "=", 
    "*", ",", "+", "|", "->", "~>", "@", ".", "!", 
    "(", ")", "[", "]", "{", "}"];
pub const special_tokens: &[TokenKind] = &[
    TokenKind::Include,
    TokenKind::Load,
    TokenKind::Let,
    TokenKind::LetEnd,
    TokenKind::Type,
    TokenKind::Eq,
    TokenKind::Prod,
    TokenKind::Tuple,
    TokenKind::Sum,
    TokenKind::Cases,
    TokenKind::Function,
    TokenKind::Lambda,
    TokenKind::Universe,
    TokenKind::Top,
    TokenKind::Bottom,
    TokenKind::OpenBracket,
    TokenKind::CloseBracket,
    TokenKind::OpenSquear,
    TokenKind::CloseSquear,
    TokenKind::OpenCurly,
    TokenKind::CloseCurly,
];

