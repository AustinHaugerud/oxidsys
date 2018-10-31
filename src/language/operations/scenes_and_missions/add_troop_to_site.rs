use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AddTroopToSiteOp;

const DOC : &str = "Set troop's position in the world to the specified scene and entry point. Entry point must have mtef_scene_source type. Agent will always appear at that entry when entering that scene. No longer used in Native.";

pub const OP_CODE: u32 = 1250;

pub const IDENT: &str = "add_troop_to_site";

impl Operation for AddTroopToSiteOp {
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
                make_param_doc("<troop_id>", ""),
                make_param_doc("<scene_id>", ""),
                make_param_doc("<entry_no>", ""),
            ],
        }
    }
}
