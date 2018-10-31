use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenLootOp;

const DOC : &str = "Opens the Looting interface, using the provided troop as loot storage. Player has full access to troop inventory.";

pub const OP_CODE: u32 = 2041;

pub const IDENT: &str = "change_screen_loot";

impl Operation for ChangeScreenLootOp {
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
            param_docs: vec![make_param_doc("<troop_id>", "")],
        }
    }
}
