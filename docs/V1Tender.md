# V1Tender

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The tender's unique ID. | [optional]
**r#type** | Option<[**models::V1TenderType**](V1TenderType.md)> |  | [optional]
**name** | Option<**String**> | A human-readable description of the tender. | [optional]
**employee_id** | Option<**String**> | The ID of the employee that processed the tender. | [optional]
**receipt_url** | Option<**String**> | The URL of the receipt for the tender. | [optional]
**card_brand** | Option<[**models::V1TenderCardBrand**](V1TenderCardBrand.md)> |  | [optional]
**pan_suffix** | Option<**String**> | The last four digits of the provided credit card's account number. | [optional]
**entry_method** | Option<[**models::V1TenderEntryMethod**](V1TenderEntryMethod.md)> |  | [optional]
**payment_note** | Option<**String**> | Notes entered by the merchant about the tender at the time of payment, if any. Typically only present for tender with the type: OTHER. | [optional]
**total_money** | Option<[**models::V1Money**](V1Money.md)> |  | [optional]
**tendered_money** | Option<[**models::V1Money**](V1Money.md)> |  | [optional]
**tendered_at** | Option<**String**> | The time when the tender was created, in ISO 8601 format. | [optional]
**settled_at** | Option<**String**> | The time when the tender was settled, in ISO 8601 format. | [optional]
**change_back_money** | Option<[**models::V1Money**](V1Money.md)> |  | [optional]
**refunded_money** | Option<[**models::V1Money**](V1Money.md)> |  | [optional]
**is_exchange** | Option<**bool**> | Indicates whether or not the tender is associated with an exchange. If is_exchange is true, the tender represents the value of goods returned in an exchange not the actual money paid. The exchange value reduces the tender amounts needed to pay for items purchased in the exchange. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


