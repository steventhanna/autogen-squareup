# BulkUpsertOrderCustomAttributesRequestUpsertCustomAttribute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**custom_attribute** | [**models::CustomAttribute**](CustomAttribute.md) |  | 
**idempotency_key** | Option<**String**> | A unique identifier for this request, used to ensure idempotency.  For more information, see [Idempotency](https://developer.squareup.com/docs/build-basics/common-api-patterns/idempotency). | [optional]
**order_id** | **String** | The ID of the target [order](entity:Order). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


