use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceSetPositionOp;

const DOC : &str = "Teleports prop instance to another position. Optional flag dont_send_to_clients can be used on the server to prevent position change from being replicated to client machines (useful when doing some calculations which require to move the prop temporarily to another place).";

pub const OP_CODE: u32 = 1855;

pub const IDENT: &str = "prop_instance_set_position";

impl Operation for PropInstanceSetPositionOp {
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
                make_param_doc("<scene_prop_id>", ""),
                make_param_doc("<position>", ""),
                make_param_doc("[dont_send_to_clients]", ""),
            ],
        }
    }
}
