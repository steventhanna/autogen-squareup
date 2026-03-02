# GiftCardActivityRefund

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**redeem_activity_id** | Option<**String**> | The ID of the refunded `REDEEM` gift card activity. Square populates this field if the  `payment_id` in the corresponding [RefundPayment](api-endpoint:Refunds-RefundPayment) request  represents a gift card redemption.  For applications that use a custom payment processing system, this field is required when creating a `REFUND` activity. The provided `REDEEM` activity ID must be linked to the same gift card. | [optional]
**amount_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**reference_id** | Option<**String**> | A client-specified ID that associates the gift card activity with an entity in another system. | [optional]
**payment_id** | Option<**String**> | The ID of the refunded payment. Square populates this field if the refund is for a  payment processed by Square. This field matches the `payment_id` in the corresponding [RefundPayment](api-endpoint:Refunds-RefundPayment) request. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


