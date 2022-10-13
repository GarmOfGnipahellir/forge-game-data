use crate::model::*;
use combine::{
    between, many, many1,
    parser::char::{alpha_num, space},
    satisfy, sep_by, skip_many, token, ParseError, Parser, Stream,
};

fn whitespace<Input>() -> impl Parser<Input>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    skip_many(space())
}

fn value<Input>() -> impl Parser<Input, Output = Value>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many(satisfy(|c| c != ',' && c != ')')).map(|value| Value::String(value))
}

fn class_property<Input>() -> impl Parser<Input, Output = ClassProperty>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        many1(alpha_num()),
        token('('),
        sep_by(value(), token(',').skip(whitespace())),
        token(')'),
    )
        .map(|(name, _, parameters, _)| ClassProperty { name, parameters })
}

fn class<Input>() -> impl Parser<Input, Output = Class>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        token('@'),
        many1(alpha_num()).skip(whitespace()),
        many(class_property().skip(whitespace())),
    )
        .map(|(_, name, properties)| Class { name, properties })
}

fn entity<Input>() -> impl Parser<Input, Output = Entity>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        class().skip(whitespace()),
        token('=').skip(whitespace()),
        many1(alpha_num()).skip(whitespace()),
        token(':').skip(whitespace()),
        between(token('"'), token('"'), many(satisfy(|c| c != '"'))),
    )
        .map(|(class, _, name, _, description)| Entity {
            class,
            name,
            description,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(
            value().parse("PlayerClass").map(|t| t.0),
            Ok(Value::String("PlayerClass".to_string()))
        );
    }

    #[test]
    fn test_class_property() {
        assert_eq!(
            class_property().parse("base(PlayerClass)").map(|t| t.0),
            Ok(ClassProperty {
                name: "base".to_string(),
                parameters: vec![Value::String("PlayerClass".to_string())],
            })
        );
        assert_eq!(
            class_property()
                .parse("size(-16 -16 -24, 16 16 32)")
                .map(|t| t.0),
            Ok(ClassProperty {
                name: "size".to_string(),
                parameters: vec![
                    Value::String("-16 -16 -24".to_string()),
                    Value::String("16 16 32".to_string())
                ],
            })
        );
        assert_eq!(
            class_property().parse("color(0 255 0)").map(|t| t.0),
            Ok(ClassProperty {
                name: "color".to_string(),
                parameters: vec![Value::String("0 255 0".to_string())],
            })
        );
    }

    #[test]
    fn test_class() {
        assert_eq!(
            class().parse("@PointClass").map(|t| t.0),
            Ok(Class {
                name: "PointClass".to_string(),
                properties: vec![]
            })
        );
        assert_eq!(
            class()
                .parse("@baseclass size(-16 -16 -24, 16 16 32) color(0 255 0)")
                .map(|t| t.0),
            Ok(Class {
                name: "baseclass".to_string(),
                properties: vec![
                    ClassProperty {
                        name: "size".to_string(),
                        parameters: vec![
                            Value::String("-16 -16 -24".to_string()),
                            Value::String("16 16 32".to_string())
                        ],
                    },
                    ClassProperty {
                        name: "color".to_string(),
                        parameters: vec![Value::String("0 255 0".to_string())],
                    }
                ]
            })
        );
    }

    #[test]
    fn test_entity() {
        assert_eq!(
            entity()
                .parse(r#"@SolidClass = worldspawn : "World entity" []"#)
                .map(|t| t.0),
            Ok(Entity {
                class: Class {
                    name: "SolidClass".to_string(),
                    properties: vec![]
                },
                name: "worldspawn".to_string(),
                description: "World entity".to_string()
            })
        );
    }
}
