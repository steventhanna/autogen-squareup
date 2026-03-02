# LoyaltyEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The Square-assigned ID of the loyalty event. | [readonly]
**r#type** | [**models::LoyaltyEventType**](LoyaltyEventType.md) |  | 
**created_at** | **String** | The timestamp when the event was created, in RFC 3339 format. | [readonly]
**accumulate_points** | Option<[**models::LoyaltyEventAccumulatePoints**](LoyaltyEventAccumulatePoints.md)> |  | [optional]
**create_reward** | Option<[**models::LoyaltyEventCreateReward**](LoyaltyEventCreateReward.md)> |  | [optional]
**redeem_reward** | Option<[**models::LoyaltyEventRedeemReward**](LoyaltyEventRedeemReward.md)> |  | [optional]
**delete_reward** | Option<[**models::LoyaltyEventDeleteReward**](LoyaltyEventDeleteReward.md)> |  | [optional]
**adjust_points** | Option<[**models::LoyaltyEventAdjustPoints**](LoyaltyEventAdjustPoints.md)> |  | [optional]
**loyalty_account_id** | **String** | The ID of the [loyalty account](entity:LoyaltyAccount) associated with the event. | [readonly]
**location_id** | Option<**String**> | The ID of the [location](entity:Location) where the event occurred. | [optional][readonly]
**source** | [**models::LoyaltyEventSource**](LoyaltyEventSource.md) |  | 
**expire_points** | Option<[**models::LoyaltyEventExpirePoints**](LoyaltyEventExpirePoints.md)> |  | [optional]
**other_event** | Option<[**models::LoyaltyEventOther**](LoyaltyEventOther.md)> |  | [optional]
**accumulate_promotion_points** | Option<[**models::LoyaltyEventAccumulatePromotionPoints**](LoyaltyEventAccumulatePromotionPoints.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


