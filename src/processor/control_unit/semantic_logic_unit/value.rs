#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Text(String),
    Number(u8),
    None,
}