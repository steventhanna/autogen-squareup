# \V1TransactionsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_list_orders**](V1TransactionsApi.md#v1_list_orders) | **GET** /v1/{location_id}/orders | V1ListOrders
[**v1_retrieve_order**](V1TransactionsApi.md#v1_retrieve_order) | **GET** /v1/{location_id}/orders/{order_id} | V1RetrieveOrder
[**v1_update_order**](V1TransactionsApi.md#v1_update_order) | **PUT** /v1/{location_id}/orders/{order_id} | V1UpdateOrder



## v1_list_orders

> Vec<models::V1Order> v1_list_orders(location_id, order, limit, batch_token)
V1ListOrders

Provides summary information for a merchant's online store orders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the location to list online store orders for. | [required] |
**order** | Option<[**SortOrder**](.md)> | The order in which payments are listed in the response. |  |
**limit** | Option<**i32**> | The maximum number of payments to return in a single response. This value cannot exceed 200. |  |
**batch_token** | Option<**String**> | A pagination cursor to retrieve the next set of results for your original query to the endpoint. |  |

### Return type

[**Vec<models::V1Order>**](V1Order.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_retrieve_order

> models::V1Order v1_retrieve_order(location_id, order_id)
V1RetrieveOrder

Provides comprehensive information for a single online store order, including the order's history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the order's associated location. | [required] |
**order_id** | **String** | The order's Square-issued ID. You obtain this value from Order objects returned by the List Orders endpoint | [required] |

### Return type

[**models::V1Order**](V1Order.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_update_order

> models::V1Order v1_update_order(location_id, order_id, v1_update_order_request)
V1UpdateOrder

Updates the details of an online store order. Every update you perform on an order corresponds to one of three actions:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the order's associated location. | [required] |
**order_id** | **String** | The order's Square-issued ID. You obtain this value from Order objects returned by the List Orders endpoint | [required] |
**v1_update_order_request** | [**V1UpdateOrderRequest**](V1UpdateOrderRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::V1Order**](V1Order.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

