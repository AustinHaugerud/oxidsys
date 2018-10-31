use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreRemainingTeamNoOp;

const DOC : &str = "Retrieves the number of the last remaining team. Currently not used in Native, possibly deprecated.";

pub const OP_CODE: u32 = 2360;

pub const IDENT: &str = "store_remaining_team_no";

impl Operation for StoreRemainingTeamNoOp {
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
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
