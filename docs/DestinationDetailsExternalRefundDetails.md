# DestinationDetailsExternalRefundDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of external refund the seller paid to the buyer. It can be one of the following: - CHECK - Refunded using a physical check. - BANK_TRANSFER - Refunded using external bank transfer. - OTHER\\_GIFT\\_CARD - Refunded using a non-Square gift card. - CRYPTO - Refunded using a crypto currency. - SQUARE_CASH - Refunded using Square Cash App. - SOCIAL - Refunded using peer-to-peer payment applications. - EXTERNAL - A third-party application gathered this refund outside of Square. - EMONEY - Refunded using an E-money provider. - CARD - A credit or debit card that Square does not support. - STORED_BALANCE - Use for house accounts, store credit, and so forth. - FOOD_VOUCHER - Restaurant voucher provided by employers to employees to pay for meals - OTHER - A type not listed here. | 
**source** | **String** | A description of the external refund source. For example, \"Food Delivery Service\". | 
**source_id** | Option<**String**> | An ID to associate the refund to its originating source. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


