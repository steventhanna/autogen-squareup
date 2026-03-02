# BookingCustomAttributeOwnedDeletedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**merchant_id** | Option<**String**> | The ID of the seller associated with the event that triggered the event notification. | [optional]
**r#type** | Option<**String**> | The type of this event. The value is `\"booking.custom_attribute.owned.deleted\"`. | [optional]
**event_id** | Option<**String**> | A unique ID for the event notification. | [optional]
**created_at** | Option<**String**> | The timestamp that indicates when the event notification was created, in RFC 3339 format. | [optional][readonly]
**data** | Option<[**models::CustomAttributeEventData**](CustomAttributeEventData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


