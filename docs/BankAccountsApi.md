# \BankAccountsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bank_account**](BankAccountsApi.md#get_bank_account) | **GET** /v2/bank-accounts/{bank_account_id} | GetBankAccount
[**get_bank_account_by_v1_id**](BankAccountsApi.md#get_bank_account_by_v1_id) | **GET** /v2/bank-accounts/by-v1-id/{v1_bank_account_id} | GetBankAccountByV1Id
[**list_bank_accounts**](BankAccountsApi.md#list_bank_accounts) | **GET** /v2/bank-accounts | ListBankAccounts



## get_bank_account

> models::GetBankAccountResponse get_bank_account(bank_account_id)
GetBankAccount

Returns details of a [BankAccount](entity:BankAccount) linked to a Square account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bank_account_id** | **String** | Square-issued ID of the desired `BankAccount`. | [required] |

### Return type

[**models::GetBankAccountResponse**](GetBankAccountResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bank_account_by_v1_id

> models::GetBankAccountByV1IdResponse get_bank_account_by_v1_id(v1_bank_account_id)
GetBankAccountByV1Id

Returns details of a [BankAccount](entity:BankAccount) identified by V1 bank account ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_bank_account_id** | **String** | Connect V1 ID of the desired `BankAccount`. For more information, see  [Retrieve a bank account by using an ID issued by V1 Bank Accounts API](https://developer.squareup.com/docs/bank-accounts-api#retrieve-a-bank-account-by-using-an-id-issued-by-v1-bank-accounts-api). | [required] |

### Return type

[**models::GetBankAccountByV1IdResponse**](GetBankAccountByV1IdResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_bank_accounts

> models::ListBankAccountsResponse list_bank_accounts(cursor, limit, location_id)
ListBankAccounts

Returns a list of [BankAccount](entity:BankAccount) objects linked to a Square account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | The pagination cursor returned by a previous call to this endpoint. Use it in the next `ListBankAccounts` request to retrieve the next set  of results.  See the [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination) guide for more information. |  |
**limit** | Option<**i32**> | Upper limit on the number of bank accounts to return in the response.  Currently, 1000 is the largest supported limit. You can specify a limit  of up to 1000 bank accounts. This is also the default limit. |  |
**location_id** | Option<**String**> | Location ID. You can specify this optional filter  to retrieve only the linked bank accounts belonging to a specific location. |  |

### Return type

[**models::ListBankAccountsResponse**](ListBankAccountsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

