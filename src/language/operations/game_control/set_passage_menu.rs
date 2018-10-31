use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetPassageMenuOp;

const DOC : &str = "When setting up a mission, this allows you to determine what game menu will be used for that mission passages instead of \"mnu_town\". Passage menu item number will determine what menu option (in sequential order, starting from 0) will be executed when the player activates that passage on the scene. Note that menu option condition code block will be ignored.";

pub const OP_CODE: u32 = 1304;

pub const IDENT: &str = "set_passage_menu";

impl Operation for SetPassageMenuOp {
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
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}
