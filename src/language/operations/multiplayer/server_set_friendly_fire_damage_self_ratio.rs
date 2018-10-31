use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ServerSetFriendlyFireDamageSelfRatioOp;

const DOC: &str = "Official docs: 0-100";

pub const OP_CODE: u32 = 496;

pub const IDENT: &str = "server_set_friendly_fire_damage_self_ratio";

impl Operation for ServerSetFriendlyFireDamageSelfRatioOp {
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
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}
