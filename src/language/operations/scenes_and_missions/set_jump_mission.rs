use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetJumpMissionOp;

const DOC : &str = "Tells the game to use the specified mission template for the next mission. Apparently should precede the call to (jump_to_scene).";

pub const OP_CODE: u32 = 1911;

pub const IDENT: &str = "set_jump_mission";

impl Operation for SetJumpMissionOp {
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
            param_docs: vec![make_param_doc("<mission_template_id>", "")],
        }
    }
}
