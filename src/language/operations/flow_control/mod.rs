pub mod call_script;
pub mod else_try;
pub mod try_begin;
pub mod try_end;
pub mod try_for_agents;
pub mod try_for_parties;
pub mod try_for_players;
pub mod try_for_prop_instances;
pub mod try_for_range;
pub mod try_for_range_backwards;

use language::operations::Operation;

pub fn load_operations() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];

    result.push(Box::new(call_script::CallScriptOp {}));
    result.push(Box::new(else_try::ElseTryOp {}));
    result.push(Box::new(try_begin::TryBeginOp {}));
    result.push(Box::new(try_end::TryEndOp {}));
    result.push(Box::new(try_for_agents::TryForAgentsOp {}));
    result.push(Box::new(try_for_parties::TryForPartiesOp {}));
    result.push(Box::new(try_for_players::TryForPlayersOp {}));
    result.push(Box::new(try_for_prop_instances::TryForPropInstancesOp {}));
    result.push(Box::new(try_for_range::TryForRangeOp {}));
    result.push(Box::new(try_for_range_backwards::TryForRangeBackwardsOp {}));

    result
}
