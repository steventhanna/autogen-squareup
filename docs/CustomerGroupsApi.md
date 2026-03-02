# \CustomerGroupsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_customer_group**](CustomerGroupsApi.md#create_customer_group) | **POST** /v2/customers/groups | CreateCustomerGroup
[**delete_customer_group**](CustomerGroupsApi.md#delete_customer_group) | **DELETE** /v2/customers/groups/{group_id} | DeleteCustomerGroup
[**list_customer_groups**](CustomerGroupsApi.md#list_customer_groups) | **GET** /v2/customers/groups | ListCustomerGroups
[**retrieve_customer_group**](CustomerGroupsApi.md#retrieve_customer_group) | **GET** /v2/customers/groups/{group_id} | RetrieveCustomerGroup
[**update_customer_group**](CustomerGroupsApi.md#update_customer_group) | **PUT** /v2/customers/groups/{group_id} | UpdateCustomerGroup



## create_customer_group

> models::CreateCustomerGroupResponse create_customer_group(create_customer_group_request)
CreateCustomerGroup

Creates a new customer group for a business.  The request must include the `name` value of the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_customer_group_request** | [**CreateCustomerGroupRequest**](CreateCustomerGroupRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateCustomerGroupResponse**](CreateCustomerGroupResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_customer_group

> models::DeleteCustomerGroupResponse delete_customer_group(group_id)
DeleteCustomerGroup

Deletes a customer group as identified by the `group_id` value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the customer group to delete. | [required] |

### Return type

[**models::DeleteCustomerGroupResponse**](DeleteCustomerGroupResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_customer_groups

> models::ListCustomerGroupsResponse list_customer_groups(cursor, limit)
ListCustomerGroups

Retrieves the list of customer groups of a business.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this cursor to retrieve the next set of results for your original query.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**limit** | Option<**i32**> | The maximum number of results to return in a single page. This limit is advisory. The response might contain more or fewer results. If the limit is less than 1 or greater than 50, Square returns a `400 VALUE_TOO_LOW` or `400 VALUE_TOO_HIGH` error. The default value is 50.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |

### Return type

[**models::ListCustomerGroupsResponse**](ListCustomerGroupsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_customer_group

> models::RetrieveCustomerGroupResponse retrieve_customer_group(group_id)
RetrieveCustomerGroup

Retrieves a specific customer group as identified by the `group_id` value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the customer group to retrieve. | [required] |

### Return type

[**models::RetrieveCustomerGroupResponse**](RetrieveCustomerGroupResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_customer_group

> models::UpdateCustomerGroupResponse update_customer_group(group_id, update_customer_group_request)
UpdateCustomerGroup

Updates a customer group as identified by the `group_id` value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the customer group to update. | [required] |
**update_customer_group_request** | [**UpdateCustomerGroupRequest**](UpdateCustomerGroupRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateCustomerGroupResponse**](UpdateCustomerGroupResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

