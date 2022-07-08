use std::{vec, collections::HashMap};

use rand::prelude::{SliceRandom, IteratorRandom};

use crate::{kenken::{Area, Field, KenKen, Type}, asg::Assignment};

pub fn generate(size: u16) -> KenKen {
    let mut kenken = KenKen {
        id: 0,
        areas: vec![],
        size,
    };

    let mut rng = rand::thread_rng();

    for i in 0..size {
        for j in 0..size {
            let mut added = false;
            if j > 0 && rand::random() {
                added = kenken.add_to_area_if_exists(&Field(i, j - 1), Field(i, j))
            }
            if !added && i > 0 && rand::random() {
                added = kenken.add_to_area_if_exists(&Field(i - 1, j), Field(i, j))
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

    let mut sol = Assignment::empty();

    for i in 0..size {
        for j in 0..size {
            sol.set(Field(i,j), ((i + j) % size) + 1 );
        }
    }

    for _ in 0..(size*size) {
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

    for area in &mut kenken.areas {
        if area.fields.len() == 1 {
            area.solution = sol.get(&area.fields[0]).unwrap();
        } else if area.fields.len() == 2 {
            let f1 =  sol.get(&area.fields[0]).unwrap();
            let f2 =  sol.get(&area.fields[1]).unwrap();
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
                area.solution = area.fields.iter().map(|f| sol.get(f).unwrap()).fold(1, |a,b| a * b);
            } else {
                area.ty = Type::Add;
                area.solution = area.fields.iter().map(|f| sol.get(f).unwrap()).sum();
            }
        }
    }

    crate::print::print(&kenken, vec![sol], 10).unwrap();
    kenken
}
