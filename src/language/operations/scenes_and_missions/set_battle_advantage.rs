use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetBattleAdvantageOp;

const DOC: &str = "Sets a new value for battle advantage.";

pub const OP_CODE: u32 = 1691;

pub const IDENT: &str = "set_battle_advantage";

impl Operation for SetBattleAdvantageOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}
