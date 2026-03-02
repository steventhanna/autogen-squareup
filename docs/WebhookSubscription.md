# WebhookSubscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A Square-generated unique ID for the subscription. | [optional][readonly]
**name** | Option<**String**> | The name of this subscription. | [optional]
**enabled** | Option<**bool**> | Indicates whether the subscription is enabled (`true`) or not (`false`). | [optional]
**event_types** | Option<**Vec<String>**> | The event types associated with this subscription. | [optional]
**notification_url** | Option<**String**> | The URL to which webhooks are sent. | [optional]
**api_version** | Option<**String**> | The API version of the subscription. This field is optional for `CreateWebhookSubscription`.  The value defaults to the API version used by the application. | [optional]
**signature_key** | Option<**String**> | The Square-generated signature key used to validate the origin of the webhook event. | [optional][readonly]
**created_at** | Option<**String**> | The timestamp of when the subscription was created, in RFC 3339 format. For example, \"2016-09-04T23:59:33.123Z\". | [optional][readonly]
**updated_at** | Option<**String**> | The timestamp of when the subscription was last updated, in RFC 3339 format. For example, \"2016-09-04T23:59:33.123Z\". | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


