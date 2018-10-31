use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ClassIsListeningOrderOp;

const DOC: &str =
    "Checks that the specified group of specified team is listening to player's orders.";

pub const OP_CODE: u32 = 1775;

pub const IDENT: &str = "class_is_listening_order";

impl Operation for ClassIsListeningOrderOp {
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
                make_param_doc("<team_no>", ""),
                make_param_doc("<sub_class>", ""),
            ],
        }
    }
}
