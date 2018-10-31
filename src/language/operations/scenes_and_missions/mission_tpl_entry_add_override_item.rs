use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionTplEntryAddOverrideItemOp;

const DOC: &str = "Specified item will be added to any agent spawning on specified entry point.";

pub const OP_CODE: u32 = 1942;

pub const IDENT: &str = "mission_tpl_entry_add_override_item";

impl Operation for MissionTplEntryAddOverrideItemOp {
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
                make_param_doc("<item_kind_id>", ""),
            ],
        }
    }
}
