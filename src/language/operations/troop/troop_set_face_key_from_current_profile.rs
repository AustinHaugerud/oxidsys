use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSetFaceKeyFromCurrentProfileOp;

const DOC: &str =
    "Forces the troop to adopt the face from player's currently selected multiplayer profile.";

pub const OP_CODE: u32 = 1503;

pub const IDENT: &str = "troop_set_face_key_from_current_profile";

impl Operation for TroopSetFaceKeyFromCurrentProfileOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<troop_id>", "")],
        }
    }
}
