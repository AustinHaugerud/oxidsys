use language::operations::{make_param_doc, Operation, ParamInfo};
pub mod set_result_string;
pub mod store_script_param;
pub mod store_script_param_1;
pub mod store_script_param_2;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(set_result_string::SetResultStringOp {}));
    result.push(Box::new(store_script_param::StoreScriptParamOp {}));
    result.push(Box::new(store_script_param_1::StoreScriptParam1Op {}));
    result.push(Box::new(store_script_param_2::StoreScriptParam2Op {}));
    result
}
