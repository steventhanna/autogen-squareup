# CatalogModifier

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The modifier name.  This is a searchable attribute for use in applicable query filters, and its value length is of Unicode code points. | [optional]
**price_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**on_by_default** | Option<**bool**> | When `true`, this modifier is selected by default when displaying the modifier list. This setting can be overridden at the item level using `CatalogModifierListInfo.modifier_overrides`. | [optional]
**ordinal** | Option<**i32**> | Determines where this `CatalogModifier` appears in the `CatalogModifierList`. | [optional]
**modifier_list_id** | Option<**String**> | The ID of the `CatalogModifierList` associated with this modifier. | [optional]
**location_overrides** | Option<[**Vec<models::ModifierLocationOverrides>**](ModifierLocationOverrides.md)> | Location-specific price overrides. | [optional]
**image_id** | Option<**String**> | The ID of the image associated with this `CatalogModifier` instance. Currently this image is not displayed by Square, but is free to be displayed in 3rd party applications. | [optional]
**hidden_online** | Option<**bool**> | When `true`, this modifier is hidden from online ordering channels. This setting can be overridden at the item level using `CatalogModifierListInfo.modifier_overrides`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


