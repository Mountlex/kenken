use std::collections::HashMap;

use crate::{Area, Field, Type};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assignment {
    values: HashMap<Field, u16>,
}

impl Assignment {
    fn empty() -> Self {
        Self {
            values: HashMap::default(),
        }
    }

    fn set(&mut self, field: Field, value: u16) {
        self.values.insert(field, value);
    }

    pub fn get(&self, field: &Field) -> Option<u16> {
        self.values.get(field).cloned()
    }

    pub fn conflict(&self, field: &Field, value: u16) -> bool {
        self.values
            .iter()
            .any(|(f, &v)| v == value && f.equal_axis(field))
    }

    pub fn conflict_asg(&self, other: &Assignment) -> bool {
        self.values.iter().any(|(f,v)| other.conflict(f, *v))
    }
}

pub fn merge(asgs: Vec<Assignment>) -> Assignment {
    let mut asg = HashMap::new();
    for a in asgs {
        for (f, v) in a.values {
            asg.insert(f, v);
        }
    }
    Assignment { values: asg }
}

const numbers: [u16; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

impl Area {
    pub fn possible_assignments(&self) -> Vec<Assignment> {
        match self.ty {
            Type::Add => {
                let mut asgs = Vec::<Assignment>::new();
                inc_assignments(
                    &mut asgs,
                    &IncType::Add,
                    self.fields.clone(),
                    None,
                    0,
                    self.solution,
                );
                asgs
            }
            Type::Mul => {
                let mut asgs = Vec::<Assignment>::new();
                inc_assignments(
                    &mut asgs,
                    &IncType::Mul,
                    self.fields.clone(),
                    None,
                    1,
                    self.solution,
                );
                asgs
            }
            Type::Sub => {
                dec_assignments(DecType::Sub, self.fields[0], self.fields[1], self.solution)
            }
            Type::Div => {
                dec_assignments(DecType::Div, self.fields[0], self.fields[1], self.solution)
            }
            Type::Single => vec![Assignment {
                values: [(self.fields.first().unwrap().clone(), self.solution)].into(),
            }],
        }
    }
}

enum IncType {
    Add,
    Mul,
}

fn inc_assignments(
    asgs: &mut Vec<Assignment>,
    ty: &IncType,
    rem_fields: Vec<Field>,
    partial_asg: Option<Assignment>,
    partial_sol: u16,
    target_sol: u16,
) {
    if rem_fields.is_empty() {
        if partial_sol == target_sol {
            asgs.push(partial_asg.unwrap())
        }
    } else {
        //for field in &rem_fields {
        let field = rem_fields.first().unwrap();
        for v in numbers {
            let mut asg = if let Some(asg) = &partial_asg {
                asg.clone()
            } else {
                Assignment::empty()
            };

            if !asg.conflict(field, v) {
                asg.set(*field, v);

                let new_partial_sol = match ty {
                    IncType::Add => partial_sol + v,
                    IncType::Mul => partial_sol * v,
                };

                inc_assignments(
                    asgs,
                    ty,
                    rem_fields.iter().filter(|&f| f != field).cloned().collect(),
                    Some(asg),
                    new_partial_sol,
                    target_sol,
                );
            }
        }
        //}
    }
}

enum DecType {
    Sub,
    Div,
}

fn dec_assignments(ty: DecType, field1: Field, field2: Field, solution: u16) -> Vec<Assignment> {
    let mut asgs = Vec::<Assignment>::new();
    for i in numbers {
        for j in numbers {
            if i < j {
                let sol = match ty {
                    DecType::Div => {
                        let tmp = j / i;
                        if tmp * i == j {
                            tmp
                        } else {
                            0
                        }
                    }
                    DecType::Sub => j - i,
                };
                if sol == solution {
                    let mut asg1 = Assignment::empty();
                    asg1.set(field1, i);
                    asg1.set(field2, j);
                    let mut asg2 = Assignment::empty();
                    asg2.set(field2, i);
                    asg2.set(field1, j);
                    asgs.push(asg1);
                    asgs.push(asg2);
                }
            }
        }
    }
    asgs
}

#[cfg(test)]
mod test_asg_gen {
    use super::*;

    #[test]
    fn test_add_two_fields() {
        let area = Area::new(Type::Add, 10, vec![Field(0, 0), Field(1, 0)]);
        let asgs = area.possible_assignments();
        assert_eq!(asgs.len(), 8)
    }

    #[test]
    fn test_add_three_straight_fields() {
        let area = Area::new(Type::Add, 10, vec![Field(0, 0), Field(1, 0), Field(2, 0)]);
        let asgs = area.possible_assignments();
        assert_eq!(asgs.len(), 24)
    }

    #[test]
    fn test_add_three_fields_corner() {
        let area = Area::new(Type::Add, 10, vec![Field(0, 0), Field(1, 0), Field(0, 1)]);
        let asgs = area.possible_assignments();
        assert_eq!(asgs.len(), 28)
    }

    #[test]
    fn test_add_four_fields_rect() {
        let area = Area::new(
            Type::Add,
            8,
            vec![Field(0, 0), Field(1, 0), Field(0, 1), Field(1, 1)],
        );
        let asgs = area.possible_assignments();
        println!("{:?}", asgs);
        assert_eq!(asgs.len(), 10)
    }

    #[test]
    fn test_sub() {
        let area = Area::new(Type::Sub, 4, vec![Field(0, 0), Field(1, 0)]);
        let asgs = area.possible_assignments();
        println!("{:?}", asgs);
        assert_eq!(asgs.len(), 10)
    }

    #[test]
    fn test_div() {
        let area = Area::new(Type::Div, 2, vec![Field(0, 0), Field(1, 0)]);
        let asgs = area.possible_assignments();
        println!("{:?}", asgs);
        assert_eq!(asgs.len(), 8)
    }
}
