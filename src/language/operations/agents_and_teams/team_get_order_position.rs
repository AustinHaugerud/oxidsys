use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TeamGetOrderPositionOp;

const DOC: &str = "Retrieves position which is used for specified team/division current orders.";

pub const OP_CODE: u32 = 1794;

pub const IDENT: &str = "team_get_order_position";

impl Operation for TeamGetOrderPositionOp {
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
                make_param_doc("<position>", ""),
                make_param_doc("<team_no>", ""),
                make_param_doc("<division>", ""),
            ],
        }
    }
}
