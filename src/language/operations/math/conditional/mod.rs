use language::operations::Operation;
pub mod eq;
pub mod ge;
pub mod gt;
pub mod is_between;
pub mod le;
pub mod lt;
pub mod neq;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(eq::EqOp {}));
    result.push(Box::new(ge::GeOp {}));
    result.push(Box::new(gt::GtOp {}));
    result.push(Box::new(is_between::IsBetweenOp {}));
    result.push(Box::new(le::LeOp {}));
    result.push(Box::new(lt::LtOp {}));
    result.push(Box::new(neq::NeqOp {}));
    result
}
