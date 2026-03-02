# CatalogItemOption

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The item option's display name for the seller. Must be unique across all item options. This is a searchable attribute for use in applicable query filters. | [optional]
**display_name** | Option<**String**> | The item option's display name for the customer. This is a searchable attribute for use in applicable query filters. | [optional]
**description** | Option<**String**> | The item option's human-readable description. Displayed in the Square Point of Sale app for the seller and in the Online Store or on receipts for the buyer. This is a searchable attribute for use in applicable query filters. | [optional]
**show_colors** | Option<**bool**> | If true, display colors for entries in `values` when present. | [optional]
**values** | Option<[**Vec<models::CatalogObject>**](CatalogObject.md)> | A list of CatalogObjects containing the `CatalogItemOptionValue`s for this item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


