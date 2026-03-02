# SearchOrdersResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_entries** | Option<[**Vec<models::OrderEntry>**](OrderEntry.md)> | A list of [OrderEntries](entity:OrderEntry) that fit the query conditions. The list is populated only if `return_entries` is set to `true` in the request. | [optional]
**orders** | Option<[**Vec<models::Order>**](Order.md)> | A list of [Order](entity:Order) objects that match the query conditions. The list is populated only if `return_entries` is set to `false` in the request. | [optional]
**cursor** | Option<**String**> | The pagination cursor to be used in a subsequent request. If unset, this is the final response. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | [Errors](entity:Error) encountered during the search. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


