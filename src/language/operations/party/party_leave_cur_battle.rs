use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyLeaveCurBattleOp;

const DOC: &str = "Forces the party to leave it's current battle (if it's engaged).";

pub const OP_CODE: u32 = 1666;

pub const IDENT: &str = "party_leave_cur_battle";

impl Operation for PartyLeaveCurBattleOp {
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
            param_docs: vec![make_param_doc("<party_id>", "")],
        }
    }
}
