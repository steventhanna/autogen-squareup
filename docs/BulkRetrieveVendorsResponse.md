# BulkRetrieveVendorsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**responses** | Option<[**std::collections::HashMap<String, models::RetrieveVendorResponse>**](RetrieveVendorResponse.md)> | The set of [RetrieveVendorResponse](entity:RetrieveVendorResponse) objects encapsulating successfully retrieved [Vendor](entity:Vendor) objects or error responses for failed attempts. The set is represented by  a collection of `Vendor`-ID/`Vendor`-object or `Vendor`-ID/error-object pairs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


