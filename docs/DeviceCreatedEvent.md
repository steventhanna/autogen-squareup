# DeviceCreatedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**merchant_id** | Option<**String**> | The merchant the newly created device belongs to. | [optional]
**r#type** | Option<**String**> | The type of event this represents. The value is `\"device.created\"`. | [optional]
**event_id** | Option<**String**> | A UUID that uniquely identifies this device creation event. | [optional]
**created_at** | Option<**String**> | The time when the device creation event was first created, in RFC 3339 format. | [optional][readonly]
**data** | Option<[**models::DeviceCreatedEventData**](DeviceCreatedEventData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


