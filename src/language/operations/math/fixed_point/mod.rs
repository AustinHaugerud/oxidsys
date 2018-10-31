use language::operations::{make_param_doc, Operation, ParamInfo};
pub mod convert_from_fixed_point;
pub mod convert_to_fixed_point;
pub mod set_fixed_point_multiplier;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(
        convert_from_fixed_point::ConvertFromFixedPointOp {},
    ));
    result.push(Box::new(convert_to_fixed_point::ConvertToFixedPointOp {}));
    result.push(Box::new(
        set_fixed_point_multiplier::SetFixedPointMultiplierOp {},
    ));
    result
}
