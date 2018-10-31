use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetItemCurAmmoOp;

const DOC: &str = "Version 1.153+. Returns remaining ammo for specified agent's item.";

pub const OP_CODE: u32 = 1977;

pub const IDENT: &str = "agent_get_item_cur_ammo";

impl Operation for AgentGetItemCurAmmoOp {
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
                make_param_doc("<agent_id>", ""),
                make_param_doc("<slot_no>", ""),
            ],
        }
    }
}
