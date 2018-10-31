use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FaceKeysSetMorphKeyOp;

const DOC : &str = "Version 1.161+. Updates face keys string with a new morph key value. See morph key indices in module_skins.py file. Note that only 8 out of 27 morph keys are actually accessible (from 'chin_size' to 'cheeks'). Morph key values should be in the 0..7 range.";

pub const OP_CODE: u32 = 2767;

pub const IDENT: &str = "face_keys_set_morph_key";

impl Operation for FaceKeysSetMorphKeyOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<string_no>", ""),
                make_param_doc("<key_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
