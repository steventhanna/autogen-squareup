# CatalogItemModifierListInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**modifier_list_id** | **String** | The ID of the `CatalogModifierList` controlled by this `CatalogModifierListInfo`. | 
**modifier_overrides** | Option<[**Vec<models::CatalogModifierOverride>**](CatalogModifierOverride.md)> | A set of `CatalogModifierOverride` objects that override default modifier settings for this item. | [optional]
**min_selected_modifiers** | Option<**i32**> | The minimum number of modifiers that must be selected from this modifier list. Values:  - 0: No selection is required. - -1: Default value, the attribute was not set by the client. When `max_selected_modifiers` is also -1, use the minimum and maximum selection values set on the `CatalogItemModifierList`. - &gt;0: The required minimum modifier selections. This can be larger than the total `CatalogModifiers` when `allow_quantities` is enabled. - &lt; -1: Invalid. Treated as no selection required. | [optional]
**max_selected_modifiers** | Option<**i32**> | The maximum number of modifiers that can be selected. Values:  - 0: No maximum limit. - -1: Default value, the attribute was not set by the client. When `min_selected_modifiers` is also -1, use the minimum and maximum selection values set on the `CatalogItemModifierList`. - &gt;0: The maximum total modifier selections. This can be larger than the total `CatalogModifiers` when `allow_quantities` is enabled. - &lt; -1: Invalid. Treated as no maximum limit. | [optional]
**enabled** | Option<**bool**> | If `true`, enable this `CatalogModifierList`. The default value is `true`. | [optional]
**ordinal** | Option<**i32**> | The position of this `CatalogItemModifierListInfo` object within the `modifier_list_info` list applied  to a `CatalogItem` instance. | [optional]
**allow_quantities** | Option<[**models::CatalogModifierToggleOverrideType**](CatalogModifierToggleOverrideType.md)> |  | [optional]
**is_conversational** | Option<[**models::CatalogModifierToggleOverrideType**](CatalogModifierToggleOverrideType.md)> |  | [optional]
**hidden_from_customer_override** | Option<[**models::CatalogModifierToggleOverrideType**](CatalogModifierToggleOverrideType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


