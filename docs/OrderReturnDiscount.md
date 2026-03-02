# OrderReturnDiscount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | A unique ID that identifies the returned discount only within this order. | [optional]
**source_discount_uid** | Option<**String**> | The discount `uid` from the order that contains the original application of this discount. | [optional]
**catalog_object_id** | Option<**String**> | The catalog object ID referencing [CatalogDiscount](entity:CatalogDiscount). | [optional]
**catalog_version** | Option<**i64**> | The version of the catalog object that this discount references. | [optional]
**name** | Option<**String**> | The discount's name. | [optional]
**r#type** | Option<[**models::OrderLineItemDiscountType**](OrderLineItemDiscountType.md)> |  | [optional]
**percentage** | Option<**String**> | The percentage of the tax, as a string representation of a decimal number. A value of `\"7.25\"` corresponds to a percentage of 7.25%.  `percentage` is not set for amount-based discounts. | [optional]
**amount_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**applied_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**scope** | Option<[**models::OrderLineItemDiscountScope**](OrderLineItemDiscountScope.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


