use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FaceKeysGetSkinColorOp;

const DOC : &str = "Version 1.161+. Apparently doesn't work. Should retrieve skin color value from face keys string into <destination>.";

pub const OP_CODE: u32 = 2764;

pub const IDENT: &str = "face_keys_get_skin_color";

impl Operation for FaceKeysGetSkinColorOp {
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
                make_param_doc("<string_no>", ""),
            ],
        }
    }
}
