use crate::FieldType;
use std::fmt::{self, Display};

/// Field in a given SDL type.
#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    description: Option<String>,
    //TODO(@lrlna): fields for objects types and interfaces can also take
    //arguments. This struct should also account for that.
    name: String,
    type_: FieldType,
    deprecated: bool,
    deprecation_reason: Option<String>,
}

impl Field {
    /// Create a new instance of Field.
    pub fn new(name: String, type_: FieldType) -> Self {
        Self {
            description: None,
            name,
            type_,
            deprecated: false,
            deprecation_reason: None,
        }
    }

    /// Set the field's description.
    pub fn description(&mut self, description: Option<String>) {
        self.description = description;
    }

    /// Set the field's deprecation properties.
    pub fn deprecated(&mut self, reason: Option<String>) {
        self.deprecated = true;
        self.deprecation_reason = reason;
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(description) = &self.description {
            // Let's indent description on a field level for now, as all fields
            // are always on the same level and are indented by 2 spaces.
            writeln!(f, "  \"\"\"\n  {}\n  \"\"\"", description)?;
        }

        let mut deprecated = String::new();
        if self.deprecated {
            deprecated += " @deprecated";
            // Just in case deprecated field is ever used without a reason,
            // let's properly unwrap this Option.
            if let Some(reason) = &self.deprecation_reason {
                deprecated += &format!("(reason: \"{}\")", reason);
            }
        }
        write!(f, "  {}: {}{}", self.name, self.type_, deprecated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_encodes_simple_fields() {
        let ty_1 = FieldType::Type {
            ty: "SpaceProgram".to_string(),
            default: None,
        };

        let ty_2 = FieldType::List { ty: Box::new(ty_1) };
        let ty_3 = FieldType::NonNull { ty: Box::new(ty_2) };
        let field = Field::new("spaceCat".to_string(), ty_3);

        assert_eq!(field.to_string(), r#"  spaceCat: [SpaceProgram]!"#);
    }

    #[test]
    fn it_encodes_fields_with_deprecation() {
        let ty_1 = FieldType::Type {
            ty: "SpaceProgram".to_string(),
            default: None,
        };

        let ty_2 = FieldType::List { ty: Box::new(ty_1) };
        let mut field = Field::new("cat".to_string(), ty_2);
        field.description(Some("Very good cats".to_string()));
        field.deprecated(Some("Cats are no longer sent to space.".to_string()));

        assert_eq!(
            field.to_string(),
            r#"  """
  Very good cats
  """
  cat: [SpaceProgram] @deprecated(reason: "Cats are no longer sent to space.")"#
        );
    }

    #[test]
    fn it_encodes_fields_with_description() {
        let ty_1 = FieldType::Type {
            ty: "SpaceProgram".to_string(),
            default: None,
        };

        let ty_2 = FieldType::NonNull { ty: Box::new(ty_1) };
        let ty_3 = FieldType::List { ty: Box::new(ty_2) };
        let ty_4 = FieldType::NonNull { ty: Box::new(ty_3) };
        let mut field = Field::new("spaceCat".to_string(), ty_4);
        field.description(Some("Very good space cats".to_string()));

        assert_eq!(
            field.to_string(),
            r#"  """
  Very good space cats
  """
  spaceCat: [SpaceProgram!]!"#
        );
    }

    #[test]
    fn it_encodes_fields_with_arguments() {
        let ty_1 = FieldType::Type {
            ty: "SpaceProgram".to_string(),
            default: None,
        };

        let ty_2 = FieldType::NonNull { ty: Box::new(ty_1) };
        let ty_3 = FieldType::List { ty: Box::new(ty_2) };
        let ty_4 = FieldType::NonNull { ty: Box::new(ty_3) };
        let mut field = Field::new("spaceCat".to_string(), ty_4);
        field.description(Some("Very good space cats".to_string()));

        let arg_1 = FieldType::Type {
            ty: "SpaceProgram".to_string(),
            default: None,
        };

        let arg_2 = FieldType::List {
            ty: Box::new(arg_1),
        };
        let mut arg = FieldArgument::new("cat".to_string(), arg_2);
        arg.deprecated(Some("Cats are no longer sent to space.".to_string()));
        field.arg(arg);

        assert_eq!(
            field.to_string(),
            r#"  """
  Very good space cats
  """
  spaceCat(cat: [SpaceProgram] @deprecated(reason: "Cats are no longer sent to space.")): [SpaceProgram!]!"#
        );
    }
}
