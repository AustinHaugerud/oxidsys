use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetupQuestTextOp;

const DOC : &str = "Operation will refresh default quest description (as defined in module_quests.py). This is important when quest description contains references to variables and registers which need to be initialized with their current values.";

pub const OP_CODE: u32 = 1290;

pub const IDENT: &str = "setup_quest_text";

impl Operation for SetupQuestTextOp {
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
            param_docs: vec![make_param_doc("<quest_id>", "")],
        }
    }
}
