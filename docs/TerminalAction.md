# TerminalAction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A unique ID for this `TerminalAction`. | [optional][readonly]
**device_id** | Option<**String**> | The unique Id of the device intended for this `TerminalAction`. The Id can be retrieved from /v2/devices api. | [optional]
**deadline_duration** | Option<**String**> | The duration as an RFC 3339 duration, after which the action will be automatically canceled. TerminalActions that are `PENDING` will be automatically `CANCELED` and have a cancellation reason of `TIMED_OUT`  Default: 5 minutes from creation  Maximum: 5 minutes | [optional]
**status** | Option<**String**> | The status of the `TerminalAction`. Options: `PENDING`, `IN_PROGRESS`, `CANCEL_REQUESTED`, `CANCELED`, `COMPLETED` | [optional][readonly]
**cancel_reason** | Option<[**models::ActionCancelReason**](ActionCancelReason.md)> |  | [optional]
**created_at** | Option<**String**> | The time when the `TerminalAction` was created as an RFC 3339 timestamp. | [optional][readonly]
**updated_at** | Option<**String**> | The time when the `TerminalAction` was last updated as an RFC 3339 timestamp. | [optional][readonly]
**app_id** | Option<**String**> | The ID of the application that created the action. | [optional][readonly]
**location_id** | Option<**String**> | The location id the action is attached to, if a link can be made. | [optional][readonly]
**r#type** | Option<[**models::TerminalActionActionType**](TerminalActionActionType.md)> |  | [optional]
**qr_code_options** | Option<[**models::QrCodeOptions**](QrCodeOptions.md)> |  | [optional]
**save_card_options** | Option<[**models::SaveCardOptions**](SaveCardOptions.md)> |  | [optional]
**signature_options** | Option<[**models::SignatureOptions**](SignatureOptions.md)> |  | [optional]
**confirmation_options** | Option<[**models::ConfirmationOptions**](ConfirmationOptions.md)> |  | [optional]
**receipt_options** | Option<[**models::ReceiptOptions**](ReceiptOptions.md)> |  | [optional]
**data_collection_options** | Option<[**models::DataCollectionOptions**](DataCollectionOptions.md)> |  | [optional]
**select_options** | Option<[**models::SelectOptions**](SelectOptions.md)> |  | [optional]
**device_metadata** | Option<[**models::DeviceMetadata**](DeviceMetadata.md)> |  | [optional]
**await_next_action** | Option<**bool**> | Indicates the action will be linked to another action and requires a waiting dialog to be displayed instead of returning to the idle screen on completion of the action.  Only supported on SIGNATURE, CONFIRMATION, DATA_COLLECTION, and SELECT types. | [optional]
**await_next_action_duration** | Option<**String**> | The timeout duration of the waiting dialog as an RFC 3339 duration, after which the waiting dialog will no longer be displayed and the Terminal will return to the idle screen.  Default: 5 minutes from when the waiting dialog is displayed  Maximum: 5 minutes | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


