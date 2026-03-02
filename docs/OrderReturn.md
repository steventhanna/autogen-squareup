# OrderReturn

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | A unique ID that identifies the return only within this order. | [optional]
**source_order_id** | Option<**String**> | An order that contains the original sale of these return line items. This is unset for unlinked returns. | [optional]
**return_line_items** | Option<[**Vec<models::OrderReturnLineItem>**](OrderReturnLineItem.md)> | A collection of line items that are being returned. | [optional]
**return_service_charges** | Option<[**Vec<models::OrderReturnServiceCharge>**](OrderReturnServiceCharge.md)> | A collection of service charges that are being returned. | [optional]
**return_taxes** | Option<[**Vec<models::OrderReturnTax>**](OrderReturnTax.md)> | A collection of references to taxes being returned for an order, including the total applied tax amount to be returned. The taxes must reference a top-level tax ID from the source order. | [optional][readonly]
**return_discounts** | Option<[**Vec<models::OrderReturnDiscount>**](OrderReturnDiscount.md)> | A collection of references to discounts being returned for an order, including the total applied discount amount to be returned. The discounts must reference a top-level discount ID from the source order. | [optional][readonly]
**return_tips** | Option<[**Vec<models::OrderReturnTip>**](OrderReturnTip.md)> | A collection of references to tips being returned for an order. | [optional]
**rounding_adjustment** | Option<[**models::OrderRoundingAdjustment**](OrderRoundingAdjustment.md)> |  | [optional]
**return_amounts** | Option<[**models::OrderMoneyAmounts**](OrderMoneyAmounts.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


