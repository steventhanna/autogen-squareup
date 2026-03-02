# BatchRetrieveInventoryChangesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**catalog_object_ids** | Option<**Vec<String>**> | The filter to return results by `CatalogObject` ID. The filter is only applicable when set. The default value is null. | [optional]
**location_ids** | Option<**Vec<String>**> | The filter to return results by `Location` ID. The filter is only applicable when set. The default value is null. | [optional]
**types** | Option<[**Vec<models::InventoryChangeType>**](InventoryChangeType.md)> | The filter to return results by `InventoryChangeType` values other than `TRANSFER`. The default value is `[PHYSICAL_COUNT, ADJUSTMENT]`. | [optional]
**states** | Option<[**Vec<models::InventoryState>**](InventoryState.md)> | The filter to return `ADJUSTMENT` query results by `InventoryState`. This filter is only applied when set. The default value is null. | [optional]
**updated_after** | Option<**String**> | The filter to return results with their `calculated_at` value after the given time as specified in an RFC 3339 timestamp. The default value is the UNIX epoch of (`1970-01-01T00:00:00Z`). | [optional]
**updated_before** | Option<**String**> | The filter to return results with their `created_at` or `calculated_at` value strictly before the given time as specified in an RFC 3339 timestamp. The default value is the UNIX epoch of (`1970-01-01T00:00:00Z`). | [optional]
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this to retrieve the next set of results for the original query.  See the [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination) guide for more information. | [optional]
**limit** | Option<**i32**> | The number of [records](entity:InventoryChange) to return. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


