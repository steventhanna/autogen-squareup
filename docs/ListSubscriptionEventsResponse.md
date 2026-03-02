# ListSubscriptionEventsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Errors encountered during the request. | [optional]
**subscription_events** | Option<[**Vec<models::SubscriptionEvent>**](SubscriptionEvent.md)> | The retrieved subscription events. | [optional]
**cursor** | Option<**String**> | When the total number of resulting subscription events exceeds the limit of a paged response,  the response includes a cursor for you to use in a subsequent request to fetch the next set of events. If the cursor is unset, the response contains the last page of the results.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


