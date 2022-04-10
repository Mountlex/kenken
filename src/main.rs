use std::fs::read_to_string;

use anyhow::Result;
use serde::{Serialize, Deserialize};

fn main() -> Result<()> {
    let input = read_to_string("kenken1.ron")?;
    let kenken: KenKen = ron::from_str(&input)?;

    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Deserialize, Serialize)]
struct KenKen {
    areas: Vec<Area>
}

