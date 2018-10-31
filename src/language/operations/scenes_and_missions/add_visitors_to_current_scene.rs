use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AddVisitorsToCurrentSceneOp;

const DOC : &str = "Adds a number of troops to the specified entry point when the scene is already loaded. Team and group parameters are used in multiplayer mode only, singleplayer mode uses team settings for selected entry point as defined in module_mission_templates.py.";

pub const OP_CODE: u32 = 1265;

pub const IDENT: &str = "add_visitors_to_current_scene";

impl Operation for AddVisitorsToCurrentSceneOp {
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
            num_required: 5,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<entry_no>", ""),
                make_param_doc("<troop_id>", ""),
                make_param_doc("<number_of_troops>", ""),
                make_param_doc("<team_no>", ""),
                make_param_doc("<group_no>", ""),
            ],
        }
    }
}
