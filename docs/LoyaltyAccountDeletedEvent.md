# LoyaltyAccountDeletedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**merchant_id** | Option<**String**> | The ID of the Square seller associated with the event. | [optional]
**r#type** | Option<**String**> | The type of event. For this event, the value is `loyalty.account.deleted`. | [optional]
**event_id** | Option<**String**> | The unique ID for the event, which is used for [idempotency support](https://developer.squareup.com/docs/webhooks/step4manage#webhooks-best-practices). | [optional]
**created_at** | Option<**String**> | The timestamp of when the event was created, in RFC 3339 format. | [optional][readonly]
**data** | Option<[**models::LoyaltyAccountDeletedEventData**](LoyaltyAccountDeletedEventData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


