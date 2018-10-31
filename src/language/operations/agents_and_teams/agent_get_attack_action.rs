use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetAttackActionOp;

const DOC : &str = "Retrieves agent's current attack action. Possible values: free = 0, readying_attack = 1, releasing_attack = 2, completing_attack_after_hit = 3, attack_parried = 4, reloading = 5, after_release = 6, cancelling_attack = 7.";

pub const OP_CODE: u32 = 1763;

pub const IDENT: &str = "agent_get_attack_action";

impl Operation for AgentGetAttackActionOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<agent_id>", ""),
            ],
        }
    }
}
