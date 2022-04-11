use std::fs::read_to_string;

use serde::{Serialize, Deserialize};

fn main() -> anyhow::Result<()> {
    let input = read_to_string("kenken1.ron")?;
    let kenken: KenKen = ron::from_str(&input)?;

    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct Field(u16, u16);

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
enum Type {
    Mul,
    Sub,
    Add,
    Div,
}

#[derive(Debug, Deserialize, Serialize)]
struct Area {
    ty: Type,
    solution: u16,
    fields: Vec<Field>,
}

impl Area {
    fn possible_assignments(&self) -> Vec<Assignment> {
        match self.ty {
            Type::Add => {
                let mut asg = Vec::<Assignment>::new();
                for field in &self.fields {

                }
                asg
            }
            _ => {
                vec![]
            }
        }
    }

    fn sort(&mut self) {
        self.fields.sort()
    }
}

struct Assignment {
    values: Vec<u16>
}

impl Assignment {
    fn empty(fields: &[Field]) -> Self {
        Self {
            values: Vec::<u16>::with_capacity(fields.len())
        }
    }

    fn set(&mut self, idx: usize, value: u16) {
        self.values[idx] = value;
    }

    fn get(&self, idx: usize) -> u16 {
        self.values[idx]
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct KenKen {
    areas: Vec<Area>
}

impl KenKen {
    fn sort_areas(&mut self) {
        for area in &mut self.areas {
            area.sort();
        }
    }

    fn validate(&self) -> ValidationResult {
        Ok(())
    }
}

type ValidationResult = Result<(), Vec<ValidationError>>;


enum ValidationError {
    
}