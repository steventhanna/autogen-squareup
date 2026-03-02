# BulkCreateCustomersRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customers** | [**std::collections::HashMap<String, models::BulkCreateCustomerData>**](BulkCreateCustomerData.md) | A map of 1 to 100 individual create requests, represented by `idempotency key: { customer data }` key-value pairs.  Each key is an [idempotency key](https://developer.squareup.com/docs/build-basics/common-api-patterns/idempotency) that uniquely identifies the create request. Each value contains the customer data used to create the customer profile. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


