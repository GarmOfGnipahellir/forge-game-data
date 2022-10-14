use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Value {
    String(String),
    Integer(i32),
    Float(f32),
    Boolean(bool),
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ClassProperty {
    pub name: String,
    pub parameters: Vec<Value>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Class {
    pub name: String,
    pub properties: Vec<ClassProperty>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct EntityProperty {
    pub name: String,
    pub ty: String,
    pub display_name: String,
    pub default_value: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Entity {
    pub class: Class,
    pub name: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_ser_de_value() {
        assert_tokens(
            &Value::String("Test string".to_string()),
            &[
                Token::Enum { name: "Value" },
                Token::Str("String"),
                Token::String("Test string"),
            ],
        )
    }

    #[test]
    fn test_ser_de_entity() {
        assert_tokens(
            &Entity {
                class: Class {
                    name: "SolidClass".to_string(),
                    properties: vec![],
                },
                name: "worldspawn".to_string(),
                description: "World entity".to_string(),
            },
            &[
                Token::Struct {
                    name: "Entity",
                    len: 3,
                },
                Token::Str("class"),
                Token::Struct {
                    name: "Class",
                    len: 2,
                },
                Token::Str("name"),
                Token::String("SolidClass"),
                Token::Str("properties"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
                Token::Str("name"),
                Token::String("worldspawn"),
                Token::Str("description"),
                Token::String("World entity"),
                Token::StructEnd,
            ],
        )
    }
}
