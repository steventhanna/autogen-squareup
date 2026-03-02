# SearchCatalogObjectsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**cursor** | Option<**String**> | The pagination cursor to be used in a subsequent request. If unset, this is the final response. See [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination) for more information. | [optional]
**objects** | Option<[**Vec<models::CatalogObject>**](CatalogObject.md)> | The CatalogObjects returned. | [optional]
**related_objects** | Option<[**Vec<models::CatalogObject>**](CatalogObject.md)> | A list of CatalogObjects referenced by the objects in the `objects` field. | [optional]
**latest_time** | Option<**String**> | When the associated product catalog was last updated. Will match the value for `end_time` or `cursor` if either field is included in the `SearchCatalog` request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


