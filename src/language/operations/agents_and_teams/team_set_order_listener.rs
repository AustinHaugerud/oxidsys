use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TeamSetOrderListenerOp;

const DOC : &str = "Set the specified division as the one which will be following orders issued by the player (assuming the player is on the same team). If optional parameter add_to_listeners is greater than 0, then the operation will instead *add* specified division to order listeners. If division number is -1, then list of order listeners is cleared. If division number is 9, then all divisions will listen to player's orders.";

pub const OP_CODE: u32 = 1795;

pub const IDENT: &str = "team_set_order_listener";

impl Operation for TeamSetOrderListenerOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<team_no>", ""),
                make_param_doc("<division>", ""),
                make_param_doc("[add_to_listeners]", ""),
            ],
        }
    }
}
