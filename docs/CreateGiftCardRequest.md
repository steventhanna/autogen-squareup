# CreateGiftCardRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | **String** | A unique identifier for this request, used to ensure idempotency. For more information,  see [Idempotency](https://developer.squareup.com/docs/build-basics/common-api-patterns/idempotency). | 
**location_id** | **String** | The ID of the [location](entity:Location) where the gift card should be registered for  reporting purposes. Gift cards can be redeemed at any of the seller's locations. | 
**gift_card** | [**models::GiftCard**](GiftCard.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


