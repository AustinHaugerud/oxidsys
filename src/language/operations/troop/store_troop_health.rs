use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreTroopHealthOp;

const DOC : &str = "Retrieves current troop health. Use absolute = 1 to retrieve actual number of hp points left, use absolute = 0 to retrieve a value in 0..100 range (percentage).";

pub const OP_CODE: u32 = 2175;

pub const IDENT: &str = "store_troop_health";

impl Operation for StoreTroopHealthOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<troop_id>", ""),
                make_param_doc("[absolute]", ""),
            ],
        }
    }
}
