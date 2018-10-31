use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FaceKeysSetHairOp;

const DOC : &str = "Version 1.161+. Updates face keys string with a new hair value. Hair meshes associated with skin (as defined in module_skins) are numbered from 1. Use 0 for no hair.";

pub const OP_CODE: u32 = 2753;

pub const IDENT: &str = "face_keys_set_hair";

impl Operation for FaceKeysSetHairOp {
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
