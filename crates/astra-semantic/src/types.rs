use std::fmt;

/// The semantic type used during analysis.
///
/// This is independent of the parser's AST types.
/// During semantic analysis, AST types are resolved into
/// these concrete semantic types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SemanticType {
    Unknown,

    Void,

    Bool,

    Char,

    String,

    Int,

    Float,

    U8,
    U16,
    U32,
    U64,

    I8,
    I16,
    I32,
    I64,

    Usize,
    Isize,

    Array(Box<SemanticType>),

    Function {
        parameters: Vec<SemanticType>,
        return_type: Box<SemanticType>,
    },

    Struct(String),

    Enum(String),

    Trait(String),

    Module(String),

    Custom(String),
}

impl SemanticType {
    /// Returns true if this is an integer type.
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            Self::U8
                | Self::U16
                | Self::U32
                | Self::U64
                | Self::I8
                | Self::I16
                | Self::I32
                | Self::I64
                | Self::Usize
                | Self::Isize
                | Self::Int
        )
    }

    /// Returns true if this is a floating point type.
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float)
    }

    /// Returns true if this is a numeric type.
    pub fn is_numeric(&self) -> bool {
        self.is_integer() || self.is_float()
    }

    /// Returns true if this is callable.
    pub fn is_callable(&self) -> bool {
        matches!(self, Self::Function { .. })
    }

    /// Returns true if this is a user-defined type.
    pub fn is_user_defined(&self) -> bool {
        matches!(
            self,
            Self::Struct(_)
                | Self::Enum(_)
                | Self::Trait(_)
                | Self::Custom(_)
        )
    }

    /// Returns whether two semantic types are compatible.
    pub fn is_assignable_from(
        &self,
        other: &SemanticType,
    ) -> bool {
        if self == other {
            return true;
        }

        match (self, other) {
            (Self::Unknown, _)
            | (_, Self::Unknown) => true,

            _ => false,
        }
    }
}

impl fmt::Display for SemanticType {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Void => write!(f, "void"),
            Self::Bool => write!(f, "bool"),
            Self::Char => write!(f, "char"),
            Self::String => write!(f, "string"),
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),

            Self::U8 => write!(f, "u8"),
            Self::U16 => write!(f, "u16"),
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),

            Self::I8 => write!(f, "i8"),
            Self::I16 => write!(f, "i16"),
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),

            Self::Usize => write!(f, "usize"),
            Self::Isize => write!(f, "isize"),

            Self::Array(inner) => {
                write!(f, "[{}]", inner)
            }

            Self::Function {
                parameters,
                return_type,
            } => {
                write!(f, "fn(")?;

                for (i, parameter) in parameters.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{parameter}")?;
                }

                write!(f, ") -> {}", return_type)
            }

            Self::Struct(name)
            | Self::Enum(name)
            | Self::Trait(name)
            | Self::Module(name)
            | Self::Custom(name) => {
                write!(f, "{name}")
            }
        }
    }
}
