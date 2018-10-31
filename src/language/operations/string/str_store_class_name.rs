use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStoreClassNameOp;

const DOC : &str = "Stores name of the selected troop class (Infantry, Archers, Cavalry or any of the custom class names) in referenced string register.";

pub const OP_CODE: u32 = 2346;

pub const IDENT: &str = "str_store_class_name";

impl Operation for StrStoreClassNameOp {
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
                make_param_doc("<stribg_register>", ""),
                make_param_doc("<class_id>", ""),
            ],
        }
    }
}
