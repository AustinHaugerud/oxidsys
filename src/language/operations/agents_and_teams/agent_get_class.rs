use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetClassOp;

const DOC : &str = "Retrieves the agent class (see grc_* constants in header_mission_templates.py for reference). Note this operation returns the troop class that the game divines from troop equipment and flags, ignoring any custom troop class settings.";

pub const OP_CODE: u32 = 1772;

pub const IDENT: &str = "agent_get_class";

impl Operation for AgentGetClassOp {
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
