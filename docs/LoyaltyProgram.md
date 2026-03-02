# LoyaltyProgram

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The Square-assigned ID of the loyalty program. Updates to  the loyalty program do not modify the identifier. | [optional][readonly]
**status** | Option<[**models::LoyaltyProgramStatus**](LoyaltyProgramStatus.md)> |  | [optional]
**reward_tiers** | Option<[**Vec<models::LoyaltyProgramRewardTier>**](LoyaltyProgramRewardTier.md)> | The list of rewards for buyers, sorted by ascending points. | [optional]
**expiration_policy** | Option<[**models::LoyaltyProgramExpirationPolicy**](LoyaltyProgramExpirationPolicy.md)> |  | [optional]
**terminology** | Option<[**models::LoyaltyProgramTerminology**](LoyaltyProgramTerminology.md)> |  | [optional]
**location_ids** | Option<**Vec<String>**> | The [locations](entity:Location) at which the program is active. | [optional]
**created_at** | Option<**String**> | The timestamp when the program was created, in RFC 3339 format. | [optional][readonly]
**updated_at** | Option<**String**> | The timestamp when the reward was last updated, in RFC 3339 format. | [optional][readonly]
**accrual_rules** | Option<[**Vec<models::LoyaltyProgramAccrualRule>**](LoyaltyProgramAccrualRule.md)> | Defines how buyers can earn loyalty points from the base loyalty program. To check for associated [loyalty promotions](entity:LoyaltyPromotion) that enable buyers to earn extra points, call [ListLoyaltyPromotions](api-endpoint:Loyalty-ListLoyaltyPromotions). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


