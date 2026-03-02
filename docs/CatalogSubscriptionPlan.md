# CatalogSubscriptionPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the plan. | 
**phases** | Option<[**Vec<models::SubscriptionPhase>**](SubscriptionPhase.md)> | A list of SubscriptionPhase containing the [SubscriptionPhase](entity:SubscriptionPhase) for this plan. This field it required. Not including this field will throw a REQUIRED_FIELD_MISSING error | [optional]
**subscription_plan_variations** | Option<[**Vec<models::CatalogObject>**](CatalogObject.md)> | The list of subscription plan variations available for this product | [optional]
**eligible_item_ids** | Option<**Vec<String>**> | The list of IDs of `CatalogItems` that are eligible for subscription by this SubscriptionPlan's variations. | [optional]
**eligible_category_ids** | Option<**Vec<String>**> | The list of IDs of `CatalogCategory` that are eligible for subscription by this SubscriptionPlan's variations. | [optional]
**all_items** | Option<**bool**> | If true, all items in the merchant's catalog are subscribable by this SubscriptionPlan. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


