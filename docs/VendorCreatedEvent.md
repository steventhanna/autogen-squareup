# VendorCreatedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**merchant_id** | Option<**String**> | The ID of a seller associated with this event. | [optional]
**location_id** | Option<**String**> | The ID of a location associated with the event, if the event is associated with the location of the seller. | [optional]
**r#type** | Option<**String**> | The type of this event. The value is `\"vendor.created\".` | [optional]
**event_id** | Option<**String**> | A unique ID for this event. | [optional]
**created_at** | Option<**String**> | The RFC 3339-formatted time when the underlying event data object is created. | [optional][readonly]
**data** | Option<[**models::VendorCreatedEventData**](VendorCreatedEventData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


