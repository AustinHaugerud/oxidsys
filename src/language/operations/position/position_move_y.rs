use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionMoveYOp;

const DOC : &str = "Moves position along Y axis. Movement distance is in cms. Optional parameter determines whether the position is moved along the local (value=0) or global (value=1) Y axis (i.e. whether the position will be moved forward/backwards, or to the global north/south).";

pub const OP_CODE: u32 = 721;

pub const IDENT: &str = "position_move_y";

impl Operation for PositionMoveYOp {
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
                make_param_doc("<position>", ""),
                make_param_doc("<movement>", ""),
                make_param_doc("[value]", ""),
            ],
        }
    }
}
