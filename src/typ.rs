
pub enum Type {
    Int,
    Arrow(Box<Type, Box<Type>),
}

