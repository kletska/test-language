#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    text: String,
    kind: TokenKind,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.text)
    }
}

impl Token {
    pub fn new(text: String, kind: TokenKind) -> Token {
        Token {
            text,
            kind,
        }
    }
    
    pub fn get_kind(&self) -> TokenKind {
        self.kind
    }

    pub fn get_text(self) -> String {
        self.text
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenDir {
    Pair,
    Down,
    Stay,
    Forward,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Include, // include
    Load, // load
    Let, // $
    LetEnd, //;
    Type, // :
    Eq, // =
    Prod, // * v o
    Tuple,// , v o
    Sum, // + v o
    Cases,// | v o
    Function, // -> v o
    Lambda, //~> v o
    Universe, // @ v 
    Top, // . v
    Bottom, // ! v
    OpenBracket, // ( v o
    CloseBracket, // ) v
    OpenSquear,  // [
    CloseSquear, // ]
    OpenCurly, // {
    CloseCurly, // }
    Int, // 1234567890 v
    StringLiteral, // /abacaba/ v
    Name, //v 
    Application, // <- v o
}

impl TokenKind {
    pub fn get_prior(self) -> i32 {
        match self {
            TokenKind::Prod => 8,
            TokenKind::Tuple => 16,
            TokenKind::Sum => 10,
            TokenKind::Cases => 14,
            TokenKind::Function => 12,
            TokenKind::Lambda => 4,
            TokenKind::Universe => 0,
            TokenKind::Top => 0,
            TokenKind::Bottom => 0,
            TokenKind::OpenBracket => 2,
            TokenKind::CloseBracket => 18,
            TokenKind::Int => 0,
            TokenKind::StringLiteral => 0,
            TokenKind::Name => 0,
            TokenKind::Application => 6,
            _ => -1
        }
    }

    pub fn get_weight(self) -> i32 {
        match self {
            TokenKind::Prod =>7,
            TokenKind::Tuple => 16,
            TokenKind::Sum => 9,
            TokenKind::Cases => 14,
            TokenKind::Function => 12,
            TokenKind::Lambda => 4,
            TokenKind::OpenBracket => 18,
            TokenKind::Application => 5,
            _ => -1,
        }
    }

    pub fn nary_operation(self) -> i32 {
        match self {
            TokenKind::Prod => 2,
            TokenKind::Tuple => 2,
            TokenKind::Sum => 2,
            TokenKind::Cases => 2,
            TokenKind::Function => 2,
            TokenKind::Lambda => 2,
            TokenKind::Universe => 0,
            TokenKind::Top => 0,
            TokenKind::Bottom => 0,
            TokenKind::Int => 0,
            TokenKind::StringLiteral => 0,
            TokenKind::Name => 0,
            TokenKind::Application => 2,
            _ => -1,
        }
    }

    pub fn next_step(self) -> Option<TokenKind> {
        match self {
            TokenKind::OpenBracket => Some(TokenKind::CloseBracket),
            _ => None,
        }
    }

    pub fn compare(self, another: TokenKind) -> Result<TokenDir, String> {
        if self.get_weight() == -1 || another.get_prior() == -1 {
            return Err("incomparable tokens".to_string());
        }
        if let Some(pair) = self.next_step() {
            if pair == another {
                return Ok(TokenDir::Pair);
            }
        }
        if another.get_prior() == 0 {
            return Ok(TokenDir::Forward);
        }
       
        if another.get_prior() > self.get_weight() {
            Ok(TokenDir::Down)
        } else {
            Ok(TokenDir::Stay)
        }
    }

    fn is_both_applicate(self) -> bool {
        match self {
            TokenKind::Name => true,
            TokenKind::Universe => true,
            TokenKind::Bottom => true,
            TokenKind::Top => true,
            _ => false,
        }
    } 

    fn is_left_applicate(self) -> bool {
        self.is_both_applicate() || self == TokenKind::CloseBracket
    }

    fn is_right_applicate(self) -> bool {
        self.is_both_applicate() || self == TokenKind::OpenBracket
    }

    pub fn is_applicate(first_kind: TokenKind, second_kind: TokenKind) -> bool {
        second_kind.is_left_applicate() && first_kind.is_right_applicate()
    }

    pub fn allow_in_type(self) -> bool {
        match self {
            TokenKind::Application => true,
            TokenKind::Bottom => true,
            TokenKind::OpenBracket => true,
            TokenKind::CloseBracket => true,
            TokenKind::Prod => true,
            TokenKind::Sum => true,
            TokenKind::Top => true,
            TokenKind::Universe => true,
            TokenKind::Name => true,
            TokenKind::Function => true,
            _ => false,
        }
    }

    pub fn allow_in_value(self) -> bool {
        let pure_value = match self {
            TokenKind::Lambda => true,
            TokenKind::Tuple => true,
            TokenKind::Cases => true,
            TokenKind::StringLiteral => true,
            TokenKind::Int => true,
            _ => false, 
        };
        self.allow_in_type() || pure_value
    }
}

pub struct RawToken {
    pub text: String,
    pub kind: RawTokenKind,
}

impl RawToken {
    pub fn new(text: String, kind: RawTokenKind) -> RawToken {
        RawToken {
            text,
            kind,
        }
    }
}

pub enum RawTokenKind {
    StringLiteral,
    DontKnow,
}