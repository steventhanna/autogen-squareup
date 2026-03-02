# BulkUpdateCustomersResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**responses** | Option<[**std::collections::HashMap<String, models::UpdateCustomerResponse>**](UpdateCustomerResponse.md)> | A map of responses that correspond to individual update requests, represented by key-value pairs.  Each key is the customer ID that was specified for an update request and each value is the corresponding response. If the request succeeds, the value is the updated customer profile. If the request fails, the value contains any errors that occurred during the request. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any top-level errors that prevented the bulk operation from running. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


