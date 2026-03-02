# ListJobsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**jobs** | Option<[**Vec<models::Job>**](Job.md)> | The retrieved jobs. A single paged response contains up to 100 jobs. | [optional]
**cursor** | Option<**String**> | An opaque cursor used to retrieve the next page of results. This field is present only if the request succeeded and additional results are available. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | The errors that occurred during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


