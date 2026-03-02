# Refund

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The refund's unique ID. | 
**location_id** | **String** | The ID of the refund's associated location. | 
**transaction_id** | Option<**String**> | The ID of the transaction that the refunded tender is part of. | [optional]
**tender_id** | Option<**String**> | The ID of the refunded tender. | [optional]
**created_at** | Option<**String**> | The timestamp for when the refund was created, in RFC 3339 format. | [optional][readonly]
**reason** | **String** | The reason for the refund being issued. | 
**amount_money** | [**models::Money**](Money.md) |  | 
**status** | [**models::RefundStatus**](RefundStatus.md) |  | 
**processing_fee_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**additional_recipients** | Option<[**Vec<models::AdditionalRecipient>**](AdditionalRecipient.md)> | Additional recipients (other than the merchant) receiving a portion of this refund. For example, fees assessed on a refund of a purchase by a third party integration. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


