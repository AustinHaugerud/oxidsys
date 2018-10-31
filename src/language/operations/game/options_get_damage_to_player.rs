use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OptionsGetDamageToPlayerOp;

const DOC: &str = "0 = 1/4, 1 = 1/2, 2 = 1/1";

pub const OP_CODE: u32 = 260;

pub const IDENT: &str = "options_get_damage_to_player";

impl Operation for OptionsGetDamageToPlayerOp {
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
