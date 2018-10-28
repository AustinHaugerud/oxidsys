use language::operations::Operation;
pub mod reset_item_probabilities;
pub mod reset_price_rates;
pub mod set_item_probability_in_merchandise;
pub mod set_merchandise_max_value;
pub mod set_merchandise_modifier_quality;
pub mod set_price_rate_for_item;
pub mod set_price_rate_for_item_type;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(
        reset_item_probabilities::ResetItemProbabilitiesOp {},
    ));
    result.push(Box::new(reset_price_rates::ResetPriceRatesOp {}));
    result.push(Box::new(
        set_item_probability_in_merchandise::SetItemProbabilityInMerchandiseOp {},
    ));
    result.push(Box::new(
        set_merchandise_max_value::SetMerchandiseMaxValueOp {},
    ));
    result.push(Box::new(
        set_merchandise_modifier_quality::SetMerchandiseModifierQualityOp {},
    ));
    result.push(Box::new(set_price_rate_for_item::SetPriceRateForItemOp {}));
    result.push(Box::new(
        set_price_rate_for_item_type::SetPriceRateForItemTypeOp {},
    ));
    result
}
