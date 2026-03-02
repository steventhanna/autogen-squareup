# GiftCardActivityRedeem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount_money** | [**models::Money**](Money.md) |  | 
**payment_id** | Option<**String**> | The ID of the payment that represents the gift card redemption. Square populates this field  if the payment was processed by Square. | [optional][readonly]
**reference_id** | Option<**String**> | A client-specified ID that associates the gift card activity with an entity in another system.   Applications that use a custom payment processing system can use this field to track information related to an order or payment. | [optional]
**status** | Option<[**models::GiftCardActivityRedeemStatus**](GiftCardActivityRedeemStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


