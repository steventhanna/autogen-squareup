# BatchDeleteCatalogObjectsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**deleted_object_ids** | Option<**Vec<String>**> | The IDs of all CatalogObjects deleted by this request. | [optional]
**deleted_at** | Option<**String**> | The database [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) of this deletion in RFC 3339 format, e.g., \"2016-09-04T23:59:33.123Z\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


