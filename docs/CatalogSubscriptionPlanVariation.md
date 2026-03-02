# CatalogSubscriptionPlanVariation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the plan variation. | 
**phases** | [**Vec<models::SubscriptionPhase>**](SubscriptionPhase.md) | A list containing each [SubscriptionPhase](entity:SubscriptionPhase) for this plan variation. | 
**subscription_plan_id** | Option<**String**> | The id of the subscription plan, if there is one. | [optional]
**monthly_billing_anchor_date** | Option<**i64**> | The day of the month the billing period starts. | [optional]
**can_prorate** | Option<**bool**> | Whether bills for this plan variation can be split for proration. | [optional]
**successor_plan_variation_id** | Option<**String**> | The ID of a \"successor\" plan variation to this one. If the field is set, and this object is disabled at all locations, it indicates that this variation is deprecated and the object identified by the successor ID be used in its stead. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


