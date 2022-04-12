use anyhow::Result;
use serde_json::Value;

use crate::kenken::{Area, Field, KenKen, Type};

pub fn parse(base64_input: &str) -> Result<KenKen> {
    let raw = base64::decode(base64_input)?;
    let raw_json: Value = serde_json::from_slice(&raw)?;
    let id = raw_json["id"].to_string().parse::<u64>().unwrap();
    let game = raw_json["data"].to_string();

    let (targets, tmp) = game.split_once('T').unwrap().1.split_once('S').unwrap();

    let (types, tmp) = tmp.split_once('V').unwrap();
    let (verts, horiz) = tmp.split_once('H').unwrap();

    let targets = parse_matrix(&targets);
    let types = parse_matrix(&types);
    let verts = parse_matrix(&verts);
    let horiz = parse_matrix(&horiz);
    let size = targets.len() as u16;

    let mut fields: Vec<Field> = (0..size)
        .flat_map(|x| (0..size).map(move |y| Field(x, y)))
        .collect();

    let mut areas: Vec<Vec<Field>> = vec![];

    while !fields.is_empty() {
        let mut area = vec![];
        let mut search: Vec<Field> = vec![fields.swap_remove(0)];
        while !search.is_empty() {
            let mut new_search: Vec<Field> = vec![];
            for field in search {
                // no border on the right?
                if field.0 < size - 1 && verts[field.1 as usize][field.0 as usize] == "0" {
                    new_search.push(Field(field.0 + 1, field.1));
                }

                // no border on the left?
                if field.0 > 0 && verts[field.1 as usize][field.0 as usize - 1] == "0" {
                    new_search.push(Field(field.0 - 1, field.1));
                }

                // no border below?
                if field.1 < size - 1 && horiz[field.0 as usize][field.1 as usize] == "0" {
                    new_search.push(Field(field.0, field.1 + 1));
                }

                // no border above?
                if field.1 > 0 && horiz[field.0 as usize][field.1 as usize - 1] == "0" {
                    new_search.push(Field(field.0, field.1 - 1));
                }

                area.push(field);
            }

            new_search.sort();
            new_search.dedup();
            new_search.retain(|f| !area.contains(f));
            fields.retain(|f| !new_search.contains(f));

            search = new_search;
        }
        areas.push(area);
    }

    let areas = areas
        .into_iter()
        .map(|area| {
            let mut target = 0;
            let mut ty = Type::Single;

            for f in &area {
                let t = targets[f.1 as usize][f.0 as usize].parse::<u16>().unwrap();
                if t > 0 {
                    target = t;
                }

                let raw_ty = types[f.1 as usize][f.0 as usize];
                if raw_ty != "0" {
                    ty = match raw_ty {
                        "+" => Type::Add,
                        "-" => Type::Sub,
                        "/" => Type::Div,
                        "*" => Type::Mul,
                        _ => todo!(),
                    }
                }
            }

            Area {
                fields: area,
                solution: target,
                ty,
            }
        })
        .collect();

    Ok(KenKen { id, areas, size })
}

fn parse_matrix<'a>(input: &'a str) -> Vec<Vec<&'a str>> {
    input
        .trim()
        .split("\\r\\n")
        .filter(|s| !s.is_empty() && *s != "\"")
        .map(|row| row.split_whitespace().collect())
        .collect()
}
