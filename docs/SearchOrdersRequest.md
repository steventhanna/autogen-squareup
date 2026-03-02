# SearchOrdersRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**location_ids** | Option<**Vec<String>**> | The location IDs for the orders to query. All locations must belong to the same merchant.  Max: 10 location IDs. | [optional]
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this cursor to retrieve the next set of results for your original query. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). | [optional]
**query** | Option<[**models::SearchOrdersQuery**](SearchOrdersQuery.md)> |  | [optional]
**limit** | Option<**i32**> | The maximum number of results to be returned in a single page.  Default: `500` Max: `1000` | [optional]
**return_entries** | Option<**bool**> | A Boolean that controls the format of the search results. If `true`, `SearchOrders` returns [OrderEntry](entity:OrderEntry) objects. If `false`, `SearchOrders` returns complete order objects.  Default: `false`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


