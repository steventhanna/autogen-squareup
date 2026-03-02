# BatchRetrieveOrdersRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**location_id** | Option<**String**> | The ID of the location for these orders. This field is optional: omit it to retrieve orders within the scope of the current authorization's merchant ID. | [optional]
**order_ids** | **Vec<String>** | The IDs of the orders to retrieve. A maximum of 100 orders can be retrieved per request. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


