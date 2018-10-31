use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStorePlayerFaceKeysOp;

const DOC: &str = "Version 1.161+. Stores player's face keys into string register.";

pub const OP_CODE: u32 = 2747;

pub const IDENT: &str = "str_store_player_face_keys";

impl Operation for StrStorePlayerFaceKeysOp {
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
                make_param_doc("<string_no>", ""),
                make_param_doc("<player_id>", ""),
            ],
        }
    }
}
