use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionTplEntrySetOverrideFlagsOp;

const DOC : &str = "Allows modder to use a different set of equipment override flags (see af_* constants in header_mission_templates.py) for the selected entry point.";

pub const OP_CODE: u32 = 1940;

pub const IDENT: &str = "mission_tpl_entry_set_override_flags";

impl Operation for MissionTplEntrySetOverrideFlagsOp {
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
                make_param_doc("<mission_template_id>", ""),
                make_param_doc("<entry_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
