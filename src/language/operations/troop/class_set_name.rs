use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ClassSetNameOp;

const DOC: &str =
    "Sets a new name for troop class (aka \"Infantry\", \"Cavalry\", \"Custom Group 3\"...).";

pub const OP_CODE: u32 = 1837;

pub const IDENT: &str = "class_set_name";

impl Operation for ClassSetNameOp {
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
                make_param_doc("<sub_class>", ""),
                make_param_doc("<string_id>", ""),
            ],
        }
    }
}
