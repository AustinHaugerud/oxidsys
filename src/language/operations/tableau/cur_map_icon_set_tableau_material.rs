use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurMapIconSetTableauMaterialOp;

const DOC : &str = "Can only be used inside ti_on_init_map_icon trigger in module_map_icons.py. Assigns tableau to the icon prop instance. Value of <instance_code> will be passed to tableau code. Commonly used for player/lord party banners.";

pub const OP_CODE: u32 = 1983;

pub const IDENT: &str = "cur_map_icon_set_tableau_material";

impl Operation for CurMapIconSetTableauMaterialOp {
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
