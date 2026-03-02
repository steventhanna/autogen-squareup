# SearchSubscriptionsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Errors encountered during the request. | [optional]
**subscriptions** | Option<[**Vec<models::Subscription>**](Subscription.md)> | The subscriptions matching the specified query expressions. | [optional]
**cursor** | Option<**String**> | When the total number of resulting subscription exceeds the limit of a paged response,  the response includes a cursor for you to use in a subsequent request to fetch the next set of results. If the cursor is unset, the response contains the last page of the results.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


