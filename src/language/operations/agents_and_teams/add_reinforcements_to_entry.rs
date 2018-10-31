use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AddReinforcementsToEntryOp;

const DOC : &str = "For battle missions, adds reinforcement wave to the specified entry point. Additional parameter determines relative wave size. Agents in reinforcement wave are taken from all parties of the side that the entry point belongs to due to mtef_team_* flags.";

pub const OP_CODE: u32 = 1930;

pub const IDENT: &str = "add_reinforcements_to_entry";

impl Operation for AddReinforcementsToEntryOp {
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
                make_param_doc("<mission_template_entry_no>", ""),
                make_param_doc("<wave_size>", ""),
            ],
        }
    }
}
