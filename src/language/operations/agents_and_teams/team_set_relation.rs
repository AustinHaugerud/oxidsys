use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TeamSetRelationOp;

const DOC: &str =
    "Sets relations between two teams. Possible values: enemy (-1), neutral (0) and friendly (1).";

pub const OP_CODE: u32 = 1796;

pub const IDENT: &str = "team_set_relation";

impl Operation for TeamSetRelationOp {
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
                make_param_doc("<team_no>", ""),
                make_param_doc("<team_no_2>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
