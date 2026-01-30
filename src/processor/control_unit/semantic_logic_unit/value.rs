#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Text(String),
    Number(u8),
    Audio(Vec<u8>),
    None,
}