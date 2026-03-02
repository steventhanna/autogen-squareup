# CatalogQuickAmount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::CatalogQuickAmountType**](CatalogQuickAmountType.md) |  | 
**amount** | [**models::Money**](Money.md) |  | 
**score** | Option<**i64**> | Describes the ranking of the Quick Amount provided by machine learning model, in the range [0, 100]. MANUAL type amount will always have score = 100. | [optional]
**ordinal** | Option<**i64**> | The order in which this Quick Amount should be displayed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


