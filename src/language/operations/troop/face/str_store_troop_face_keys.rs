use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStoreTroopFaceKeysOp;

const DOC : &str = "Version 1.161+. Stores specified troop's face keys into string register. Use optional <alt> parameter to determine what facekey set to retrieve: 0 for first and 1 for second.";

pub const OP_CODE: u32 = 2750;

pub const IDENT: &str = "str_store_troop_face_keys";

impl Operation for StrStoreTroopFaceKeysOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<string_no>", ""),
                make_param_doc("<troop_no>", ""),
                make_param_doc("[<alt>]", ""),
            ],
        }
    }
}
