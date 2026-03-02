# OrderReturnLineItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | A unique ID for this return line-item entry. | [optional]
**source_line_item_uid** | Option<**String**> | The `uid` of the line item in the original sale order. | [optional]
**name** | Option<**String**> | The name of the line item. | [optional]
**quantity** | **String** | The quantity returned, formatted as a decimal number. For example, `\"3\"`.  Line items with a `quantity_unit` can have non-integer quantities. For example, `\"1.70000\"`. | 
**quantity_unit** | Option<[**models::OrderQuantityUnit**](OrderQuantityUnit.md)> |  | [optional]
**note** | Option<**String**> | The note of the return line item. | [optional]
**catalog_object_id** | Option<**String**> | The [CatalogItemVariation](entity:CatalogItemVariation) ID applied to this return line item. | [optional]
**catalog_version** | Option<**i64**> | The version of the catalog object that this line item references. | [optional]
**variation_name** | Option<**String**> | The name of the variation applied to this return line item. | [optional]
**item_type** | Option<[**models::OrderLineItemItemType**](OrderLineItemItemType.md)> |  | [optional]
**return_modifiers** | Option<[**Vec<models::OrderReturnLineItemModifier>**](OrderReturnLineItemModifier.md)> | The [CatalogModifier](entity:CatalogModifier)s applied to this line item. | [optional]
**applied_taxes** | Option<[**Vec<models::OrderLineItemAppliedTax>**](OrderLineItemAppliedTax.md)> | The list of references to `OrderReturnTax` entities applied to the return line item. Each `OrderLineItemAppliedTax` has a `tax_uid` that references the `uid` of a top-level `OrderReturnTax` applied to the return line item. On reads, the applied amount is populated. | [optional]
**applied_discounts** | Option<[**Vec<models::OrderLineItemAppliedDiscount>**](OrderLineItemAppliedDiscount.md)> | The list of references to `OrderReturnDiscount` entities applied to the return line item. Each `OrderLineItemAppliedDiscount` has a `discount_uid` that references the `uid` of a top-level `OrderReturnDiscount` applied to the return line item. On reads, the applied amount is populated. | [optional]
**base_price_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**variation_total_price_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**gross_return_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**total_tax_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**total_discount_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**total_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**applied_service_charges** | Option<[**Vec<models::OrderLineItemAppliedServiceCharge>**](OrderLineItemAppliedServiceCharge.md)> | The list of references to `OrderReturnServiceCharge` entities applied to the return line item. Each `OrderLineItemAppliedServiceCharge` has a `service_charge_uid` that references the `uid` of a top-level `OrderReturnServiceCharge` applied to the return line item. On reads, the applied amount is populated. | [optional]
**total_service_charge_money** | Option<[**models::Money**](Money.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


