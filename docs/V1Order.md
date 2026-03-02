# V1Order

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**id** | Option<**String**> | The order's unique identifier. | [optional]
**buyer_email** | Option<**String**> | The email address of the order's buyer. | [optional]
**recipient_name** | Option<**String**> | The name of the order's buyer. | [optional]
**recipient_phone_number** | Option<**String**> | The phone number to use for the order's delivery. | [optional]
**state** | Option<[**models::V1OrderState**](V1OrderState.md)> |  | [optional]
**shipping_address** | Option<[**models::Address**](Address.md)> |  | [optional]
**subtotal_money** | Option<[**models::V1Money**](V1Money.md)> |  | [optional]
**total_shipping_money** | Option<[**models::V1Money**](V1Money.md)> |  | [optional]
**total_tax_money** | Option<[**models::V1Money**](V1Money.md)> |  | [optional]
**total_price_money** | Option<[**models::V1Money**](V1Money.md)> |  | [optional]
**total_discount_money** | Option<[**models::V1Money**](V1Money.md)> |  | [optional]
**created_at** | Option<**String**> | The time when the order was created, in ISO 8601 format. | [optional]
**updated_at** | Option<**String**> | The time when the order was last modified, in ISO 8601 format. | [optional]
**expires_at** | Option<**String**> | The time when the order expires if no action is taken, in ISO 8601 format. | [optional]
**payment_id** | Option<**String**> | The unique identifier of the payment associated with the order. | [optional]
**buyer_note** | Option<**String**> | A note provided by the buyer when the order was created, if any. | [optional]
**completed_note** | Option<**String**> | A note provided by the merchant when the order's state was set to COMPLETED, if any | [optional]
**refunded_note** | Option<**String**> | A note provided by the merchant when the order's state was set to REFUNDED, if any. | [optional]
**canceled_note** | Option<**String**> | A note provided by the merchant when the order's state was set to CANCELED, if any. | [optional]
**tender** | Option<[**models::V1Tender**](V1Tender.md)> |  | [optional]
**order_history** | Option<[**Vec<models::V1OrderHistoryEntry>**](V1OrderHistoryEntry.md)> | The history of actions associated with the order. | [optional]
**promo_code** | Option<**String**> | The promo code provided by the buyer, if any. | [optional]
**btc_receive_address** | Option<**String**> | For Bitcoin transactions, the address that the buyer sent Bitcoin to. | [optional]
**btc_price_satoshi** | Option<**f64**> | For Bitcoin transactions, the price of the buyer's order in satoshi (100 million satoshi equals 1 BTC). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


