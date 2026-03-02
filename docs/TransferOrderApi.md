# \TransferOrderApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_transfer_order**](TransferOrderApi.md#cancel_transfer_order) | **POST** /v2/transfer-orders/{transfer_order_id}/cancel | CancelTransferOrder
[**create_transfer_order**](TransferOrderApi.md#create_transfer_order) | **POST** /v2/transfer-orders | CreateTransferOrder
[**delete_transfer_order**](TransferOrderApi.md#delete_transfer_order) | **DELETE** /v2/transfer-orders/{transfer_order_id} | DeleteTransferOrder
[**receive_transfer_order**](TransferOrderApi.md#receive_transfer_order) | **POST** /v2/transfer-orders/{transfer_order_id}/receive | ReceiveTransferOrder
[**retrieve_transfer_order**](TransferOrderApi.md#retrieve_transfer_order) | **GET** /v2/transfer-orders/{transfer_order_id} | RetrieveTransferOrder
[**search_transfer_orders**](TransferOrderApi.md#search_transfer_orders) | **POST** /v2/transfer-orders/search | SearchTransferOrders
[**start_transfer_order**](TransferOrderApi.md#start_transfer_order) | **POST** /v2/transfer-orders/{transfer_order_id}/start | StartTransferOrder
[**update_transfer_order**](TransferOrderApi.md#update_transfer_order) | **PUT** /v2/transfer-orders/{transfer_order_id} | UpdateTransferOrder



## cancel_transfer_order

> models::CancelTransferOrderResponse cancel_transfer_order(transfer_order_id, cancel_transfer_order_request)
CancelTransferOrder

Cancels a transfer order in [STARTED](entity:TransferOrderStatus) or  [PARTIALLY_RECEIVED](entity:TransferOrderStatus) status. Any unreceived quantities will no longer be receivable and will be immediately returned to the source [Location](entity:Location)'s inventory.  Common reasons for cancellation: - Items no longer needed at destination - Source location needs the inventory - Order created in error  Creates a [transfer_order.updated](webhook:transfer_order.updated) webhook event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_order_id** | **String** | The ID of the transfer order to cancel. Must be in STARTED or PARTIALLY_RECEIVED status. | [required] |
**cancel_transfer_order_request** | [**CancelTransferOrderRequest**](CancelTransferOrderRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CancelTransferOrderResponse**](CancelTransferOrderResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_transfer_order

> models::CreateTransferOrderResponse create_transfer_order(create_transfer_order_request)
CreateTransferOrder

Creates a new transfer order in [DRAFT](entity:TransferOrderStatus) status. A transfer order represents the intent  to move [CatalogItemVariation](entity:CatalogItemVariation)s from one [Location](entity:Location) to another.  The source and destination locations must be different and must belong to your Square account.  In [DRAFT](entity:TransferOrderStatus) status, you can: - Add or remove items - Modify quantities - Update shipping information - Delete the entire order via [DeleteTransferOrder](api-endpoint:TransferOrders-DeleteTransferOrder)  The request requires source_location_id and destination_location_id. Inventory levels are not affected until the order is started via  [StartTransferOrder](api-endpoint:TransferOrders-StartTransferOrder).  Common integration points: - Sync with warehouse management systems - Automate regular stock transfers - Initialize transfers from inventory optimization systems  Creates a [transfer_order.created](webhook:transfer_order.created) webhook event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_transfer_order_request** | [**CreateTransferOrderRequest**](CreateTransferOrderRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateTransferOrderResponse**](CreateTransferOrderResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_transfer_order

> models::DeleteTransferOrderResponse delete_transfer_order(transfer_order_id, version)
DeleteTransferOrder

Deletes a transfer order in [DRAFT](entity:TransferOrderStatus) status. Only draft orders can be deleted. Once an order is started via  [StartTransferOrder](api-endpoint:TransferOrders-StartTransferOrder), it can no longer be deleted.  Creates a [transfer_order.deleted](webhook:transfer_order.deleted) webhook event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_order_id** | **String** | The ID of the transfer order to delete | [required] |
**version** | Option<**i64**> | Version for optimistic concurrency |  |

### Return type

[**models::DeleteTransferOrderResponse**](DeleteTransferOrderResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## receive_transfer_order

> models::ReceiveTransferOrderResponse receive_transfer_order(transfer_order_id, receive_transfer_order_request)
ReceiveTransferOrder

Records receipt of [CatalogItemVariation](entity:CatalogItemVariation)s for a transfer order. This endpoint supports partial receiving - you can receive items in multiple batches.  For each line item, you can specify: - Quantity received in good condition (added to destination inventory with [InventoryState](entity:InventoryState) of IN_STOCK) - Quantity damaged during transit/handling (added to destination inventory with [InventoryState](entity:InventoryState) of WASTE) - Quantity canceled (returned to source location's inventory)  The order must be in [STARTED](entity:TransferOrderStatus) or [PARTIALLY_RECEIVED](entity:TransferOrderStatus) status. Received quantities are added to the destination [Location](entity:Location)'s inventory according to their condition. Canceled quantities are immediately returned to the source [Location](entity:Location)'s inventory.  When all items are either received, damaged, or canceled, the order moves to [COMPLETED](entity:TransferOrderStatus) status.  Creates a [transfer_order.updated](webhook:transfer_order.updated) webhook event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_order_id** | **String** | The ID of the transfer order to receive items for | [required] |
**receive_transfer_order_request** | [**ReceiveTransferOrderRequest**](ReceiveTransferOrderRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::ReceiveTransferOrderResponse**](ReceiveTransferOrderResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_transfer_order

> models::RetrieveTransferOrderResponse retrieve_transfer_order(transfer_order_id)
RetrieveTransferOrder

Retrieves a specific [TransferOrder](entity:TransferOrder) by ID. Returns the complete order details including:  - Basic information (status, dates, notes) - Line items with ordered and received quantities - Source and destination [Location](entity:Location)s - Tracking information (if available)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_order_id** | **String** | The ID of the transfer order to retrieve | [required] |

### Return type

[**models::RetrieveTransferOrderResponse**](RetrieveTransferOrderResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_transfer_orders

> models::SearchTransferOrdersResponse search_transfer_orders(search_transfer_orders_request)
SearchTransferOrders

Searches for transfer orders using filters. Returns a paginated list of matching [TransferOrder](entity:TransferOrder)s sorted by creation date.  Common search scenarios: - Find orders for a source [Location](entity:Location) - Find orders for a destination [Location](entity:Location) - Find orders in a particular [TransferOrderStatus](entity:TransferOrderStatus)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_transfer_orders_request** | [**SearchTransferOrdersRequest**](SearchTransferOrdersRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchTransferOrdersResponse**](SearchTransferOrdersResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_transfer_order

> models::StartTransferOrderResponse start_transfer_order(transfer_order_id, start_transfer_order_request)
StartTransferOrder

Changes a [DRAFT](entity:TransferOrderStatus) transfer order to [STARTED](entity:TransferOrderStatus) status. This decrements inventory at the source [Location](entity:Location) and marks it as in-transit.  The order must be in [DRAFT](entity:TransferOrderStatus) status and have all required fields populated. Once started, the order can no longer be deleted, but it can be canceled via  [CancelTransferOrder](api-endpoint:TransferOrders-CancelTransferOrder).  Creates a [transfer_order.updated](webhook:transfer_order.updated) webhook event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_order_id** | **String** | The ID of the transfer order to start. Must be in DRAFT status. | [required] |
**start_transfer_order_request** | [**StartTransferOrderRequest**](StartTransferOrderRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::StartTransferOrderResponse**](StartTransferOrderResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_transfer_order

> models::UpdateTransferOrderResponse update_transfer_order(transfer_order_id, update_transfer_order_request)
UpdateTransferOrder

Updates an existing transfer order. This endpoint supports sparse updates, allowing you to modify specific fields without affecting others.  Creates a [transfer_order.updated](webhook:transfer_order.updated) webhook event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_order_id** | **String** | The ID of the transfer order to update | [required] |
**update_transfer_order_request** | [**UpdateTransferOrderRequest**](UpdateTransferOrderRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateTransferOrderResponse**](UpdateTransferOrderResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

