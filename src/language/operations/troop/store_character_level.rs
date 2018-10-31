use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreCharacterLevelOp;

const DOC: &str = "Retrieves character level of the troop. Default troop is the player.";

pub const OP_CODE: u32 = 2171;

pub const IDENT: &str = "store_character_level";

impl Operation for StoreCharacterLevelOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("[troop_id]", ""),
            ],
        }
    }
}
