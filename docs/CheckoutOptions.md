# CheckoutOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_tipping** | Option<**bool**> | Indicates whether the payment allows tipping. | [optional]
**custom_fields** | Option<[**Vec<models::CustomField>**](CustomField.md)> | The custom fields requesting information from the buyer. | [optional]
**subscription_plan_id** | Option<**String**> | The ID of the subscription plan for the buyer to pay and subscribe. For more information, see [Subscription Plan Checkout](https://developer.squareup.com/docs/checkout-api/subscription-plan-checkout). | [optional]
**redirect_url** | Option<**String**> | The confirmation page URL to redirect the buyer to after Square processes the payment. | [optional]
**merchant_support_email** | Option<**String**> | The email address that buyers can use to contact the seller. | [optional]
**ask_for_shipping_address** | Option<**bool**> | Indicates whether to include the address fields in the payment form. | [optional]
**accepted_payment_methods** | Option<[**models::AcceptedPaymentMethods**](AcceptedPaymentMethods.md)> |  | [optional]
**app_fee_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**shipping_fee** | Option<[**models::ShippingFee**](ShippingFee.md)> |  | [optional]
**enable_coupon** | Option<**bool**> | Indicates whether to include the `Add coupon` section for the buyer to provide a Square marketing coupon in the payment form. | [optional]
**enable_loyalty** | Option<**bool**> | Indicates whether to include the `REWARDS` section for the buyer to opt in to loyalty, redeem rewards in the payment form, or both. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


