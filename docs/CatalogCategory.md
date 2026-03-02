# CatalogCategory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The category name. This is a searchable attribute for use in applicable query filters, and its value length is of Unicode code points. | [optional]
**image_ids** | Option<**Vec<String>**> | The IDs of images associated with this `CatalogCategory` instance. Currently these images are not displayed by Square, but are free to be displayed in 3rd party applications. | [optional]
**category_type** | Option<[**models::CatalogCategoryType**](CatalogCategoryType.md)> |  | [optional]
**parent_category** | Option<[**models::CatalogObjectCategory**](CatalogObjectCategory.md)> |  | [optional]
**is_top_level** | Option<**bool**> | Indicates whether a category is a top level category, which does not have any parent_category. | [optional]
**channels** | Option<**Vec<String>**> | A list of IDs representing channels, such as a Square Online site, where the category can be made visible. | [optional]
**availability_period_ids** | Option<**Vec<String>**> | The IDs of the `CatalogAvailabilityPeriod` objects associated with the category. | [optional]
**online_visibility** | Option<**bool**> | Indicates whether the category is visible (`true`) or hidden (`false`) on all of the seller's Square Online sites. | [optional]
**root_category** | Option<**String**> | The top-level category in a category hierarchy. | [optional][readonly]
**ecom_seo_data** | Option<[**models::CatalogEcomSeoData**](CatalogEcomSeoData.md)> |  | [optional]
**path_to_root** | Option<[**Vec<models::CategoryPathToRootNode>**](CategoryPathToRootNode.md)> | The path from the category to its root category. The first node of the path is the parent of the category and the last is the root category. The path is empty if the category is a root category. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


