use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSetHealthOp;

const DOC: &str = "Sets troop health. Accepts value in range 0..100 (percentage).";

pub const OP_CODE: u32 = 1560;

pub const IDENT: &str = "troop_set_health";

impl Operation for TroopSetHealthOp {
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
            param_docs: vec![make_param_doc("<troop_id>", "")],
        }
    }
}
