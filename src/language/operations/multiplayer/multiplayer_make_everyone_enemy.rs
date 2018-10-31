use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MultiplayerMakeEveryoneEnemyOp;

const DOC: &str = "Used in deathmatch mode to make everyone hostile to all other agents.";

pub const OP_CODE: u32 = 420;

pub const IDENT: &str = "multiplayer_make_everyone_enemy";

impl Operation for MultiplayerMakeEveryoneEnemyOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}
