# CatalogItemOptionValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_option_id** | Option<**String**> | Unique ID of the associated item option. | [optional]
**name** | Option<**String**> | Name of this item option value. This is a searchable attribute for use in applicable query filters. | [optional]
**description** | Option<**String**> | A human-readable description for the option value. This is a searchable attribute for use in applicable query filters. | [optional]
**color** | Option<**String**> | The HTML-supported hex color for the item option (e.g., \"#ff8d4e85\"). Only displayed if `show_colors` is enabled on the parent `ItemOption`. When left unset, `color` defaults to white (\"#ffffff\") when `show_colors` is enabled on the parent `ItemOption`. | [optional]
**ordinal** | Option<**i32**> | Determines where this option value appears in a list of option values. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


