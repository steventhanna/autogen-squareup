# Tender

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The tender's unique ID. It is the associated payment ID. | [optional]
**location_id** | Option<**String**> | The ID of the transaction's associated location. | [optional]
**transaction_id** | Option<**String**> | The ID of the tender's associated transaction. | [optional]
**created_at** | Option<**String**> | The timestamp for when the tender was created, in RFC 3339 format. | [optional][readonly]
**note** | Option<**String**> | An optional note associated with the tender at the time of payment. | [optional]
**amount_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**tip_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**processing_fee_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**customer_id** | Option<**String**> | If the tender is associated with a customer or represents a customer's card on file, this is the ID of the associated customer. | [optional]
**r#type** | [**models::TenderType**](TenderType.md) |  | 
**card_details** | Option<[**models::TenderCardDetails**](TenderCardDetails.md)> |  | [optional]
**cash_details** | Option<[**models::TenderCashDetails**](TenderCashDetails.md)> |  | [optional]
**bank_account_details** | Option<[**models::TenderBankAccountDetails**](TenderBankAccountDetails.md)> |  | [optional]
**buy_now_pay_later_details** | Option<[**models::TenderBuyNowPayLaterDetails**](TenderBuyNowPayLaterDetails.md)> |  | [optional]
**square_account_details** | Option<[**models::TenderSquareAccountDetails**](TenderSquareAccountDetails.md)> |  | [optional]
**additional_recipients** | Option<[**Vec<models::AdditionalRecipient>**](AdditionalRecipient.md)> | Additional recipients (other than the merchant) receiving a portion of this tender. For example, fees assessed on the purchase by a third party integration. | [optional]
**payment_id** | Option<**String**> | The ID of the [Payment](entity:Payment) that corresponds to this tender. This value is only present for payments created with the v2 Payments API. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


