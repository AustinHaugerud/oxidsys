use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenMapOp;

const DOC: &str =
    "Changes the screen to global map, closing any currently running game menu, dialog or mission.";

pub const OP_CODE: u32 = 2052;

pub const IDENT: &str = "change_screen_map";

impl Operation for ChangeScreenMapOp {
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
