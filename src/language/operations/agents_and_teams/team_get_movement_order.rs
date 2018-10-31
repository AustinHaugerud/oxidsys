use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TeamGetMovementOrderOp;

const DOC : &str = "Retrieves current movement orders for specified team/division (see mordr_* constants in header_mission_templates.py for reference).";

pub const OP_CODE: u32 = 1785;

pub const IDENT: &str = "team_get_movement_order";

impl Operation for TeamGetMovementOrderOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<team_no>", ""),
                make_param_doc("<division>", ""),
            ],
        }
    }
}
