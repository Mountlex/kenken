use good_lp::{
    default_solver, variable, variables, Constraint, Expression, Solution, SolverModel, Variable,
};

use crate::{asg::Assignment, KenKen};

pub fn solve(kenken: &KenKen) -> Vec<Assignment> {
    let mut asgs = Vec::new();
    let objective: Expression = 0.into();
    let mut constraints = Vec::<Constraint>::new();
    let mut vars = variables!();
    let mut indexed_vars = Vec::<Variable>::new();

    let mut idx = 0;

    for area in &kenken.areas {
        let area_asgs = area.possible_assignments();
        let mut area_vars = Vec::<Variable>::new();
        for asg in area_asgs {
            let var = vars.add(variable().binary().name(format!("x_{}", idx)));
            area_vars.push(var);
            asgs.push(asg);
            idx += 1;
        }

        constraints.push(area_vars.iter().sum::<Expression>().eq(1i32));
        indexed_vars.append(&mut area_vars)
    }

    println!("Total number of assignments: {}", asgs.len());

    for (i, asgi) in asgs.iter().enumerate() {
        for (j, asgj) in asgs.iter().enumerate() {
            if asgi.conflict_asg(asgj) {
                let vi = indexed_vars[i];
                let vj = indexed_vars[j];
                constraints.push((vi + vj).leq(1i32));
            }
        }
    }

    let mut model = vars.minimise(objective).using(default_solver);
    for constr in constraints {
        model.add_constraint(constr);
    }

    let solution = model.solve().unwrap();

    let sol_asgs = asgs
        .into_iter()
        .enumerate()
        .filter(|(i, _)| solution.eval(indexed_vars[*i]).eq(&1.0))
        .map(|(_, asg)| asg)
        .collect();

    sol_asgs
}
