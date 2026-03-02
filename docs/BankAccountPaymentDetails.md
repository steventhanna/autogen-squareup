# BankAccountPaymentDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bank_name** | Option<**String**> | The name of the bank associated with the bank account. | [optional]
**transfer_type** | Option<**String**> | The type of the bank transfer. The type can be `ACH` or `UNKNOWN`. | [optional]
**account_ownership_type** | Option<**String**> | The ownership type of the bank account performing the transfer. The type can be `INDIVIDUAL`, `COMPANY`, or `ACCOUNT_TYPE_UNKNOWN`. | [optional]
**fingerprint** | Option<**String**> | Uniquely identifies the bank account for this seller and can be used to determine if payments are from the same bank account. | [optional]
**country** | Option<**String**> | The two-letter ISO code representing the country the bank account is located in. | [optional]
**statement_description** | Option<**String**> | The statement description as sent to the bank. | [optional]
**ach_details** | Option<[**models::AchDetails**](ACHDetails.md)> |  | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Information about errors encountered during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


