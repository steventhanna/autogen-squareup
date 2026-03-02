# InventoryPhysicalCount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A unique Square-generated ID for the [InventoryPhysicalCount](entity:InventoryPhysicalCount). | [optional]
**reference_id** | Option<**String**> | An optional ID provided by the application to tie the [InventoryPhysicalCount](entity:InventoryPhysicalCount) to an external system. | [optional]
**catalog_object_id** | Option<**String**> | The Square-generated ID of the [CatalogObject](entity:CatalogObject) being tracked. | [optional]
**catalog_object_type** | Option<**String**> | The [type](entity:CatalogObjectType) of the [CatalogObject](entity:CatalogObject) being tracked.   The Inventory API supports setting and reading the `\"catalog_object_type\": \"ITEM_VARIATION\"` field value.  In addition, it can also read the `\"catalog_object_type\": \"ITEM\"` field value that is set by the Square Restaurants app. | [optional]
**state** | Option<[**models::InventoryState**](InventoryState.md)> |  | [optional]
**location_id** | Option<**String**> | The Square-generated ID of the [Location](entity:Location) where the related quantity of items is being tracked. | [optional]
**quantity** | Option<**String**> | The number of items affected by the physical count as a decimal string. The number can support up to 5 digits after the decimal point. | [optional]
**source** | Option<[**models::SourceApplication**](SourceApplication.md)> |  | [optional]
**employee_id** | Option<**String**> | The Square-generated ID of the [Employee](entity:Employee) responsible for the physical count. | [optional]
**team_member_id** | Option<**String**> | The Square-generated ID of the [Team Member](entity:TeamMember) responsible for the physical count. | [optional]
**occurred_at** | Option<**String**> | A client-generated RFC 3339-formatted timestamp that indicates when the physical count was examined. For physical count updates, the `occurred_at` timestamp cannot be older than 24 hours or in the future relative to the time of the request. | [optional]
**created_at** | Option<**String**> | An RFC 3339-formatted timestamp that indicates when the physical count is received. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


