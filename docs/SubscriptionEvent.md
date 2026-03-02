# SubscriptionEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the subscription event. | 
**subscription_event_type** | [**models::SubscriptionEventSubscriptionEventType**](SubscriptionEventSubscriptionEventType.md) |  | 
**effective_date** | **String** | The `YYYY-MM-DD`-formatted date (for example, 2013-01-15) when the subscription event occurred. | 
**monthly_billing_anchor_date** | Option<**i32**> | The day-of-the-month the billing anchor date was changed to, if applicable. | [optional][readonly]
**info** | Option<[**models::SubscriptionEventInfo**](SubscriptionEventInfo.md)> |  | [optional]
**phases** | Option<[**Vec<models::Phase>**](Phase.md)> | A list of Phases, to pass phase-specific information used in the swap. | [optional]
**plan_variation_id** | **String** | The ID of the subscription plan variation associated with the subscription. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


