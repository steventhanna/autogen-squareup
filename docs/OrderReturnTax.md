# OrderReturnTax

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | A unique ID that identifies the returned tax only within this order. | [optional]
**source_tax_uid** | Option<**String**> | The tax `uid` from the order that contains the original tax charge. | [optional]
**catalog_object_id** | Option<**String**> | The catalog object ID referencing [CatalogTax](entity:CatalogTax). | [optional]
**catalog_version** | Option<**i64**> | The version of the catalog object that this tax references. | [optional]
**name** | Option<**String**> | The tax's name. | [optional]
**r#type** | Option<[**models::OrderLineItemTaxType**](OrderLineItemTaxType.md)> |  | [optional]
**percentage** | Option<**String**> | The percentage of the tax, as a string representation of a decimal number. For example, a value of `\"7.25\"` corresponds to a percentage of 7.25%. | [optional]
**applied_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**scope** | Option<[**models::OrderLineItemTaxScope**](OrderLineItemTaxScope.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


