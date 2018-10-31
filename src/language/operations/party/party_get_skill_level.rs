use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetSkillLevelOp;

const DOC : &str = "Retrieves skill level for the specified party (usually max among the heroes). Makes a callback to (script_game_get_skill_modifier_for_troop).";

pub const OP_CODE: u32 = 1685;

pub const IDENT: &str = "party_get_skill_level";

impl Operation for PartyGetSkillLevelOp {
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
                make_param_doc("<party_id>", ""),
                make_param_doc("<skill_no>", ""),
            ],
        }
    }
}
