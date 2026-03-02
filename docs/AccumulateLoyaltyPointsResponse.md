# AccumulateLoyaltyPointsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**event** | Option<[**models::LoyaltyEvent**](LoyaltyEvent.md)> |  | [optional]
**events** | Option<[**Vec<models::LoyaltyEvent>**](LoyaltyEvent.md)> | The resulting loyalty events. If the purchase qualifies for points, the `ACCUMULATE_POINTS` event is always included. When using the Orders API, the `ACCUMULATE_PROMOTION_POINTS` event is included if the purchase also qualifies for a loyalty promotion. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


