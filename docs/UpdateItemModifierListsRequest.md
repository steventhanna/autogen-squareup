# UpdateItemModifierListsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_ids** | **Vec<String>** | The IDs of the catalog items associated with the CatalogModifierList objects being updated. | 
**modifier_lists_to_enable** | Option<**Vec<String>**> | The IDs of the CatalogModifierList objects to enable for the CatalogItem. At least one of `modifier_lists_to_enable` or `modifier_lists_to_disable` must be specified. | [optional]
**modifier_lists_to_disable** | Option<**Vec<String>**> | The IDs of the CatalogModifierList objects to disable for the CatalogItem. At least one of `modifier_lists_to_enable` or `modifier_lists_to_disable` must be specified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


