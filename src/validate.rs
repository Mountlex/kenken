use crate::kenken::{Area, KenKen};

pub trait Validator {
    fn validate(&self) -> ValidationResult;
}

impl Validator for KenKen {
    fn validate(&self) -> ValidationResult {
        Ok(())
    }
}

impl Area {
    fn validate(&self) -> ValidationResult {
        Ok(())
    }
}

type ValidationResult = Result<(), Vec<ValidationError>>;

#[derive(Debug, Clone)]
pub enum ValidationError {}
