use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreAttributeLevelOp;

const DOC : &str = "Stores current value of troop attribute. See ca_* constants in header_troops.py for reference.";

pub const OP_CODE: u32 = 2172;

pub const IDENT: &str = "store_attribute_level";

impl Operation for StoreAttributeLevelOp {
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
                make_param_doc("<troop_id>", ""),
                make_param_doc("<attribute_id>", ""),
            ],
        }
    }
}
