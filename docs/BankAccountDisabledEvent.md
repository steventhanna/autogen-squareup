# BankAccountDisabledEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**merchant_id** | Option<**String**> | The ID of the target merchant associated with the event. | [optional]
**location_id** | Option<**String**> | The ID of the target location associated with the event. | [optional]
**r#type** | Option<**String**> | The type of event this represents, `\"bank_account.disabled\"`. | [optional]
**event_id** | Option<**String**> | A unique ID for the event. | [optional]
**created_at** | Option<**String**> | Timestamp of when the event was disabled, in RFC 3339 format. | [optional][readonly]
**data** | Option<[**models::BankAccountDisabledEventData**](BankAccountDisabledEventData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


