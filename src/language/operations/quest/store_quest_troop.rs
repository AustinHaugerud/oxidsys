use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreQuestTroopOp;

const DOC: &str =
    "Apparently deprecated. Native now uses quest slots to keep track of this information.";

pub const OP_CODE: u32 = 2263;

pub const IDENT: &str = "store_quest_troop";

impl Operation for StoreQuestTroopOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<troop_id>", ""),
            ],
        }
    }
}
