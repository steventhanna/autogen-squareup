# VendorUpdatedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**merchant_id** | Option<**String**> | The ID of a seller associated with this event. | [optional]
**location_id** | Option<**String**> | The ID of a seller location associated with this event, if the event is associated with the location. | [optional]
**r#type** | Option<**String**> | The type of this event. The value is `\"vendor.updated\".` | [optional]
**event_id** | Option<**String**> | A unique ID for this webhoook event. | [optional]
**created_at** | Option<**String**> | The RFC 3339-formatted time when the underlying event data object is created. | [optional][readonly]
**data** | Option<[**models::VendorUpdatedEventData**](VendorUpdatedEventData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


