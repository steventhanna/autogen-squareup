# BulkDeleteMerchantCustomAttributesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**values** | [**std::collections::HashMap<String, models::BulkDeleteMerchantCustomAttributesResponseMerchantCustomAttributeDeleteResponse>**](BulkDeleteMerchantCustomAttributesResponseMerchantCustomAttributeDeleteResponse.md) | A map of responses that correspond to individual delete requests. Each response has the same key as the corresponding request. | 
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


