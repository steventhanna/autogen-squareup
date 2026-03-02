# LoyaltyProgramAccrualRuleSpendData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount_money** | [**models::Money**](Money.md) |  | 
**excluded_category_ids** | Option<**Vec<String>**> | The IDs of any `CATEGORY` catalog objects that are excluded from points accrual.  You can use the [BatchRetrieveCatalogObjects](api-endpoint:Catalog-BatchRetrieveCatalogObjects) endpoint to retrieve information about the excluded categories. | [optional]
**excluded_item_variation_ids** | Option<**Vec<String>**> | The IDs of any `ITEM_VARIATION` catalog objects that are excluded from points accrual.  You can use the [BatchRetrieveCatalogObjects](api-endpoint:Catalog-BatchRetrieveCatalogObjects) endpoint to retrieve information about the excluded item variations. | [optional]
**tax_mode** | [**models::LoyaltyProgramAccrualRuleTaxMode**](LoyaltyProgramAccrualRuleTaxMode.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


