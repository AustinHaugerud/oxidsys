use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TalkInfoSetRelationBarOp;

const DOC : &str = "Sets the relations value for relationship bar in the dialog. Value should be in range -100..100.";

pub const OP_CODE: u32 = 2021;

pub const IDENT: &str = "talk_info_set_relation_bar";

impl Operation for TalkInfoSetRelationBarOp {
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
