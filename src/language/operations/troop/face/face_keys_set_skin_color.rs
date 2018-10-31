use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FaceKeysSetSkinColorOp;

const DOC : &str = "Version 1.161+. Apparently doesn't work. Should update face keys string with a new skin color value.";

pub const OP_CODE: u32 = 2765;

pub const IDENT: &str = "face_keys_set_skin_color";

impl Operation for FaceKeysSetSkinColorOp {
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
                make_param_doc("<value>", ""),
            ],
        }
    }
}
