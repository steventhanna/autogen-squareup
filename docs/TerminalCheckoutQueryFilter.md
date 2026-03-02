# TerminalCheckoutQueryFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device_id** | Option<**String**> | The `TerminalCheckout` objects associated with a specific device. If no device is specified, then all `TerminalCheckout` objects for the merchant are displayed. | [optional]
**created_at** | Option<[**models::TimeRange**](TimeRange.md)> |  | [optional]
**status** | Option<**String**> | Filtered results with the desired status of the `TerminalCheckout`. Options: `PENDING`, `IN_PROGRESS`, `CANCEL_REQUESTED`, `CANCELED`, `COMPLETED` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


