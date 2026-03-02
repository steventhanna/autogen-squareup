# BatchRetrieveCatalogObjectsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**objects** | Option<[**Vec<models::CatalogObject>**](CatalogObject.md)> | A list of [CatalogObject](entity:CatalogObject)s returned. | [optional]
**related_objects** | Option<[**Vec<models::CatalogObject>**](CatalogObject.md)> | A list of [CatalogObject](entity:CatalogObject)s referenced by the object in the `objects` field. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


