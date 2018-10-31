use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TeamGetGapDistanceOp;

const DOC : &str = "Version 1.153+. UNTESTED. Supposedly returns average gap between troops of a specified team/class (depends on how many Stand Closer/Spread Out orders were given).";

pub const OP_CODE: u32 = 1828;

pub const IDENT: &str = "team_get_gap_distance";

impl Operation for TeamGetGapDistanceOp {
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
                make_param_doc("<sub_class>", ""),
            ],
        }
    }
}
