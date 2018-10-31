use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionTplEntryClearOverrideItemsOp;

const DOC : &str = "Clears the list of override equipment provided by the entry point definition in module_mission_templates.py.";

pub const OP_CODE: u32 = 1941;

pub const IDENT: &str = "mission_tpl_entry_clear_override_items";

impl Operation for MissionTplEntryClearOverrideItemsOp {
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
                make_param_doc("<mission_template_id>", ""),
                make_param_doc("<entry_no>", ""),
            ],
        }
    }
}
