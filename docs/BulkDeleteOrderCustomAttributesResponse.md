# BulkDeleteOrderCustomAttributesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**values** | [**std::collections::HashMap<String, models::DeleteOrderCustomAttributeResponse>**](DeleteOrderCustomAttributeResponse.md) |  A map of responses that correspond to individual delete requests. Each response has the same ID  as the corresponding request and contains either a `custom_attribute` or an `errors` field. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


