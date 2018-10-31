use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetNextBattleSimulationTimeOp;

const DOC : &str = "Defines the period of time (in hours) after which the battle must be simulated for the specified party for the next time. When a value <= 0 is passed, the combat simulation round is performed immediately.";

pub const OP_CODE: u32 = 1667;

pub const IDENT: &str = "party_set_next_battle_simulation_time";

impl Operation for PartySetNextBattleSimulationTimeOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<party_id>", ""),
                make_param_doc("<next_simulation_time_in_hours>", ""),
            ],
        }
    }
}
