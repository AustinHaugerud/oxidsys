use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ShuffleRangeOp;

const DOC : &str = "Randomly shuffles a range of registers, reordering the values contained in them. Commonly used for list randomization.";

pub const OP_CODE: u32 = 2134;

pub const IDENT: &str = "shuffle_range";

impl Operation for ShuffleRangeOp {
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
                make_param_doc("<reg_no>", ""),
                make_param_doc("<reg_no>", ""),
            ],
        }
    }
}
