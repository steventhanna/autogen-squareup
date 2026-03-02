# TerminalRefund

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A unique ID for this `TerminalRefund`. | [optional][readonly]
**refund_id** | Option<**String**> | The reference to the payment refund created by completing this `TerminalRefund`. | [optional][readonly]
**payment_id** | **String** | The unique ID of the payment being refunded. | 
**order_id** | Option<**String**> | The reference to the Square order ID for the payment identified by the `payment_id`. | [optional][readonly]
**amount_money** | [**models::Money**](Money.md) |  | 
**reason** | **String** | A description of the reason for the refund. | 
**device_id** | **String** | The unique ID of the device intended for this `TerminalRefund`. The Id can be retrieved from /v2/devices api. | 
**deadline_duration** | Option<**String**> | The RFC 3339 duration, after which the refund is automatically canceled. A `TerminalRefund` that is `PENDING` is automatically `CANCELED` and has a cancellation reason of `TIMED_OUT`.  Default: 5 minutes from creation.  Maximum: 5 minutes | [optional]
**status** | Option<**String**> | The status of the `TerminalRefund`. Options: `PENDING`, `IN_PROGRESS`, `CANCEL_REQUESTED`, `CANCELED`, or `COMPLETED`. | [optional][readonly]
**cancel_reason** | Option<[**models::ActionCancelReason**](ActionCancelReason.md)> |  | [optional]
**created_at** | Option<**String**> | The time when the `TerminalRefund` was created, as an RFC 3339 timestamp. | [optional][readonly]
**updated_at** | Option<**String**> | The time when the `TerminalRefund` was last updated, as an RFC 3339 timestamp. | [optional][readonly]
**app_id** | Option<**String**> | The ID of the application that created the refund. | [optional][readonly]
**location_id** | Option<**String**> | The location of the device where the `TerminalRefund` was directed. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


