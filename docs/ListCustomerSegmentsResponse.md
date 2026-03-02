# ListCustomerSegmentsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**segments** | Option<[**Vec<models::CustomerSegment>**](CustomerSegment.md)> | The list of customer segments belonging to the associated Square account. | [optional]
**cursor** | Option<**String**> | A pagination cursor to be used in subsequent calls to `ListCustomerSegments` to retrieve the next set of query results. The cursor is only present if the request succeeded and additional results are available.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


