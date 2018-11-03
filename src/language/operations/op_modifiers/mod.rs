use language::operations::{Operation};
pub mod neg;
pub mod this_or_next;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(neg::NegOp {}));
    result.push(Box::new(this_or_next::ThisOrNextOp {}));
    result
}
