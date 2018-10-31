use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MultiplayerClearSceneOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 416;

pub const IDENT: &str = "multiplayer_clear_scene";

impl Operation for MultiplayerClearSceneOp {
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
