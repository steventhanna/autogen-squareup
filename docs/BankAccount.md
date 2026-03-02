# BankAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique, Square-issued identifier for the bank account. | 
**account_number_suffix** | **String** | The last few digits of the account number. | 
**country** | [**models::Country**](Country.md) |  | 
**currency** | [**models::Currency**](Currency.md) |  | 
**account_type** | [**models::BankAccountType**](BankAccountType.md) |  | 
**holder_name** | **String** | Name of the account holder. This name must match the name  on the targeted bank account record. | 
**primary_bank_identification_number** | **String** | Primary identifier for the bank. For more information, see  [Bank Accounts API](https://developer.squareup.com/docs/bank-accounts-api). | 
**secondary_bank_identification_number** | Option<**String**> | Secondary identifier for the bank. For more information, see  [Bank Accounts API](https://developer.squareup.com/docs/bank-accounts-api). | [optional]
**debit_mandate_reference_id** | Option<**String**> | Reference identifier that will be displayed to UK bank account owners when collecting direct debit authorization. Only required for UK bank accounts. | [optional]
**reference_id** | Option<**String**> | Client-provided identifier for linking the banking account to an entity in a third-party system (for example, a bank account number or a user identifier). | [optional]
**location_id** | Option<**String**> | The location to which the bank account belongs. | [optional]
**status** | [**models::BankAccountStatus**](BankAccountStatus.md) |  | 
**creditable** | **bool** | Indicates whether it is possible for Square to send money to this bank account. | 
**debitable** | **bool** | Indicates whether it is possible for Square to take money from this  bank account. | 
**fingerprint** | Option<**String**> | A Square-assigned, unique identifier for the bank account based on the account information. The account fingerprint can be used to compare account entries and determine if the they represent the same real-world bank account. | [optional]
**version** | Option<**i32**> | The current version of the `BankAccount`. | [optional]
**bank_name** | Option<**String**> | Read only. Name of actual financial institution.  For example \"Bank of America\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


