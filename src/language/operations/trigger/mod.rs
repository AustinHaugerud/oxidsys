use language::operations::{Operation};
pub mod get_trigger_object_position;
pub mod set_trigger_result;
pub mod store_trigger_param;
pub mod store_trigger_param_1;
pub mod store_trigger_param_2;
pub mod store_trigger_param_3;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(
        get_trigger_object_position::GetTriggerObjectPositionOp {},
    ));
    result.push(Box::new(set_trigger_result::SetTriggerResultOp {}));
    result.push(Box::new(store_trigger_param::StoreTriggerParamOp {}));
    result.push(Box::new(store_trigger_param_1::StoreTriggerParam1Op {}));
    result.push(Box::new(store_trigger_param_2::StoreTriggerParam2Op {}));
    result.push(Box::new(store_trigger_param_3::StoreTriggerParam3Op {}));
    result
}
