use language::operations::{make_param_doc, Operation, ParamInfo};
pub mod get_global_cloud_amount;
pub mod get_global_haze_amount;
pub mod is_currently_night;
pub mod map_free;
pub mod rest_for_hours;
pub mod rest_for_hours_interactive;
pub mod set_global_cloud_amount;
pub mod set_global_haze_amount;
pub mod store_current_day;
pub mod store_current_hours;
pub mod store_time_of_day;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(get_global_cloud_amount::GetGlobalCloudAmountOp {}));
    result.push(Box::new(get_global_haze_amount::GetGlobalHazeAmountOp {}));
    result.push(Box::new(is_currently_night::IsCurrentlyNightOp {}));
    result.push(Box::new(map_free::MapFreeOp {}));
    result.push(Box::new(rest_for_hours::RestForHoursOp {}));
    result.push(Box::new(
        rest_for_hours_interactive::RestForHoursInteractiveOp {},
    ));
    result.push(Box::new(set_global_cloud_amount::SetGlobalCloudAmountOp {}));
    result.push(Box::new(set_global_haze_amount::SetGlobalHazeAmountOp {}));
    result.push(Box::new(store_current_day::StoreCurrentDayOp {}));
    result.push(Box::new(store_current_hours::StoreCurrentHoursOp {}));
    result.push(Box::new(store_time_of_day::StoreTimeOfDayOp {}));
    result
}
