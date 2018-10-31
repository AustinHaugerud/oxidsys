use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TeamSetOrderPositionOp;

const DOC: &str =
    "Defines the position for specified team/division when currently issued order requires one.";

pub const OP_CODE: u32 = 1791;

pub const IDENT: &str = "team_set_order_position";

impl Operation for TeamSetOrderPositionOp {
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
                make_param_doc("<division>", ""),
                make_param_doc("<position>", ""),
            ],
        }
    }
}
