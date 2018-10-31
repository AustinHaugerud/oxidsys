use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FaceKeysGetAgeOp;

const DOC : &str = "Version 1.161+. Unpacks age slider value from face keys string. Values are in the range of 0..63.";

pub const OP_CODE: u32 = 2762;

pub const IDENT: &str = "face_keys_get_age";

impl Operation for FaceKeysGetAgeOp {
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
