# ListDisputeEvidenceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**evidence** | Option<[**Vec<models::DisputeEvidence>**](DisputeEvidence.md)> | The list of evidence previously uploaded to the specified dispute. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Information about errors encountered during the request. | [optional]
**cursor** | Option<**String**> | The pagination cursor to be used in a subsequent request. If unset, this is the final response. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


