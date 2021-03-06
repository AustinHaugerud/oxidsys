use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OptionsGetDamageToFriendsOp;

const DOC: &str = "0 = 1/2, 1 = 3/4, 2 = 1/1";

pub const OP_CODE: u32 = 262;

pub const IDENT: &str = "options_get_damage_to_friends";

impl Operation for OptionsGetDamageToFriendsOp {
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
