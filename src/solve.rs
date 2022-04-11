use good_lp::{variable, variables, Expression, Constraint, Variable};

use crate::{asg::Assignment, KenKen};



pub fn solve(kenken: &KenKen) -> Vec<Assignment> {

    let mut asgs = Vec::new();
    for area in &kenken.areas {
        asgs.append(&mut area.possible_assignments());
    }

    println!("Total number of assignments: {}", asgs.len());

    let indexed_asgs: Vec<(usize, Assignment)> = asgs.into_iter().enumerate().collect();

    
    let mut objective: Expression = 0.into();
    let mut constraints = Vec::<Constraint>::new();
    let mut vars = variables!();
    let mut indexed_vars = Vec::<Variable>::new();

    for (i, asg) in &indexed_asgs {
        let var  = vars.add(variable().binary().name(format!("x_{}", i)));
        indexed_vars.push(var);
    }

    for (i, asgi) in &indexed_asgs {
        for (j, asgj) in &indexed_asgs {
            if asgi.conflict_asg(asgj) {
                let vi = indexed_vars[*i];
                let vj = indexed_vars[*j];
                constraints.push((vi + vj).leq(1 as i32));
            }
        }
    }

    vec![]
}

