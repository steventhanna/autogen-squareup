# DeviceCodePairedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**merchant_id** | Option<**String**> | The ID of the target merchant associated with the event. | [optional]
**location_id** | Option<**String**> | The ID of the target location associated with the event. | [optional]
**r#type** | Option<**String**> | The type of event this represents, `\"device.code.paired\"`. | [optional]
**event_id** | Option<**String**> | A unique ID for the event. | [optional]
**created_at** | Option<**String**> | RFC 3339 timestamp of when the event was created. | [optional][readonly]
**data** | Option<[**models::DeviceCodePairedEventData**](DeviceCodePairedEventData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


