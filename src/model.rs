#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Integer(i32),
    Float(f32),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub struct ClassProperty {
    pub name: String,
    pub parameters: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub struct Class {
    pub name: String,
    pub properties: Vec<ClassProperty>,
}

#[derive(Debug, PartialEq)]
pub struct EntityProperty {
    pub name: String,
    pub ty: String,
    pub display_name: String,
    pub default_value: String,
    pub description: String,
}

#[derive(Debug, PartialEq)]
pub struct Entity {
    pub class: Class,
    pub name: String,
    pub description: String,
}
