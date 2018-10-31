use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct EndCurrentBattleOp;

const DOC : &str = "Apparently ends the battle between player's party and it's opponent. Exact effects not clear. 4research.";

pub const OP_CODE: u32 = 1307;

pub const IDENT: &str = "end_current_battle";

impl Operation for EndCurrentBattleOp {
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
