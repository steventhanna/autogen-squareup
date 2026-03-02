# OrderLineItemAppliedServiceCharge

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | A unique ID that identifies the applied service charge only within this order. | [optional]
**service_charge_uid** | **String** | The `uid` of the service charge that the applied service charge represents. It must reference a service charge present in the `order.service_charges` field.  This field is immutable. To change which service charges apply to a line item, delete and add a new `OrderLineItemAppliedServiceCharge`. | 
**applied_money** | Option<[**models::Money**](Money.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


