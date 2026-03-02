# CatalogModifierList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the `CatalogModifierList` instance. This is a searchable attribute for use in applicable query filters, and its value length is of  Unicode code points. | [optional]
**ordinal** | Option<**i32**> | The position of this `CatalogModifierList` within a list of `CatalogModifierList` instances. | [optional]
**selection_type** | Option<[**models::CatalogModifierListSelectionType**](CatalogModifierListSelectionType.md)> |  | [optional]
**modifiers** | Option<[**Vec<models::CatalogObject>**](CatalogObject.md)> | A non-empty list of `CatalogModifier` objects to be included in the `CatalogModifierList`,  for non text-based modifiers when the `modifier_type` attribute is `LIST`. Each element of this list  is a `CatalogObject` instance of the `MODIFIER` type, containing the following attributes: ``` { \"id\": \"{{catalog_modifier_id}}\", \"type\": \"MODIFIER\",  \"modifier_data\": {{a CatalogModifier instance>}}  } ``` | [optional]
**image_ids** | Option<**Vec<String>**> | The IDs of images associated with this `CatalogModifierList` instance. Currently these images are not displayed on Square products, but may be displayed in 3rd-party applications. | [optional]
**allow_quantities** | Option<**bool**> | When `true`, allows multiple quantities of the same modifier to be selected. | [optional]
**is_conversational** | Option<**bool**> | True if modifiers belonging to this list can be used conversationally. | [optional]
**modifier_type** | Option<[**models::CatalogModifierListModifierType**](CatalogModifierListModifierType.md)> |  | [optional]
**max_length** | Option<**i32**> | The maximum length, in Unicode points, of the text string of the text-based modifier as represented by  this `CatalogModifierList` object with the `modifier_type` set to `TEXT`. | [optional]
**text_required** | Option<**bool**> | Whether the text string must be a non-empty string (`true`) or not (`false`) for a text-based modifier as represented by this `CatalogModifierList` object with the `modifier_type` set to `TEXT`. | [optional]
**internal_name** | Option<**String**> | A note for internal use by the business.     For example, for a text-based modifier applied to a T-shirt item, if the buyer-supplied text of \"Hello, Kitty!\"   is to be printed on the T-shirt, this `internal_name` attribute can be \"Use italic face\" as  an instruction for the business to follow.    For non text-based modifiers, this `internal_name` attribute can be  used to include SKUs, internal codes, or supplemental descriptions for internal use. | [optional]
**min_selected_modifiers** | Option<**i64**> | The minimum number of modifiers that must be selected from this list. The value can be overridden with `CatalogItemModifierListInfo`.  Values:  - 0: No selection is required. - -1: Default value, the attribute was not set by the client. Treated as no selection required. - &gt;0: The required minimum modifier selections. This can be larger than the total `CatalogModifiers` when `allow_quantities` is enabled. - &lt; -1: Invalid. Treated as no selection required. | [optional]
**max_selected_modifiers** | Option<**i64**> | The maximum number of modifiers that must be selected from this list. The value can be overridden with `CatalogItemModifierListInfo`.  Values:  - 0: No maximum limit. - -1: Default value, the attribute was not set by the client. Treated as no maximum limit. - &gt;0: The maximum total modifier selections. This can be larger than the total `CatalogModifiers` when `allow_quantities` is enabled. - &lt; -1: Invalid. Treated as no maximum limit. | [optional]
**hidden_from_customer** | Option<**bool**> | If `true`, modifiers from this list are hidden from customer receipts. The default value is `false`. This setting can be overridden with `CatalogItemModifierListInfo.hidden_from_customer_override`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


