# LoyaltyReward

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The Square-assigned ID of the loyalty reward. | [optional][readonly]
**status** | Option<[**models::LoyaltyRewardStatus**](LoyaltyRewardStatus.md)> |  | [optional]
**loyalty_account_id** | **String** | The Square-assigned ID of the [loyalty account](entity:LoyaltyAccount) to which the reward belongs. | 
**reward_tier_id** | **String** | The Square-assigned ID of the [reward tier](entity:LoyaltyProgramRewardTier) used to create the reward. | 
**points** | Option<**i32**> | The number of loyalty points used for the reward. | [optional][readonly]
**order_id** | Option<**String**> | The Square-assigned ID of the [order](entity:Order) to which the reward is attached. | [optional]
**created_at** | Option<**String**> | The timestamp when the reward was created, in RFC 3339 format. | [optional][readonly]
**updated_at** | Option<**String**> | The timestamp when the reward was last updated, in RFC 3339 format. | [optional][readonly]
**redeemed_at** | Option<**String**> | The timestamp when the reward was redeemed, in RFC 3339 format. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


