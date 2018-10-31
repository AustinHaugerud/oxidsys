use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SucceedQuestOp;

const DOC : &str = "Sets quest status as successful but keeps it in the list (player must visit quest giver to complete it before he can get another quest of the same type).";

pub const OP_CODE: u32 = 1282;

pub const IDENT: &str = "succeed_quest";

impl Operation for SucceedQuestOp {
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
