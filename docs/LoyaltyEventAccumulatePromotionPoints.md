# LoyaltyEventAccumulatePromotionPoints

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**loyalty_program_id** | Option<**String**> | The Square-assigned ID of the [loyalty program](entity:LoyaltyProgram). | [optional][readonly]
**loyalty_promotion_id** | Option<**String**> | The Square-assigned ID of the [loyalty promotion](entity:LoyaltyPromotion). | [optional][readonly]
**points** | **i32** | The number of points earned by the event. | [readonly]
**order_id** | **String** | The ID of the [order](entity:Order) for which the buyer earned the promotion points. Only applications that use the Orders API to process orders can trigger this event. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


