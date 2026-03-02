# LoyaltyEventCreateReward

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**loyalty_program_id** | **String** | The ID of the [loyalty program](entity:LoyaltyProgram). | [readonly]
**reward_id** | Option<**String**> | The Square-assigned ID of the created [loyalty reward](entity:LoyaltyReward). This field is returned only if the event source is `LOYALTY_API`. | [optional][readonly]
**points** | **i32** | The loyalty points used to create the reward. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


