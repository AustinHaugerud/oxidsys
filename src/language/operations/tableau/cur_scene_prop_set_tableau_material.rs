use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurScenePropSetTableauMaterialOp;

const DOC : &str = "Can only be used inside ti_on_init_scene_prop trigger in module_scene_props.py. Assigns tableau to the scene prop instance. Value of <instance_code> will be passed to tableau code. Commonly used for static banners.";

pub const OP_CODE: u32 = 1982;

pub const IDENT: &str = "cur_scene_prop_set_tableau_material";

impl Operation for CurScenePropSetTableauMaterialOp {
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
                make_param_doc("<tableau_material_id>", ""),
                make_param_doc("<instance_code>", ""),
            ],
        }
    }
}
