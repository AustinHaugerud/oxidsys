use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FaceKeysSetAgeOp;

const DOC : &str = "Version 1.161+. Updates face keys string with a new age slider value. Value should be in the 0..63 range.";

pub const OP_CODE: u32 = 2763;

pub const IDENT: &str = "face_keys_set_age";

impl Operation for FaceKeysSetAgeOp {
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
