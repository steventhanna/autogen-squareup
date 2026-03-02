# V1UpdateOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | [**models::V1UpdateOrderRequestAction**](V1UpdateOrderRequestAction.md) |  | 
**shipped_tracking_number** | Option<**String**> | The tracking number of the shipment associated with the order. Only valid if action is COMPLETE. | [optional]
**completed_note** | Option<**String**> | A merchant-specified note about the completion of the order. Only valid if action is COMPLETE. | [optional]
**refunded_note** | Option<**String**> | A merchant-specified note about the refunding of the order. Only valid if action is REFUND. | [optional]
**canceled_note** | Option<**String**> | A merchant-specified note about the canceling of the order. Only valid if action is CANCEL. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


