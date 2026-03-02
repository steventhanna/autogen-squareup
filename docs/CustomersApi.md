# \CustomersApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group_to_customer**](CustomersApi.md#add_group_to_customer) | **PUT** /v2/customers/{customer_id}/groups/{group_id} | AddGroupToCustomer
[**bulk_create_customers**](CustomersApi.md#bulk_create_customers) | **POST** /v2/customers/bulk-create | BulkCreateCustomers
[**bulk_delete_customers**](CustomersApi.md#bulk_delete_customers) | **POST** /v2/customers/bulk-delete | BulkDeleteCustomers
[**bulk_retrieve_customers**](CustomersApi.md#bulk_retrieve_customers) | **POST** /v2/customers/bulk-retrieve | BulkRetrieveCustomers
[**bulk_update_customers**](CustomersApi.md#bulk_update_customers) | **POST** /v2/customers/bulk-update | BulkUpdateCustomers
[**create_customer**](CustomersApi.md#create_customer) | **POST** /v2/customers | CreateCustomer
[**create_customer_card**](CustomersApi.md#create_customer_card) | **POST** /v2/customers/{customer_id}/cards | CreateCustomerCard
[**delete_customer**](CustomersApi.md#delete_customer) | **DELETE** /v2/customers/{customer_id} | DeleteCustomer
[**delete_customer_card**](CustomersApi.md#delete_customer_card) | **DELETE** /v2/customers/{customer_id}/cards/{card_id} | DeleteCustomerCard
[**list_customers**](CustomersApi.md#list_customers) | **GET** /v2/customers | ListCustomers
[**remove_group_from_customer**](CustomersApi.md#remove_group_from_customer) | **DELETE** /v2/customers/{customer_id}/groups/{group_id} | RemoveGroupFromCustomer
[**retrieve_customer**](CustomersApi.md#retrieve_customer) | **GET** /v2/customers/{customer_id} | RetrieveCustomer
[**search_customers**](CustomersApi.md#search_customers) | **POST** /v2/customers/search | SearchCustomers
[**update_customer**](CustomersApi.md#update_customer) | **PUT** /v2/customers/{customer_id} | UpdateCustomer



## add_group_to_customer

> models::AddGroupToCustomerResponse add_group_to_customer(customer_id, group_id)
AddGroupToCustomer

Adds a group membership to a customer.  The customer is identified by the `customer_id` value and the customer group is identified by the `group_id` value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The ID of the customer to add to a group. | [required] |
**group_id** | **String** | The ID of the customer group to add the customer to. | [required] |

### Return type

[**models::AddGroupToCustomerResponse**](AddGroupToCustomerResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_create_customers

> models::BulkCreateCustomersResponse bulk_create_customers(bulk_create_customers_request)
BulkCreateCustomers

Creates multiple [customer profiles](entity:Customer) for a business.  This endpoint takes a map of individual create requests and returns a map of responses.  You must provide at least one of the following values in each create request:  - `given_name` - `family_name` - `company_name` - `email_address` - `phone_number`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_create_customers_request** | [**BulkCreateCustomersRequest**](BulkCreateCustomersRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkCreateCustomersResponse**](BulkCreateCustomersResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_delete_customers

> models::BulkDeleteCustomersResponse bulk_delete_customers(bulk_delete_customers_request)
BulkDeleteCustomers

Deletes multiple customer profiles.  The endpoint takes a list of customer IDs and returns a map of responses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_delete_customers_request** | [**BulkDeleteCustomersRequest**](BulkDeleteCustomersRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkDeleteCustomersResponse**](BulkDeleteCustomersResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_retrieve_customers

> models::BulkRetrieveCustomersResponse bulk_retrieve_customers(bulk_retrieve_customers_request)
BulkRetrieveCustomers

Retrieves multiple customer profiles.  This endpoint takes a list of customer IDs and returns a map of responses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_retrieve_customers_request** | [**BulkRetrieveCustomersRequest**](BulkRetrieveCustomersRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkRetrieveCustomersResponse**](BulkRetrieveCustomersResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_update_customers

> models::BulkUpdateCustomersResponse bulk_update_customers(bulk_update_customers_request)
BulkUpdateCustomers

Updates multiple customer profiles.  This endpoint takes a map of individual update requests and returns a map of responses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_update_customers_request** | [**BulkUpdateCustomersRequest**](BulkUpdateCustomersRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkUpdateCustomersResponse**](BulkUpdateCustomersResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_customer

> models::CreateCustomerResponse create_customer(create_customer_request)
CreateCustomer

Creates a new customer for a business.  You must provide at least one of the following values in your request to this endpoint:  - `given_name` - `family_name` - `company_name` - `email_address` - `phone_number`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_customer_request** | [**CreateCustomerRequest**](CreateCustomerRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateCustomerResponse**](CreateCustomerResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_customer_card

> models::CreateCustomerCardResponse create_customer_card(customer_id, create_customer_card_request)
CreateCustomerCard

Adds a card on file to an existing customer.  As with charges, calls to `CreateCustomerCard` are idempotent. Multiple calls with the same card nonce return the same card record that was created with the provided nonce during the _first_ call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The Square ID of the customer profile the card is linked to. | [required] |
**create_customer_card_request** | [**CreateCustomerCardRequest**](CreateCustomerCardRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateCustomerCardResponse**](CreateCustomerCardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_customer

> models::DeleteCustomerResponse delete_customer(customer_id, version)
DeleteCustomer

Deletes a customer profile from a business.  To delete a customer profile that was created by merging existing profiles, you must use the ID of the newly created profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The ID of the customer to delete. | [required] |
**version** | Option<**i64**> | The current version of the customer profile.  As a best practice, you should include this parameter to enable [optimistic concurrency](https://developer.squareup.com/docs/build-basics/common-api-patterns/optimistic-concurrency) control.  For more information, see [Delete a customer profile](https://developer.squareup.com/docs/customers-api/use-the-api/keep-records#delete-customer-profile). |  |

### Return type

[**models::DeleteCustomerResponse**](DeleteCustomerResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_customer_card

> models::DeleteCustomerCardResponse delete_customer_card(customer_id, card_id)
DeleteCustomerCard

Removes a card on file from a customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The ID of the customer that the card on file belongs to. | [required] |
**card_id** | **String** | The ID of the card on file to delete. | [required] |

### Return type

[**models::DeleteCustomerCardResponse**](DeleteCustomerCardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_customers

> models::ListCustomersResponse list_customers(cursor, limit, sort_field, sort_order, count)
ListCustomers

Lists customer profiles associated with a Square account.  Under normal operating conditions, newly created or updated customer profiles become available for the listing operation in well under 30 seconds. Occasionally, propagation of the new or updated profiles can take closer to one minute or longer, especially during network incidents and outages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this cursor to retrieve the next set of results for your original query.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**limit** | Option<**i32**> | The maximum number of results to return in a single page. This limit is advisory. The response might contain more or fewer results. If the specified limit is less than 1 or greater than 100, Square returns a `400 VALUE_TOO_LOW` or `400 VALUE_TOO_HIGH` error. The default value is 100.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**sort_field** | Option<[**CustomerSortField**](.md)> | Indicates how customers should be sorted.  The default value is `DEFAULT`. |  |
**sort_order** | Option<[**SortOrder**](.md)> | Indicates whether customers should be sorted in ascending (`ASC`) or descending (`DESC`) order.  The default value is `ASC`. |  |
**count** | Option<**bool**> | Indicates whether to return the total count of customers in the `count` field of the response.  The default value is `false`. |  |[default to false]

### Return type

[**models::ListCustomersResponse**](ListCustomersResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_group_from_customer

> models::RemoveGroupFromCustomerResponse remove_group_from_customer(customer_id, group_id)
RemoveGroupFromCustomer

Removes a group membership from a customer.  The customer is identified by the `customer_id` value and the customer group is identified by the `group_id` value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The ID of the customer to remove from the group. | [required] |
**group_id** | **String** | The ID of the customer group to remove the customer from. | [required] |

### Return type

[**models::RemoveGroupFromCustomerResponse**](RemoveGroupFromCustomerResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_customer

> models::RetrieveCustomerResponse retrieve_customer(customer_id)
RetrieveCustomer

Returns details for a single customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The ID of the customer to retrieve. | [required] |

### Return type

[**models::RetrieveCustomerResponse**](RetrieveCustomerResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_customers

> models::SearchCustomersResponse search_customers(search_customers_request)
SearchCustomers

Searches the customer profiles associated with a Square account using one or more supported query filters.  Calling `SearchCustomers` without any explicit query filter returns all customer profiles ordered alphabetically based on `given_name` and `family_name`.  Under normal operating conditions, newly created or updated customer profiles become available for the search operation in well under 30 seconds. Occasionally, propagation of the new or updated profiles can take closer to one minute or longer, especially during network incidents and outages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_customers_request** | [**SearchCustomersRequest**](SearchCustomersRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchCustomersResponse**](SearchCustomersResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_customer

> models::UpdateCustomerResponse update_customer(customer_id, update_customer_request)
UpdateCustomer

Updates a customer profile. This endpoint supports sparse updates, so only new or changed fields are required in the request. To add or update a field, specify the new value. To remove a field, specify `null`.  To update a customer profile that was created by merging existing profiles, you must use the ID of the newly created profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The ID of the customer to update. | [required] |
**update_customer_request** | [**UpdateCustomerRequest**](UpdateCustomerRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateCustomerResponse**](UpdateCustomerResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

