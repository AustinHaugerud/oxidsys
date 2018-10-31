use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSetFactionOp;

const DOC: &str =
    "Sets a new faction for the troop (mostly used to switch lords allegiances in Native).";

pub const OP_CODE: u32 = 1550;

pub const IDENT: &str = "troop_set_faction";

impl Operation for TroopSetFactionOp {
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
                make_param_doc("<troop_id>", ""),
                make_param_doc("<faction_id>", ""),
            ],
        }
    }
}
