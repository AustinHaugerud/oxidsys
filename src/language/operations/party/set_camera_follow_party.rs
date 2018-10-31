use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetCameraFollowPartyOp;

const DOC : &str = "Self-explanatory. Can be used on world map only. Commonly used to make camera follow a party which has captured player as prisoner.";

pub const OP_CODE: u32 = 1021;

pub const IDENT: &str = "set_camera_follow_party";

impl Operation for SetCameraFollowPartyOp {
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
            param_docs: vec![make_param_doc("<party_id>", "")],
        }
    }
}
