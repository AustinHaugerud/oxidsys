use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TeamGetFactionOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 458;

pub const IDENT: &str = "team_get_faction";

impl Operation for TeamGetFactionOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<team_id>", ""),
            ],
        }
    }
}
