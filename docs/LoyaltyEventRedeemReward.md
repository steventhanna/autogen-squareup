# LoyaltyEventRedeemReward

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**loyalty_program_id** | **String** | The ID of the [loyalty program](entity:LoyaltyProgram). | [readonly]
**reward_id** | Option<**String**> | The ID of the redeemed [loyalty reward](entity:LoyaltyReward). This field is returned only if the event source is `LOYALTY_API`. | [optional][readonly]
**order_id** | Option<**String**> | The ID of the [order](entity:Order) that redeemed the reward. This field is returned only if the Orders API is used to process orders. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


