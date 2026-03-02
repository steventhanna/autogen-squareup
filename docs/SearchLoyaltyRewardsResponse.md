# SearchLoyaltyRewardsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**rewards** | Option<[**Vec<models::LoyaltyReward>**](LoyaltyReward.md)> | The loyalty rewards that satisfy the search criteria. These are returned in descending order by `updated_at`. | [optional]
**cursor** | Option<**String**> | The pagination cursor to be used in a subsequent  request. If empty, this is the final response. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


