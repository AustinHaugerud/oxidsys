use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetQuestProgressionOp;

const DOC : &str = "Deprecated and useless, operation has no game effects and it's impossible to retrieve quest progression status anyway.";

pub const OP_CODE: u32 = 1285;

pub const IDENT: &str = "set_quest_progression";

impl Operation for SetQuestProgressionOp {
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
                make_param_doc("<quest_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
