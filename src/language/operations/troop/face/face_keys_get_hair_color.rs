use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FaceKeysGetHairColorOp;

const DOC : &str = "Version 1.161+. Unpacks hair color slider value from face keys string. Values are in the range of 0..63. Mapping to specific colors depends on the hair color range defined for currently selected skin / face_texture combination.";

pub const OP_CODE: u32 = 2760;

pub const IDENT: &str = "face_keys_get_hair_color";

impl Operation for FaceKeysGetHairColorOp {
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
