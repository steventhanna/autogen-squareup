# SubscriptionAction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of an action scoped to a subscription. | [optional]
**r#type** | Option<[**models::SubscriptionActionType**](SubscriptionActionType.md)> |  | [optional]
**effective_date** | Option<**String**> | The `YYYY-MM-DD`-formatted date when the action occurs on the subscription. | [optional]
**monthly_billing_anchor_date** | Option<**i32**> | The new billing anchor day value, for a `CHANGE_BILLING_ANCHOR_DATE` action. | [optional]
**phases** | Option<[**Vec<models::Phase>**](Phase.md)> | A list of Phases, to pass phase-specific information used in the swap. | [optional]
**new_plan_variation_id** | Option<**String**> | The target subscription plan variation that a subscription switches to, for a `SWAP_PLAN` action. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


