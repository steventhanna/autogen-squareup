# CatalogQuerySortedAttribute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attribute_name** | **String** | The attribute whose value is used as the sort key. | 
**initial_attribute_value** | Option<**String**> | The first attribute value to be returned by the query. Ascending sorts will return only objects with this value or greater, while descending sorts will return only objects with this value or less. If unset, start at the beginning (for ascending sorts) or end (for descending sorts). | [optional]
**sort_order** | Option<[**models::SortOrder**](SortOrder.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


