use std::fs::read_to_string;

use serde::{Deserialize, Serialize};

mod asg;
mod print;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("kenken1.ron")?;
    let kenken: KenKen = ron::from_str(&input)?;

    Ok(())
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field(pub u16, pub u16);

impl Field {
    pub fn equal_axis(&self, other: &Self) -> bool {
        self.equal_x_axis(other) || self.equal_y_axis(other)
    }

    pub fn equal_x_axis(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    pub fn equal_y_axis(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
enum Type {
    Mul,
    Sub,
    Add,
    Div,
    Single,
}

#[derive(Debug, Deserialize, Serialize)]
struct Area {
    ty: Type,
    solution: u16,
    fields: Vec<Field>,
}

impl Area {
    pub fn new(ty: Type, solution: u16, fields: Vec<Field>) -> Self {
        Area {
            ty,
            solution,
            fields,
        }
    }

    fn validate(&self) -> ValidationResult {
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct KenKen {
    areas: Vec<Area>,
}

impl KenKen {
    fn validate(&self) -> ValidationResult {
        Ok(())
    }
}

type ValidationResult = Result<(), Vec<ValidationError>>;

enum ValidationError {}
