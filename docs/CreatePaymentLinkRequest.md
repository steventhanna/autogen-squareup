# CreatePaymentLinkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique string that identifies this `CreatePaymentLinkRequest` request. If you do not provide a unique string (or provide an empty string as the value), the endpoint treats each request as independent.  For more information, see [Idempotency](https://developer.squareup.com/docs/working-with-apis/idempotency). | [optional]
**description** | Option<**String**> | A description of the payment link. You provide this optional description that is useful in your application context. It is not used anywhere. | [optional]
**quick_pay** | Option<[**models::QuickPay**](QuickPay.md)> |  | [optional]
**order** | Option<[**models::Order**](Order.md)> |  | [optional]
**checkout_options** | Option<[**models::CheckoutOptions**](CheckoutOptions.md)> |  | [optional]
**pre_populated_data** | Option<[**models::PrePopulatedData**](PrePopulatedData.md)> |  | [optional]
**payment_note** | Option<**String**> | A note for the payment. After processing the payment, Square adds this note to the resulting `Payment`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


