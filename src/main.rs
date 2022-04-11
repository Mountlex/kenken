use std::fs::read_to_string;

use serde::{Deserialize, Serialize};

mod asg;
mod print;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("kenken1.ron")?;
    let kenken: KenKen = ron::from_str(&input)?;

    let mut asgs = Vec::new();
    for area in &kenken.areas {
        asgs.append(&mut area.possible_assignments());
    }

    print::print(&kenken, asgs, 10)?;

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
pub enum Type {
    Mul,
    Sub,
    Add,
    Div,
    Single,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Area {
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

    fn min_y_field(&self) -> u16 {
        self.fields.iter().map(|f| f.1).min().unwrap()
    }

    fn min_yx_field(&self) -> Field {
        *self.fields.iter().filter(|f| f.1 == self.min_y_field()).min_by_key(|f| f.0).unwrap()
    }

    pub fn id_field<'a>(&'a self, field: Field) -> Option<&'a Self> {
        if field == self.min_yx_field() {
            Some(self)
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KenKen {
    pub areas: Vec<Area>,
    pub size: u16,
}

impl KenKen {
    fn validate(&self) -> ValidationResult {
        Ok(())
    }
    
    pub fn same_area(&self, field1: &Field, field2: &Field) -> bool {
        self.areas.iter().any(|area| area.fields.contains(field1) &&  area.fields.contains(field2))
    }

    pub fn is_id_field<'a>(&'a self, field: Field) -> Option<&'a Area> {
        self.areas.iter().find_map(|a| a.id_field(field))
    }
}

type ValidationResult = Result<(), Vec<ValidationError>>;

enum ValidationError {}