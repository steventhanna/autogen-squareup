# InventoryCount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**catalog_object_id** | Option<**String**> | The Square-generated ID of the [CatalogObject](entity:CatalogObject) being tracked. | [optional]
**catalog_object_type** | Option<**String**> | The [type](entity:CatalogObjectType) of the [CatalogObject](entity:CatalogObject) being tracked.   The Inventory API supports setting and reading the `\"catalog_object_type\": \"ITEM_VARIATION\"` field value.  In addition, it can also read the `\"catalog_object_type\": \"ITEM\"` field value that is set by the Square Restaurants app. | [optional]
**state** | Option<[**models::InventoryState**](InventoryState.md)> |  | [optional]
**location_id** | Option<**String**> | The Square-generated ID of the [Location](entity:Location) where the related quantity of items is being tracked. | [optional]
**quantity** | Option<**String**> | The number of items affected by the estimated count as a decimal string. Can support up to 5 digits after the decimal point. | [optional]
**calculated_at** | Option<**String**> | An RFC 3339-formatted timestamp that indicates when the most recent physical count or adjustment affecting the estimated count is received. | [optional][readonly]
**is_estimated** | Option<**bool**> | Whether the inventory count is for composed variation (TRUE) or not (FALSE). If true, the inventory count will not be present in the response of any of these endpoints: [BatchChangeInventory](api-endpoint:Inventory-BatchChangeInventory), [BatchRetrieveInventoryChanges](api-endpoint:Inventory-BatchRetrieveInventoryChanges), [BatchRetrieveInventoryCounts](api-endpoint:Inventory-BatchRetrieveInventoryCounts), and [RetrieveInventoryChanges](api-endpoint:Inventory-RetrieveInventoryChanges). | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


