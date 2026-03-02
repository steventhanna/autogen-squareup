# LoyaltyPromotion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The Square-assigned ID of the promotion. | [optional][readonly]
**name** | **String** | The name of the promotion. | 
**incentive** | [**models::LoyaltyPromotionIncentive**](LoyaltyPromotionIncentive.md) |  | 
**available_time** | [**models::LoyaltyPromotionAvailableTimeData**](LoyaltyPromotionAvailableTimeData.md) |  | 
**trigger_limit** | Option<[**models::LoyaltyPromotionTriggerLimit**](LoyaltyPromotionTriggerLimit.md)> |  | [optional]
**status** | Option<[**models::LoyaltyPromotionStatus**](LoyaltyPromotionStatus.md)> |  | [optional]
**created_at** | Option<**String**> | The timestamp of when the promotion was created, in RFC 3339 format. | [optional][readonly]
**canceled_at** | Option<**String**> | The timestamp of when the promotion was canceled, in RFC 3339 format. | [optional][readonly]
**updated_at** | Option<**String**> | The timestamp when the promotion was last updated, in RFC 3339 format. | [optional][readonly]
**loyalty_program_id** | Option<**String**> | The ID of the [loyalty program](entity:LoyaltyProgram) associated with the promotion. | [optional][readonly]
**minimum_spend_amount_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**qualifying_item_variation_ids** | Option<**Vec<String>**> | The IDs of any qualifying `ITEM_VARIATION` [catalog objects](entity:CatalogObject). If specified, the purchase must include at least one of these items to qualify for the promotion.  This option is valid only if the base loyalty program uses a `VISIT` or `SPEND` accrual rule. With `SPEND` accrual rules, make sure that qualifying promotional items are not excluded.  You can specify `qualifying_item_variation_ids` or `qualifying_category_ids` for a given promotion, but not both. | [optional]
**qualifying_category_ids** | Option<**Vec<String>**> | The IDs of any qualifying `CATEGORY` [catalog objects](entity:CatalogObject). If specified, the purchase must include at least one item from one of these categories to qualify for the promotion.  This option is valid only if the base loyalty program uses a `VISIT` or `SPEND` accrual rule. With `SPEND` accrual rules, make sure that qualifying promotional items are not excluded.  You can specify `qualifying_category_ids` or `qualifying_item_variation_ids` for a promotion, but not both. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


