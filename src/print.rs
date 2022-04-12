use crate::{asg::Assignment, kenken::{KenKen, Field, Type}};
use anyhow::Result;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn print(kenken: &KenKen, asgs: Vec<Assignment>, col_size: usize) -> Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    print_horizontal_separator(&mut stdout, kenken, 0, col_size)?;
    for i in 0..kenken.size {
        print_row(&mut stdout, kenken, &asgs, i, col_size)?;
        print_horizontal_separator(&mut stdout, kenken, i + 1, col_size)?;
    }

    Ok(())
}

fn print_row<W>(
    w: &mut W,
    kenken: &KenKen,
    asgs: &Vec<Assignment>,
    row: u16,
    col_size: usize,
) -> Result<()>
where
    W: Write + WriteColor,
{
    let size = kenken.size;
    print_vertical_sep(w, kenken, row, 0)?;
    for i in 1..=size {
        if let Some(area) = kenken.is_id_field(Field(i - 1, row)) {
            w.set_color(ColorSpec::new().set_bold(true))?;
            match area.ty {
                Type::Add => write!(w, "{}{:<width$}", "+", area.solution, width = col_size - 1)?,
                Type::Mul => write!(w, "{}{:<width$}", "*", area.solution, width = col_size - 1)?,
                Type::Div => write!(w, "{}{:<width$}", "/", area.solution, width = col_size - 1)?,
                Type::Sub => write!(w, "{}{:<width$}", "-", area.solution, width = col_size - 1)?,
                Type::Single => write!(w, "{:<width$}", area.solution, width = col_size)?,
            }
            w.reset()?;
        } else {
            write!(w, "{:width$}", " ", width = col_size)?;
        }
        print_vertical_sep(w, kenken, row, i)?;
    }
    writeln!(w, "")?;

    print_vertical_sep(w, kenken, row, 0)?;
    for i in 1..=size {
        write!(w, "{:width$}", " ", width = col_size)?;
        print_vertical_sep(w, kenken, row, i)?;
    }
    writeln!(w, "")?;

    print_vertical_sep(w, kenken, row, 0)?;
    for i in 1..=size {
        if let Some(asg) = asgs.iter().find_map(|asg| asg.get(&Field(i - 1, row))) {
            write!(w, "{:^width$}", asg, width = col_size)?;
        } else {
            write!(w, "{:width$}", " ", width = col_size)?;
        }
        print_vertical_sep(w, kenken, row, i)?;
    }
    writeln!(w, "")?;

    print_vertical_sep(w, kenken, row, 0)?;
    for i in 1..=size {
        write!(w, "{:width$}", " ", width = col_size)?;
        print_vertical_sep(w, kenken, row, i)?;
    }
    writeln!(w, "")?;

    Ok(())
}

fn print_vertical_sep<W>(w: &mut W, kenken: &KenKen, row: u16, after_col: u16) -> Result<()>
where
    W: Write + WriteColor,
{
    if after_col == 0 || !kenken.same_area(&Field(after_col - 1, row), &Field(after_col, row)) {
        w.set_color(ColorSpec::new().set_bold(true).set_bg(Some(Color::White)))?;
        write!(w, "|")?;
        w.reset()?;
    } else {
        write!(w, "|")?;
    }
    Ok(())
}

fn print_horizontal_separator<W>(
    w: &mut W,
    kenken: &KenKen,
    after_row: u16,
    col_size: usize,
) -> Result<()>
where
    W: Write + WriteColor,
{
    w.set_color(ColorSpec::new().set_bold(true).set_bg(Some(Color::White)))?;
    write!(w, "+")?;
    w.reset()?;
    for i in 0..kenken.size {
        if after_row == 0 || !kenken.same_area(&Field(i, after_row - 1), &Field(i, after_row)) {
            w.set_color(ColorSpec::new().set_bold(true).set_bg(Some(Color::White)))?;
            for _ in 0..col_size {
                write!(w, "-")?;
            }
            w.reset()?;
        } else {
            for _ in 0..col_size {
                write!(w, "-")?;
            }
        }

        if kenken.same_area(&Field(i, after_row - 1), &Field(i, after_row))
            && kenken.same_area(&Field(i + 1, after_row - 1), &Field(i + 1, after_row))
            && kenken.same_area(&Field(i, after_row - 1), &Field(i + 1, after_row - 1))
            && kenken.same_area(&Field(i, after_row), &Field(i + 1, after_row))
        {
            write!(w, "+")?;
        } else {
            w.set_color(ColorSpec::new().set_bold(true).set_bg(Some(Color::White)))?;
            write!(w, "+")?;
            w.reset()?;
        }
    }
    writeln!(w, "")?;
    Ok(())
}
