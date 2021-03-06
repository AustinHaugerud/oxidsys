use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerSetFaceKeysOp;

const DOC: &str = "Version 1.161+. Sets player's face keys from string.";

pub const OP_CODE: u32 = 2748;

pub const IDENT: &str = "player_set_face_keys";

impl Operation for PlayerSetFaceKeysOp {
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
                make_param_doc("<player_id>", ""),
                make_param_doc("<string_no>", ""),
            ],
        }
    }
}
