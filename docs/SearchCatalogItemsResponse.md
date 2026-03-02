# SearchCatalogItemsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**items** | Option<[**Vec<models::CatalogObject>**](CatalogObject.md)> | Returned items matching the specified query expressions. | [optional]
**cursor** | Option<**String**> | Pagination token used in the next request to return more of the search result. | [optional]
**matched_variation_ids** | Option<**Vec<String>**> | Ids of returned item variations matching the specified query expression. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


