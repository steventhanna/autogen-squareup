# BatchUpsertCatalogObjectsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**objects** | Option<[**Vec<models::CatalogObject>**](CatalogObject.md)> | The created successfully created CatalogObjects. | [optional]
**updated_at** | Option<**String**> | The database [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) of this update in RFC 3339 format, e.g., \"2016-09-04T23:59:33.123Z\". | [optional]
**id_mappings** | Option<[**Vec<models::CatalogIdMapping>**](CatalogIdMapping.md)> | The mapping between client and server IDs for this upsert. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


