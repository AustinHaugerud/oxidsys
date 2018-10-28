use language::operations::Operation;
pub mod store_repeat_object;
pub mod talk_info_set_line;
pub mod talk_info_set_relation_bar;
pub mod talk_info_show;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(store_repeat_object::StoreRepeatObjectOp {}));
    result.push(Box::new(talk_info_set_line::TalkInfoSetLineOp {}));
    result.push(Box::new(
        talk_info_set_relation_bar::TalkInfoSetRelationBarOp {},
    ));
    result.push(Box::new(talk_info_show::TalkInfoShowOp {}));
    result
}
