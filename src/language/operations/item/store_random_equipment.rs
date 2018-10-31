use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreRandomEquipmentOp;

const DOC: &str = "Deprecated since early M&B days.";

pub const OP_CODE: u32 = 2258;

pub const IDENT: &str = "store_random_equipment";

impl Operation for StoreRandomEquipmentOp {
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
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
