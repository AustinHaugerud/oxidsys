use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSetFaceKeysOp;

const DOC : &str = "Version 1.161+. Sets troop face keys from string. Use optional <alt> parameter to determine what face keys to update: 0 for first and 1 for second.";

pub const OP_CODE: u32 = 2751;

pub const IDENT: &str = "troop_set_face_keys";

impl Operation for TroopSetFaceKeysOp {
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
                make_param_doc("<troop_no>", ""),
                make_param_doc("<string_no>", ""),
                make_param_doc("[<alt>]", ""),
            ],
        }
    }
}
