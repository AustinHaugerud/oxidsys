use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FaceKeysGetHairOp;

const DOC : &str = "Version 1.161+. Unpacks selected hair mesh from string containing troop/player face keys to <destination>.";

pub const OP_CODE: u32 = 2752;

pub const IDENT: &str = "face_keys_get_hair";

impl Operation for FaceKeysGetHairOp {
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
