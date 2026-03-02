# RefundPaymentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | **String** |  A unique string that identifies this `RefundPayment` request. The key can be any valid string but must be unique for every `RefundPayment` request.  Keys are limited to a max of 45 characters - however, the number of allowed characters might be less than 45, if multi-byte characters are used.  For more information, see [Idempotency](https://developer.squareup.com/docs/working-with-apis/idempotency). | 
**amount_money** | [**models::Money**](Money.md) |  | 
**app_fee_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**payment_id** | Option<**String**> | The unique ID of the payment being refunded. Required when unlinked=false, otherwise must not be set. | [optional]
**destination_id** | Option<**String**> | The ID indicating where funds will be refunded to. Required for unlinked refunds. For more information, see [Process an Unlinked Refund](https://developer.squareup.com/docs/refunds-api/unlinked-refunds).  For refunds linked to Square payments, `destination_id` is usually omitted; in this case, funds will be returned to the original payment source. The field may be specified in order to request a cross-method refund to a gift card. For more information, see [Cross-method refunds to gift cards](https://developer.squareup.com/docs/payments-api/refund-payments#cross-method-refunds-to-gift-cards). | [optional]
**unlinked** | Option<**bool**> | Indicates that the refund is not linked to a Square payment. If set to true, `destination_id` and `location_id` must be supplied while `payment_id` must not be provided. | [optional]
**location_id** | Option<**String**> | The location ID associated with the unlinked refund. Required for requests specifying `unlinked=true`. Otherwise, if included when `unlinked=false`, will throw an error. | [optional]
**customer_id** | Option<**String**> | The [Customer](entity:Customer) ID of the customer associated with the refund. This is required if the `destination_id` refers to a card on file created using the Cards API. Only allowed when `unlinked=true`. | [optional]
**reason** | Option<**String**> | A description of the reason for the refund. | [optional]
**payment_version_token** | Option<**String**> |  Used for optimistic concurrency. This opaque token identifies the current `Payment` version that the caller expects. If the server has a different version of the Payment, the update fails and a response with a VERSION_MISMATCH error is returned. If the versions match, or the field is not provided, the refund proceeds as normal. | [optional]
**team_member_id** | Option<**String**> | An optional [TeamMember](entity:TeamMember) ID to associate with this refund. | [optional]
**cash_details** | Option<[**models::DestinationDetailsCashRefundDetails**](DestinationDetailsCashRefundDetails.md)> |  | [optional]
**external_details** | Option<[**models::DestinationDetailsExternalRefundDetails**](DestinationDetailsExternalRefundDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


