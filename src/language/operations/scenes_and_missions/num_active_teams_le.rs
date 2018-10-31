use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct NumActiveTeamsLeOp;

const DOC : &str = "Checks that the number of active teams (i.e. teams with at least one active agent) is less than or equal to given value.";

pub const OP_CODE: u32 = 1005;

pub const IDENT: &str = "num_active_teams_le";

impl Operation for NumActiveTeamsLeOp {
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
