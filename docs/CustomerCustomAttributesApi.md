# \CustomerCustomAttributesApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_upsert_customer_custom_attributes**](CustomerCustomAttributesApi.md#bulk_upsert_customer_custom_attributes) | **POST** /v2/customers/custom-attributes/bulk-upsert | BulkUpsertCustomerCustomAttributes
[**create_customer_custom_attribute_definition**](CustomerCustomAttributesApi.md#create_customer_custom_attribute_definition) | **POST** /v2/customers/custom-attribute-definitions | CreateCustomerCustomAttributeDefinition
[**delete_customer_custom_attribute**](CustomerCustomAttributesApi.md#delete_customer_custom_attribute) | **DELETE** /v2/customers/{customer_id}/custom-attributes/{key} | DeleteCustomerCustomAttribute
[**delete_customer_custom_attribute_definition**](CustomerCustomAttributesApi.md#delete_customer_custom_attribute_definition) | **DELETE** /v2/customers/custom-attribute-definitions/{key} | DeleteCustomerCustomAttributeDefinition
[**list_customer_custom_attribute_definitions**](CustomerCustomAttributesApi.md#list_customer_custom_attribute_definitions) | **GET** /v2/customers/custom-attribute-definitions | ListCustomerCustomAttributeDefinitions
[**list_customer_custom_attributes**](CustomerCustomAttributesApi.md#list_customer_custom_attributes) | **GET** /v2/customers/{customer_id}/custom-attributes | ListCustomerCustomAttributes
[**retrieve_customer_custom_attribute**](CustomerCustomAttributesApi.md#retrieve_customer_custom_attribute) | **GET** /v2/customers/{customer_id}/custom-attributes/{key} | RetrieveCustomerCustomAttribute
[**retrieve_customer_custom_attribute_definition**](CustomerCustomAttributesApi.md#retrieve_customer_custom_attribute_definition) | **GET** /v2/customers/custom-attribute-definitions/{key} | RetrieveCustomerCustomAttributeDefinition
[**update_customer_custom_attribute_definition**](CustomerCustomAttributesApi.md#update_customer_custom_attribute_definition) | **PUT** /v2/customers/custom-attribute-definitions/{key} | UpdateCustomerCustomAttributeDefinition
[**upsert_customer_custom_attribute**](CustomerCustomAttributesApi.md#upsert_customer_custom_attribute) | **POST** /v2/customers/{customer_id}/custom-attributes/{key} | UpsertCustomerCustomAttribute



## bulk_upsert_customer_custom_attributes

> models::BulkUpsertCustomerCustomAttributesResponse bulk_upsert_customer_custom_attributes(bulk_upsert_customer_custom_attributes_request)
BulkUpsertCustomerCustomAttributes

Creates or updates [custom attributes](entity:CustomAttribute) for customer profiles as a bulk operation.  Use this endpoint to set the value of one or more custom attributes for one or more customer profiles. A custom attribute is based on a custom attribute definition in a Square seller account, which is created using the [CreateCustomerCustomAttributeDefinition](api-endpoint:CustomerCustomAttributes-CreateCustomerCustomAttributeDefinition) endpoint.  This `BulkUpsertCustomerCustomAttributes` endpoint accepts a map of 1 to 25 individual upsert requests and returns a map of individual upsert responses. Each upsert request has a unique ID and provides a customer ID and custom attribute. Each upsert response is returned with the ID of the corresponding request.  To create or update a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes (also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_upsert_customer_custom_attributes_request** | [**BulkUpsertCustomerCustomAttributesRequest**](BulkUpsertCustomerCustomAttributesRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkUpsertCustomerCustomAttributesResponse**](BulkUpsertCustomerCustomAttributesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_customer_custom_attribute_definition

> models::CreateCustomerCustomAttributeDefinitionResponse create_customer_custom_attribute_definition(create_customer_custom_attribute_definition_request)
CreateCustomerCustomAttributeDefinition

Creates a customer-related [custom attribute definition](entity:CustomAttributeDefinition) for a Square seller account. Use this endpoint to define a custom attribute that can be associated with customer profiles.  A custom attribute definition specifies the `key`, `visibility`, `schema`, and other properties for a custom attribute. After the definition is created, you can call [UpsertCustomerCustomAttribute](api-endpoint:CustomerCustomAttributes-UpsertCustomerCustomAttribute) or [BulkUpsertCustomerCustomAttributes](api-endpoint:CustomerCustomAttributes-BulkUpsertCustomerCustomAttributes) to set the custom attribute for customer profiles in the seller's Customer Directory.  Sellers can view all custom attributes in exported customer data, including those set to `VISIBILITY_HIDDEN`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_customer_custom_attribute_definition_request** | [**CreateCustomerCustomAttributeDefinitionRequest**](CreateCustomerCustomAttributeDefinitionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateCustomerCustomAttributeDefinitionResponse**](CreateCustomerCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_customer_custom_attribute

> models::DeleteCustomerCustomAttributeResponse delete_customer_custom_attribute(customer_id, key)
DeleteCustomerCustomAttribute

Deletes a [custom attribute](entity:CustomAttribute) associated with a customer profile.  To delete a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes (also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The ID of the target [customer profile](entity:Customer). | [required] |
**key** | **String** | The key of the custom attribute to delete. This key must match the `key` of a custom attribute definition in the Square seller account. If the requesting application is not the definition owner, you must use the qualified key. | [required] |

### Return type

[**models::DeleteCustomerCustomAttributeResponse**](DeleteCustomerCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_customer_custom_attribute_definition

> models::DeleteCustomerCustomAttributeDefinitionResponse delete_customer_custom_attribute_definition(key)
DeleteCustomerCustomAttributeDefinition

Deletes a customer-related [custom attribute definition](entity:CustomAttributeDefinition) from a Square seller account.  Deleting a custom attribute definition also deletes the corresponding custom attribute from all customer profiles in the seller's Customer Directory.  Only the definition owner can delete a custom attribute definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to delete. | [required] |

### Return type

[**models::DeleteCustomerCustomAttributeDefinitionResponse**](DeleteCustomerCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_customer_custom_attribute_definitions

> models::ListCustomerCustomAttributeDefinitionsResponse list_customer_custom_attribute_definitions(limit, cursor)
ListCustomerCustomAttributeDefinitions

Lists the customer-related [custom attribute definitions](entity:CustomAttributeDefinition) that belong to a Square seller account.  When all response pages are retrieved, the results include all custom attribute definitions that are visible to the requesting application, including those that are created by other applications and set to `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes (also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum number of results to return in a single paged response. This limit is advisory. The response might contain more or fewer results. The minimum value is 1 and the maximum value is 100. The default value is 20. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**cursor** | Option<**String**> | The cursor returned in the paged response from the previous call to this endpoint. Provide this cursor to retrieve the next page of results for your original request. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |

### Return type

[**models::ListCustomerCustomAttributeDefinitionsResponse**](ListCustomerCustomAttributeDefinitionsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_customer_custom_attributes

> models::ListCustomerCustomAttributesResponse list_customer_custom_attributes(customer_id, limit, cursor, with_definitions)
ListCustomerCustomAttributes

Lists the [custom attributes](entity:CustomAttribute) associated with a customer profile.  You can use the `with_definitions` query parameter to also retrieve custom attribute definitions in the same call.  When all response pages are retrieved, the results include all custom attributes that are visible to the requesting application, including those that are owned by other applications and set to `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The ID of the target [customer profile](entity:Customer). | [required] |
**limit** | Option<**i32**> | The maximum number of results to return in a single paged response. This limit is advisory. The response might contain more or fewer results. The minimum value is 1 and the maximum value is 100. The default value is 20. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**cursor** | Option<**String**> | The cursor returned in the paged response from the previous call to this endpoint. Provide this cursor to retrieve the next page of results for your original request. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**with_definitions** | Option<**bool**> | Indicates whether to return the [custom attribute definition](entity:CustomAttributeDefinition) in the `definition` field of each custom attribute. Set this parameter to `true` to get the name and description of each custom attribute, information about the data type, or other definition details. The default value is `false`. |  |[default to false]

### Return type

[**models::ListCustomerCustomAttributesResponse**](ListCustomerCustomAttributesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_customer_custom_attribute

> models::RetrieveCustomerCustomAttributeResponse retrieve_customer_custom_attribute(customer_id, key, with_definition, version)
RetrieveCustomerCustomAttribute

Retrieves a [custom attribute](entity:CustomAttribute) associated with a customer profile.  You can use the `with_definition` query parameter to also retrieve the custom attribute definition in the same call.  To retrieve a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes (also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The ID of the target [customer profile](entity:Customer). | [required] |
**key** | **String** | The key of the custom attribute to retrieve. This key must match the `key` of a custom attribute definition in the Square seller account. If the requesting application is not the definition owner, you must use the qualified key. | [required] |
**with_definition** | Option<**bool**> | Indicates whether to return the [custom attribute definition](entity:CustomAttributeDefinition) in the `definition` field of the custom attribute. Set this parameter to `true` to get the name and description of the custom attribute, information about the data type, or other definition details. The default value is `false`. |  |[default to false]
**version** | Option<**i32**> | The current version of the custom attribute, which is used for strongly consistent reads to guarantee that you receive the most up-to-date data. When included in the request, Square returns the specified version or a higher version if one exists. If the specified version is higher than the current version, Square returns a `BAD_REQUEST` error. |  |

### Return type

[**models::RetrieveCustomerCustomAttributeResponse**](RetrieveCustomerCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_customer_custom_attribute_definition

> models::RetrieveCustomerCustomAttributeDefinitionResponse retrieve_customer_custom_attribute_definition(key, version)
RetrieveCustomerCustomAttributeDefinition

Retrieves a customer-related [custom attribute definition](entity:CustomAttributeDefinition) from a Square seller account.  To retrieve a custom attribute definition created by another application, the `visibility` setting must be `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes (also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to retrieve. If the requesting application is not the definition owner, you must use the qualified key. | [required] |
**version** | Option<**i32**> | The current version of the custom attribute definition, which is used for strongly consistent reads to guarantee that you receive the most up-to-date data. When included in the request, Square returns the specified version or a higher version if one exists. If the specified version is higher than the current version, Square returns a `BAD_REQUEST` error. |  |

### Return type

[**models::RetrieveCustomerCustomAttributeDefinitionResponse**](RetrieveCustomerCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_customer_custom_attribute_definition

> models::UpdateCustomerCustomAttributeDefinitionResponse update_customer_custom_attribute_definition(key, update_customer_custom_attribute_definition_request)
UpdateCustomerCustomAttributeDefinition

Updates a customer-related [custom attribute definition](entity:CustomAttributeDefinition) for a Square seller account.  Use this endpoint to update the following fields: `name`, `description`, `visibility`, or the `schema` for a `Selection` data type.  Only the definition owner can update a custom attribute definition. Note that sellers can view all custom attributes in exported customer data, including those set to `VISIBILITY_HIDDEN`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to update. | [required] |
**update_customer_custom_attribute_definition_request** | [**UpdateCustomerCustomAttributeDefinitionRequest**](UpdateCustomerCustomAttributeDefinitionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateCustomerCustomAttributeDefinitionResponse**](UpdateCustomerCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_customer_custom_attribute

> models::UpsertCustomerCustomAttributeResponse upsert_customer_custom_attribute(customer_id, key, upsert_customer_custom_attribute_request)
UpsertCustomerCustomAttribute

Creates or updates a [custom attribute](entity:CustomAttribute) for a customer profile.  Use this endpoint to set the value of a custom attribute for a specified customer profile. A custom attribute is based on a custom attribute definition in a Square seller account, which is created using the [CreateCustomerCustomAttributeDefinition](api-endpoint:CustomerCustomAttributes-CreateCustomerCustomAttributeDefinition) endpoint.  To create or update a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes (also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | The ID of the target [customer profile](entity:Customer). | [required] |
**key** | **String** | The key of the custom attribute to create or update. This key must match the `key` of a custom attribute definition in the Square seller account. If the requesting application is not the definition owner, you must use the qualified key. | [required] |
**upsert_customer_custom_attribute_request** | [**UpsertCustomerCustomAttributeRequest**](UpsertCustomerCustomAttributeRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpsertCustomerCustomAttributeResponse**](UpsertCustomerCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

