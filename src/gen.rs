use std::vec;

use rand::{prelude::{IteratorRandom, Distribution}, Rng, thread_rng};

use crate::{
    asg::Assignment,
    kenken::{Area, Field, KenKen, Type},
};

pub struct DifficultyConfig {
    pub p_add: f32,
    pub p_mul: f32,
    pub p_div: f32,
    pub p_sub: f32,
    pub size_factor: f32,
}

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

fn random_area_gen(size: u16, config: &DifficultyConfig) -> KenKen {
    let mut kenken = KenKen {
        id: 0,
        areas: vec![],
        size,
    };

    let max_area = (size as f32 * config.size_factor).ceil() as u16; 

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

fn assign_area(area: &mut Area, typ: Type, f1: u64, f2: u64) {
    area.ty = typ;
    match typ {
        Type::Div => {
            if f1 > f2 {
                area.solution = f1 / f2;
            } else {
                area.solution = f2 / f1;
            }
        },
        Type::Sub => {
            if f1 > f2 {
                area.solution = f1 - f2;
            } else {
                area.solution = f2 - f1;
            }
        },
        Type::Add => {
            area.solution = f1 + f2;
        },
        Type::Mul => {
            area.solution = f1 * f2;
        },
        Type::Single => {}
    }
}

pub fn generate(size: u16, config: &DifficultyConfig) -> KenKen {

    assert!(config.p_add + config.p_div + config.p_mul + config.p_sub == 1.0);

    let mut kenken = random_area_gen(size, config);
    let sol = random_solution(size);
    let mut rng = thread_rng();

    for mut area in &mut kenken.areas {
        if area.fields.len() == 1 {
            area.solution = sol.get(&area.fields[0]).unwrap() as u64;
        } else if area.fields.len() == 2 {
            let f1 = sol.get(&area.fields[0]).unwrap() as u64;
            let f2 = sol.get(&area.fields[1]).unwrap() as u64;
            assert_ne!(f1,f2);
            let divisible = (f1 > f2 && f1 % f2 == 0) || (f2 > f1 && f2 % f1 == 0);

            if divisible {
                let choices = [Type::Add, Type::Sub, Type::Mul, Type::Div];
                let dist = rand::distributions::WeightedIndex::new(&[config.p_add, config.p_sub, config.p_mul, config.p_div]).unwrap();
                assign_area(&mut area, choices[dist.sample(&mut rng)], f1, f2);
            } else {
                let choices = [Type::Add, Type::Sub, Type::Mul];
                let dist = rand::distributions::WeightedIndex::new(&[config.p_add, config.p_sub, config.p_mul]).unwrap();
                assign_area(&mut area, choices[dist.sample(&mut rng)], f1, f2);
            }
        } else {
            let choices = [Type::Add, Type::Mul];
            let dist = rand::distributions::WeightedIndex::new(&[config.p_add, config.p_mul]).unwrap();
            if choices[dist.sample(&mut rng)] == Type::Mul {
                area.ty = Type::Mul;
                area.solution = area
                    .fields
                    .iter()
                    .map(|f| sol.get(f).unwrap())
                    .fold(1, |a, b| a * (b as u64));
            } else {
                area.ty = Type::Add;
                area.solution = area.fields.iter().map(|f| sol.get(f).unwrap() as u64).sum();
            }
        }
    }

    kenken
}
