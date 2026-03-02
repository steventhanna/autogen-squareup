# ListWebhookSubscriptionsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Information on errors encountered during the request. | [optional]
**subscriptions** | Option<[**Vec<models::WebhookSubscription>**](WebhookSubscription.md)> | The requested list of [Subscription](entity:WebhookSubscription)s. | [optional]
**cursor** | Option<**String**> | The pagination cursor to be used in a subsequent request. If empty, this is the final response.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


