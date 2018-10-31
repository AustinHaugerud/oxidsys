use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct JumpToSceneOp;

const DOC : &str = "Tells the game to use the specified scene for the next mission. Usually followed by (change_screen_mission) call. Parameter entry_no does not seem to have any effect.";

pub const OP_CODE: u32 = 1910;

pub const IDENT: &str = "jump_to_scene";

impl Operation for JumpToSceneOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<scene_id>", ""),
                make_param_doc("[entry_no]", ""),
            ],
        }
    }
}
