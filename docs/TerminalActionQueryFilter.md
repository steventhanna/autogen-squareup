# TerminalActionQueryFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device_id** | Option<**String**> | `TerminalAction`s associated with a specific device. If no device is specified then all `TerminalAction`s for the merchant will be displayed. | [optional]
**created_at** | Option<[**models::TimeRange**](TimeRange.md)> |  | [optional]
**status** | Option<**String**> | Filter results with the desired status of the `TerminalAction` Options: `PENDING`, `IN_PROGRESS`, `CANCEL_REQUESTED`, `CANCELED`, `COMPLETED` | [optional]
**r#type** | Option<[**models::TerminalActionActionType**](TerminalActionActionType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


