# OrderLineItemAppliedTax

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | A unique ID that identifies the applied tax only within this order. | [optional]
**tax_uid** | **String** | The `uid` of the tax for which this applied tax represents. It must reference a tax present in the `order.taxes` field.  This field is immutable. To change which taxes apply to a line item, delete and add a new `OrderLineItemAppliedTax`. | 
**applied_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**auto_applied** | Option<**bool**> | Indicates whether the tax was automatically applied to the order based on the catalog configuration. For an example, see [Automatically Apply Taxes to an Order](https://developer.squareup.com/docs/orders-api/apply-taxes-and-discounts/auto-apply-taxes). | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


