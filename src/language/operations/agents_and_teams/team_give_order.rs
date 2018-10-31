use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TeamGiveOrderOp;

const DOC: &str = "Issues an order to specified team/division.";

pub const OP_CODE: u32 = 1790;

pub const IDENT: &str = "team_give_order";

impl Operation for TeamGiveOrderOp {
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
                make_param_doc("<order_id>", ""),
            ],
        }
    }
}
