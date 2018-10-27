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
    unimplemented!()
}
