# CustomerCustomAttributeDefinitionOwnedCreatedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**merchant_id** | Option<**String**> | The ID of the seller associated with the event that triggered the event notification. | [optional]
**r#type** | Option<**String**> | The type of this event. The value is `\"customer.custom_attribute_definition.owned.created\"`. | [optional]
**event_id** | Option<**String**> | A unique ID for the event notification. | [optional]
**created_at** | Option<**String**> | The timestamp that indicates when the event notification was created, in RFC 3339 format. | [optional][readonly]
**data** | Option<[**models::CustomAttributeDefinitionEventData**](CustomAttributeDefinitionEventData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


