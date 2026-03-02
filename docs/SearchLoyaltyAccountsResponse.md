# SearchLoyaltyAccountsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**loyalty_accounts** | Option<[**Vec<models::LoyaltyAccount>**](LoyaltyAccount.md)> | The loyalty accounts that met the search criteria,   in order of creation date. | [optional]
**cursor** | Option<**String**> | The pagination cursor to use in a subsequent  request. If empty, this is the final response. For more information,  see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


