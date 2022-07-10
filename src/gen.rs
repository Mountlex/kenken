use std::vec;

use rand::{prelude::IteratorRandom, Rng};

use crate::{
    asg::Assignment,
    kenken::{Area, Field, KenKen, Type},
};

fn add_field_biased(kenken: &mut KenKen, to_add: Field, neighbor: &Field, max_area: u16) -> bool {
    let mut rng = rand::thread_rng();
    let area = kenken.get_area_mut(neighbor).unwrap();
    if area.size() == 1 && rng.gen_bool(0.85) {
        area.fields.push(to_add);
        return true;
    } else if rng.gen_ratio(
        (max_area - area.size().min(max_area)) as u32 / 2,
        max_area as u32,
    ) {
        area.fields.push(to_add);
        return true;
    } else {
        return false;
    }
}

fn random_area_gen(size: u16) -> KenKen {
    let mut kenken = KenKen {
        id: 0,
        areas: vec![],
        size,
    };

    let max_area = size / 2 + 1;

    for i in 0..size {
        for j in 0..size {
            let mut added = false;
            if rand::random() {
                if j > 0 && (i > 0 || rand::random()) {
                    added = add_field_biased(&mut kenken, Field(i, j), &Field(i, j - 1), max_area);
                }
                if !added && i > 0 {
                    added = add_field_biased(&mut kenken, Field(i, j), &Field(i - 1, j), max_area);
                }
            } else {
                if i > 0 && (j > 0 || rand::random()) {
                    added = add_field_biased(&mut kenken, Field(i, j), &Field(i - 1, j), max_area);
                }
                if !added && j > 0 {
                    added = add_field_biased(&mut kenken, Field(i, j), &Field(i, j - 1), max_area);
                }
            }
            if !added {
                let area = Area {
                    ty: Type::Single,
                    solution: 0,
                    fields: vec![Field(i, j)],
                };
                kenken.areas.push(area);
            }
        }
    }

    kenken
}

fn random_solution(size: u16) -> Assignment {
    let mut sol = Assignment::empty();
    let mut rng = rand::thread_rng();

    for i in 0..size {
        for j in 0..size {
            sol.set(Field(i, j), ((i + j) % size) + 1);
        }
    }

    for _ in 0..(size * size) {
        let swap = (0..size).choose_multiple(&mut rng, 2);
        if rand::random() {
            for i in 0..size {
                let tmp = sol.get(&Field(swap[0], i)).unwrap();
                sol.set(Field(swap[0], i), sol.get(&Field(swap[1], i)).unwrap());
                sol.set(Field(swap[1], i), tmp);
            }
        } else {
            for i in 0..size {
                let tmp = sol.get(&Field(i, swap[0])).unwrap();
                sol.set(Field(i, swap[0]), sol.get(&Field(i, swap[1])).unwrap());
                sol.set(Field(i, swap[1]), tmp);
            }
        }
    }

    sol
}

pub fn generate(size: u16) -> KenKen {
    let mut kenken = random_area_gen(size);
    let sol = random_solution(size);

    for area in &mut kenken.areas {
        if area.fields.len() == 1 {
            area.solution = sol.get(&area.fields[0]).unwrap();
        } else if area.fields.len() == 2 {
            let f1 = sol.get(&area.fields[0]).unwrap();
            let f2 = sol.get(&area.fields[1]).unwrap();
            let divisible = (f1 > f2 && f1 % f2 == 0) || (f2 > f1 && f2 % f1 == 0);

            if divisible && rand::random() {
                area.ty = Type::Div;
                if f1 > f2 {
                    area.solution = f1 / f2;
                } else {
                    area.solution = f2 / f1;
                }
            } else {
                area.ty = Type::Sub;
                if f1 > f2 {
                    area.solution = f1 - f2;
                } else {
                    area.solution = f2 - f1;
                }
            }
        } else {
            if rand::random() {
                area.ty = Type::Mul;
                area.solution = area
                    .fields
                    .iter()
                    .map(|f| sol.get(f).unwrap())
                    .fold(1, |a, b| a * b);
            } else {
                area.ty = Type::Add;
                area.solution = area.fields.iter().map(|f| sol.get(f).unwrap()).sum();
            }
        }
    }

    crate::print::print(&kenken, vec![sol], 10).unwrap();
    kenken
}
