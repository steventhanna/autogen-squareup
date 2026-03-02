# SearchLoyaltyAccountsRequestLoyaltyAccountQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mappings** | Option<[**Vec<models::LoyaltyAccountMapping>**](LoyaltyAccountMapping.md)> | The set of mappings to use in the loyalty account search.    This cannot be combined with `customer_ids`.    Max: 30 mappings | [optional]
**customer_ids** | Option<**Vec<String>**> | The set of customer IDs to use in the loyalty account search.    This cannot be combined with `mappings`.    Max: 30 customer IDs | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


