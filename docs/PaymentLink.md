# PaymentLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The Square-assigned ID of the payment link. | [optional][readonly]
**version** | **i32** | The Square-assigned version number, which is incremented each time an update is committed to the payment link. | 
**description** | Option<**String**> | The optional description of the `payment_link` object. It is primarily for use by your application and is not used anywhere. | [optional]
**order_id** | Option<**String**> | The ID of the order associated with the payment link. | [optional][readonly]
**checkout_options** | Option<[**models::CheckoutOptions**](CheckoutOptions.md)> |  | [optional]
**pre_populated_data** | Option<[**models::PrePopulatedData**](PrePopulatedData.md)> |  | [optional]
**url** | Option<**String**> | The shortened URL of the payment link. | [optional][readonly]
**long_url** | Option<**String**> | The long URL of the payment link. | [optional][readonly]
**created_at** | Option<**String**> | The timestamp when the payment link was created, in RFC 3339 format. | [optional]
**updated_at** | Option<**String**> | The timestamp when the payment link was last updated, in RFC 3339 format. | [optional]
**payment_note** | Option<**String**> | An optional note. After Square processes the payment, this note is added to the resulting `Payment`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


