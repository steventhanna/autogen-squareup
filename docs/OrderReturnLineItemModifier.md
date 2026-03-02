# OrderReturnLineItemModifier

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | A unique ID that identifies the return modifier only within this order. | [optional]
**source_modifier_uid** | Option<**String**> | The modifier `uid` from the order's line item that contains the original sale of this line item modifier. | [optional]
**catalog_object_id** | Option<**String**> | The catalog object ID referencing [CatalogModifier](entity:CatalogModifier). | [optional]
**catalog_version** | Option<**i64**> | The version of the catalog object that this line item modifier references. | [optional]
**name** | Option<**String**> | The name of the item modifier. | [optional]
**base_price_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**total_price_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**quantity** | Option<**String**> | The quantity of the line item modifier. The modifier quantity can be 0 or more. For example, suppose a restaurant offers a cheeseburger on the menu. When a buyer orders this item, the restaurant records the purchase by creating an `Order` object with a line item for a burger. The line item includes a line item modifier: the name is cheese and the quantity is 1. The buyer has the option to order extra cheese (or no cheese). If the buyer chooses the extra cheese option, the modifier quantity increases to 2. If the buyer does not want any cheese, the modifier quantity is set to 0. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


