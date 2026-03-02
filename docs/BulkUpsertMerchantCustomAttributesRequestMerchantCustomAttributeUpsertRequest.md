# BulkUpsertMerchantCustomAttributesRequestMerchantCustomAttributeUpsertRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**merchant_id** | **String** | The ID of the target [merchant](entity:Merchant). | 
**custom_attribute** | [**models::CustomAttribute**](CustomAttribute.md) |  | 
**idempotency_key** | Option<**String**> | A unique identifier for this individual upsert request, used to ensure idempotency. For more information, see [Idempotency](https://developer.squareup.com/docs/build-basics/common-api-patterns/idempotency). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


