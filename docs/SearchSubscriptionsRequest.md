# SearchSubscriptionsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cursor** | Option<**String**> | When the total number of resulting subscriptions exceeds the limit of a paged response,  specify the cursor returned from a preceding response here to fetch the next set of results. If the cursor is unset, the response contains the last page of the results.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). | [optional]
**limit** | Option<**i32**> | The upper limit on the number of subscriptions to return in a paged response. | [optional]
**query** | Option<[**models::SearchSubscriptionsQuery**](SearchSubscriptionsQuery.md)> |  | [optional]
**include** | Option<**Vec<String>**> | An option to include related information in the response.   The supported values are:   - `actions`: to include scheduled actions on the targeted subscriptions. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


