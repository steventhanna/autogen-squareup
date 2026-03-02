# \OrderCustomAttributesApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_delete_order_custom_attributes**](OrderCustomAttributesApi.md#bulk_delete_order_custom_attributes) | **POST** /v2/orders/custom-attributes/bulk-delete | BulkDeleteOrderCustomAttributes
[**bulk_upsert_order_custom_attributes**](OrderCustomAttributesApi.md#bulk_upsert_order_custom_attributes) | **POST** /v2/orders/custom-attributes/bulk-upsert | BulkUpsertOrderCustomAttributes
[**create_order_custom_attribute_definition**](OrderCustomAttributesApi.md#create_order_custom_attribute_definition) | **POST** /v2/orders/custom-attribute-definitions | CreateOrderCustomAttributeDefinition
[**delete_order_custom_attribute**](OrderCustomAttributesApi.md#delete_order_custom_attribute) | **DELETE** /v2/orders/{order_id}/custom-attributes/{custom_attribute_key} | DeleteOrderCustomAttribute
[**delete_order_custom_attribute_definition**](OrderCustomAttributesApi.md#delete_order_custom_attribute_definition) | **DELETE** /v2/orders/custom-attribute-definitions/{key} | DeleteOrderCustomAttributeDefinition
[**list_order_custom_attribute_definitions**](OrderCustomAttributesApi.md#list_order_custom_attribute_definitions) | **GET** /v2/orders/custom-attribute-definitions | ListOrderCustomAttributeDefinitions
[**list_order_custom_attributes**](OrderCustomAttributesApi.md#list_order_custom_attributes) | **GET** /v2/orders/{order_id}/custom-attributes | ListOrderCustomAttributes
[**retrieve_order_custom_attribute**](OrderCustomAttributesApi.md#retrieve_order_custom_attribute) | **GET** /v2/orders/{order_id}/custom-attributes/{custom_attribute_key} | RetrieveOrderCustomAttribute
[**retrieve_order_custom_attribute_definition**](OrderCustomAttributesApi.md#retrieve_order_custom_attribute_definition) | **GET** /v2/orders/custom-attribute-definitions/{key} | RetrieveOrderCustomAttributeDefinition
[**update_order_custom_attribute_definition**](OrderCustomAttributesApi.md#update_order_custom_attribute_definition) | **PUT** /v2/orders/custom-attribute-definitions/{key} | UpdateOrderCustomAttributeDefinition
[**upsert_order_custom_attribute**](OrderCustomAttributesApi.md#upsert_order_custom_attribute) | **POST** /v2/orders/{order_id}/custom-attributes/{custom_attribute_key} | UpsertOrderCustomAttribute



## bulk_delete_order_custom_attributes

> models::BulkDeleteOrderCustomAttributesResponse bulk_delete_order_custom_attributes(bulk_delete_order_custom_attributes_request)
BulkDeleteOrderCustomAttributes

Deletes order [custom attributes](entity:CustomAttribute) as a bulk operation.  Use this endpoint to delete one or more custom attributes from one or more orders. A custom attribute is based on a custom attribute definition in a Square seller account.  (To create a custom attribute definition, use the [CreateOrderCustomAttributeDefinition](api-endpoint:OrderCustomAttributes-CreateOrderCustomAttributeDefinition) endpoint.)  This `BulkDeleteOrderCustomAttributes` endpoint accepts a map of 1 to 25 individual delete requests and returns a map of individual delete responses. Each delete request has a unique ID and provides an order ID and custom attribute. Each delete response is returned with the ID of the corresponding request.  To delete a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes (also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_delete_order_custom_attributes_request** | [**BulkDeleteOrderCustomAttributesRequest**](BulkDeleteOrderCustomAttributesRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkDeleteOrderCustomAttributesResponse**](BulkDeleteOrderCustomAttributesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_upsert_order_custom_attributes

> models::BulkUpsertOrderCustomAttributesResponse bulk_upsert_order_custom_attributes(bulk_upsert_order_custom_attributes_request)
BulkUpsertOrderCustomAttributes

Creates or updates order [custom attributes](entity:CustomAttribute) as a bulk operation.  Use this endpoint to delete one or more custom attributes from one or more orders. A custom attribute is based on a custom attribute definition in a Square seller account.  (To create a custom attribute definition, use the [CreateOrderCustomAttributeDefinition](api-endpoint:OrderCustomAttributes-CreateOrderCustomAttributeDefinition) endpoint.)  This `BulkUpsertOrderCustomAttributes` endpoint accepts a map of 1 to 25 individual upsert requests and returns a map of individual upsert responses. Each upsert request has a unique ID and provides an order ID and custom attribute. Each upsert response is returned with the ID of the corresponding request.  To create or update a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes (also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_upsert_order_custom_attributes_request** | [**BulkUpsertOrderCustomAttributesRequest**](BulkUpsertOrderCustomAttributesRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkUpsertOrderCustomAttributesResponse**](BulkUpsertOrderCustomAttributesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_custom_attribute_definition

> models::CreateOrderCustomAttributeDefinitionResponse create_order_custom_attribute_definition(create_order_custom_attribute_definition_request)
CreateOrderCustomAttributeDefinition

Creates an order-related custom attribute definition.  Use this endpoint to define a custom attribute that can be associated with orders.  After creating a custom attribute definition, you can set the custom attribute for orders in the Square seller account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_order_custom_attribute_definition_request** | [**CreateOrderCustomAttributeDefinitionRequest**](CreateOrderCustomAttributeDefinitionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateOrderCustomAttributeDefinitionResponse**](CreateOrderCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_order_custom_attribute

> models::DeleteOrderCustomAttributeResponse delete_order_custom_attribute(order_id, custom_attribute_key)
DeleteOrderCustomAttribute

Deletes a [custom attribute](entity:CustomAttribute) associated with a customer profile.  To delete a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes (also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | The ID of the target [order](entity:Order). | [required] |
**custom_attribute_key** | **String** | The key of the custom attribute to delete.  This key must match the key of an existing custom attribute definition. | [required] |

### Return type

[**models::DeleteOrderCustomAttributeResponse**](DeleteOrderCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_order_custom_attribute_definition

> models::DeleteOrderCustomAttributeDefinitionResponse delete_order_custom_attribute_definition(key)
DeleteOrderCustomAttributeDefinition

Deletes an order-related [custom attribute definition](entity:CustomAttributeDefinition) from a Square seller account.  Only the definition owner can delete a custom attribute definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to delete. | [required] |

### Return type

[**models::DeleteOrderCustomAttributeDefinitionResponse**](DeleteOrderCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_order_custom_attribute_definitions

> models::ListOrderCustomAttributeDefinitionsResponse list_order_custom_attribute_definitions(visibility_filter, cursor, limit)
ListOrderCustomAttributeDefinitions

Lists the order-related [custom attribute definitions](entity:CustomAttributeDefinition) that belong to a Square seller account.  When all response pages are retrieved, the results include all custom attribute definitions that are visible to the requesting application, including those that are created by other applications and set to `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes (also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**visibility_filter** | Option<[**VisibilityFilter**](.md)> | Requests that all of the custom attributes be returned, or only those that are read-only or read-write. |  |
**cursor** | Option<**String**> | The cursor returned in the paged response from the previous call to this endpoint.  Provide this cursor to retrieve the next page of results for your original request.  For more information, see [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination). |  |
**limit** | Option<**i32**> | The maximum number of results to return in a single paged response. This limit is advisory.  The response might contain more or fewer results. The minimum value is 1 and the maximum value is 100.  The default value is 20. For more information, see [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination). |  |

### Return type

[**models::ListOrderCustomAttributeDefinitionsResponse**](ListOrderCustomAttributeDefinitionsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_order_custom_attributes

> models::ListOrderCustomAttributesResponse list_order_custom_attributes(order_id, visibility_filter, cursor, limit, with_definitions)
ListOrderCustomAttributes

Lists the [custom attributes](entity:CustomAttribute) associated with an order.  You can use the `with_definitions` query parameter to also retrieve custom attribute definitions in the same call.  When all response pages are retrieved, the results include all custom attributes that are visible to the requesting application, including those that are owned by other applications and set to `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | The ID of the target [order](entity:Order). | [required] |
**visibility_filter** | Option<[**VisibilityFilter**](.md)> | Requests that all of the custom attributes be returned, or only those that are read-only or read-write. |  |
**cursor** | Option<**String**> | The cursor returned in the paged response from the previous call to this endpoint.  Provide this cursor to retrieve the next page of results for your original request.  For more information, see [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination). |  |
**limit** | Option<**i32**> | The maximum number of results to return in a single paged response. This limit is advisory.  The response might contain more or fewer results. The minimum value is 1 and the maximum value is 100.  The default value is 20. For more information, see [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination). |  |
**with_definitions** | Option<**bool**> | Indicates whether to return the [custom attribute definition](entity:CustomAttributeDefinition) in the `definition` field of each custom attribute. Set this parameter to `true` to get the name and description of each custom attribute,  information about the data type, or other definition details. The default value is `false`. |  |[default to false]

### Return type

[**models::ListOrderCustomAttributesResponse**](ListOrderCustomAttributesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_order_custom_attribute

> models::RetrieveOrderCustomAttributeResponse retrieve_order_custom_attribute(order_id, custom_attribute_key, version, with_definition)
RetrieveOrderCustomAttribute

Retrieves a [custom attribute](entity:CustomAttribute) associated with an order.  You can use the `with_definition` query parameter to also retrieve the custom attribute definition in the same call.  To retrieve a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | The ID of the target [order](entity:Order). | [required] |
**custom_attribute_key** | **String** | The key of the custom attribute to retrieve.  This key must match the key of an existing custom attribute definition. | [required] |
**version** | Option<**i32**> | To enable [optimistic concurrency](https://developer.squareup.com/docs/build-basics/common-api-patterns/optimistic-concurrency) control, include this optional field and specify the current version of the custom attribute. |  |
**with_definition** | Option<**bool**> | Indicates whether to return the [custom attribute definition](entity:CustomAttributeDefinition) in the `definition` field of each  custom attribute. Set this parameter to `true` to get the name and description of each custom attribute,  information about the data type, or other definition details. The default value is `false`. |  |[default to false]

### Return type

[**models::RetrieveOrderCustomAttributeResponse**](RetrieveOrderCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_order_custom_attribute_definition

> models::RetrieveOrderCustomAttributeDefinitionResponse retrieve_order_custom_attribute_definition(key, version)
RetrieveOrderCustomAttributeDefinition

Retrieves an order-related [custom attribute definition](entity:CustomAttributeDefinition) from a Square seller account.  To retrieve a custom attribute definition created by another application, the `visibility` setting must be `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes (also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to retrieve. | [required] |
**version** | Option<**i32**> | To enable [optimistic concurrency](https://developer.squareup.com/docs/build-basics/common-api-patterns/optimistic-concurrency) control, include this optional field and specify the current version of the custom attribute. |  |

### Return type

[**models::RetrieveOrderCustomAttributeDefinitionResponse**](RetrieveOrderCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_order_custom_attribute_definition

> models::UpdateOrderCustomAttributeDefinitionResponse update_order_custom_attribute_definition(key, update_order_custom_attribute_definition_request)
UpdateOrderCustomAttributeDefinition

Updates an order-related custom attribute definition for a Square seller account.  Only the definition owner can update a custom attribute definition. Note that sellers can view all custom attributes in exported customer data, including those set to `VISIBILITY_HIDDEN`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to update. | [required] |
**update_order_custom_attribute_definition_request** | [**UpdateOrderCustomAttributeDefinitionRequest**](UpdateOrderCustomAttributeDefinitionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateOrderCustomAttributeDefinitionResponse**](UpdateOrderCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_order_custom_attribute

> models::UpsertOrderCustomAttributeResponse upsert_order_custom_attribute(order_id, custom_attribute_key, upsert_order_custom_attribute_request)
UpsertOrderCustomAttribute

Creates or updates a [custom attribute](entity:CustomAttribute) for an order.  Use this endpoint to set the value of a custom attribute for a specific order. A custom attribute is based on a custom attribute definition in a Square seller account. (To create a custom attribute definition, use the [CreateOrderCustomAttributeDefinition](api-endpoint:OrderCustomAttributes-CreateOrderCustomAttributeDefinition) endpoint.)  To create or update a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes (also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | The ID of the target [order](entity:Order). | [required] |
**custom_attribute_key** | **String** | The key of the custom attribute to create or update.  This key must match the key  of an existing custom attribute definition. | [required] |
**upsert_order_custom_attribute_request** | [**UpsertOrderCustomAttributeRequest**](UpsertOrderCustomAttributeRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpsertOrderCustomAttributeResponse**](UpsertOrderCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

