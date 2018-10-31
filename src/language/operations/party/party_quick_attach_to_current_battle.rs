use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyQuickAttachToCurrentBattleOp;

const DOC: &str = "Adds any party into current encounter at specified side (0 = ally, 1 = enemy).";

pub const OP_CODE: u32 = 1663;

pub const IDENT: &str = "party_quick_attach_to_current_battle";

impl Operation for PartyQuickAttachToCurrentBattleOp {
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
                make_param_doc("<side>", ""),
            ],
        }
    }
}
