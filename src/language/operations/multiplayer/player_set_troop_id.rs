use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerSetTroopIdOp;

const DOC: &str = "Assigns the selected troop reference to a player.";

pub const OP_CODE: u32 = 405;

pub const IDENT: &str = "player_set_troop_id";

impl Operation for PlayerSetTroopIdOp {
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
                make_param_doc("<player_id>", ""),
                make_param_doc("<troop_id>", ""),
            ],
        }
    }
}
