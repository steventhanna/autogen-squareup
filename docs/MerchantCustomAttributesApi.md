# \MerchantCustomAttributesApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_delete_merchant_custom_attributes**](MerchantCustomAttributesApi.md#bulk_delete_merchant_custom_attributes) | **POST** /v2/merchants/custom-attributes/bulk-delete | BulkDeleteMerchantCustomAttributes
[**bulk_upsert_merchant_custom_attributes**](MerchantCustomAttributesApi.md#bulk_upsert_merchant_custom_attributes) | **POST** /v2/merchants/custom-attributes/bulk-upsert | BulkUpsertMerchantCustomAttributes
[**create_merchant_custom_attribute_definition**](MerchantCustomAttributesApi.md#create_merchant_custom_attribute_definition) | **POST** /v2/merchants/custom-attribute-definitions | CreateMerchantCustomAttributeDefinition
[**delete_merchant_custom_attribute**](MerchantCustomAttributesApi.md#delete_merchant_custom_attribute) | **DELETE** /v2/merchants/{merchant_id}/custom-attributes/{key} | DeleteMerchantCustomAttribute
[**delete_merchant_custom_attribute_definition**](MerchantCustomAttributesApi.md#delete_merchant_custom_attribute_definition) | **DELETE** /v2/merchants/custom-attribute-definitions/{key} | DeleteMerchantCustomAttributeDefinition
[**list_merchant_custom_attribute_definitions**](MerchantCustomAttributesApi.md#list_merchant_custom_attribute_definitions) | **GET** /v2/merchants/custom-attribute-definitions | ListMerchantCustomAttributeDefinitions
[**list_merchant_custom_attributes**](MerchantCustomAttributesApi.md#list_merchant_custom_attributes) | **GET** /v2/merchants/{merchant_id}/custom-attributes | ListMerchantCustomAttributes
[**retrieve_merchant_custom_attribute**](MerchantCustomAttributesApi.md#retrieve_merchant_custom_attribute) | **GET** /v2/merchants/{merchant_id}/custom-attributes/{key} | RetrieveMerchantCustomAttribute
[**retrieve_merchant_custom_attribute_definition**](MerchantCustomAttributesApi.md#retrieve_merchant_custom_attribute_definition) | **GET** /v2/merchants/custom-attribute-definitions/{key} | RetrieveMerchantCustomAttributeDefinition
[**update_merchant_custom_attribute_definition**](MerchantCustomAttributesApi.md#update_merchant_custom_attribute_definition) | **PUT** /v2/merchants/custom-attribute-definitions/{key} | UpdateMerchantCustomAttributeDefinition
[**upsert_merchant_custom_attribute**](MerchantCustomAttributesApi.md#upsert_merchant_custom_attribute) | **POST** /v2/merchants/{merchant_id}/custom-attributes/{key} | UpsertMerchantCustomAttribute



## bulk_delete_merchant_custom_attributes

> models::BulkDeleteMerchantCustomAttributesResponse bulk_delete_merchant_custom_attributes(bulk_delete_merchant_custom_attributes_request)
BulkDeleteMerchantCustomAttributes

Deletes [custom attributes](entity:CustomAttribute) for a merchant as a bulk operation. To delete a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_delete_merchant_custom_attributes_request** | [**BulkDeleteMerchantCustomAttributesRequest**](BulkDeleteMerchantCustomAttributesRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkDeleteMerchantCustomAttributesResponse**](BulkDeleteMerchantCustomAttributesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_upsert_merchant_custom_attributes

> models::BulkUpsertMerchantCustomAttributesResponse bulk_upsert_merchant_custom_attributes(bulk_upsert_merchant_custom_attributes_request)
BulkUpsertMerchantCustomAttributes

Creates or updates [custom attributes](entity:CustomAttribute) for a merchant as a bulk operation. Use this endpoint to set the value of one or more custom attributes for a merchant. A custom attribute is based on a custom attribute definition in a Square seller account, which is created using the [CreateMerchantCustomAttributeDefinition](api-endpoint:MerchantCustomAttributes-CreateMerchantCustomAttributeDefinition) endpoint. This `BulkUpsertMerchantCustomAttributes` endpoint accepts a map of 1 to 25 individual upsert requests and returns a map of individual upsert responses. Each upsert request has a unique ID and provides a merchant ID and custom attribute. Each upsert response is returned with the ID of the corresponding request. To create or update a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_upsert_merchant_custom_attributes_request** | [**BulkUpsertMerchantCustomAttributesRequest**](BulkUpsertMerchantCustomAttributesRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkUpsertMerchantCustomAttributesResponse**](BulkUpsertMerchantCustomAttributesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_merchant_custom_attribute_definition

> models::CreateMerchantCustomAttributeDefinitionResponse create_merchant_custom_attribute_definition(create_merchant_custom_attribute_definition_request)
CreateMerchantCustomAttributeDefinition

Creates a merchant-related [custom attribute definition](entity:CustomAttributeDefinition) for a Square seller account. Use this endpoint to define a custom attribute that can be associated with a merchant connecting to your application. A custom attribute definition specifies the `key`, `visibility`, `schema`, and other properties for a custom attribute. After the definition is created, you can call [UpsertMerchantCustomAttribute](api-endpoint:MerchantCustomAttributes-UpsertMerchantCustomAttribute) or [BulkUpsertMerchantCustomAttributes](api-endpoint:MerchantCustomAttributes-BulkUpsertMerchantCustomAttributes) to set the custom attribute for a merchant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_merchant_custom_attribute_definition_request** | [**CreateMerchantCustomAttributeDefinitionRequest**](CreateMerchantCustomAttributeDefinitionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateMerchantCustomAttributeDefinitionResponse**](CreateMerchantCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_merchant_custom_attribute

> models::DeleteMerchantCustomAttributeResponse delete_merchant_custom_attribute(merchant_id, key)
DeleteMerchantCustomAttribute

Deletes a [custom attribute](entity:CustomAttribute) associated with a merchant. To delete a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**merchant_id** | **String** | The ID of the target [merchant](entity:Merchant). | [required] |
**key** | **String** | The key of the custom attribute to delete. This key must match the `key` of a custom attribute definition in the Square seller account. If the requesting application is not the definition owner, you must use the qualified key. | [required] |

### Return type

[**models::DeleteMerchantCustomAttributeResponse**](DeleteMerchantCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_merchant_custom_attribute_definition

> models::DeleteMerchantCustomAttributeDefinitionResponse delete_merchant_custom_attribute_definition(key)
DeleteMerchantCustomAttributeDefinition

Deletes a merchant-related [custom attribute definition](entity:CustomAttributeDefinition) from a Square seller account. Deleting a custom attribute definition also deletes the corresponding custom attribute from the merchant. Only the definition owner can delete a custom attribute definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to delete. | [required] |

### Return type

[**models::DeleteMerchantCustomAttributeDefinitionResponse**](DeleteMerchantCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_merchant_custom_attribute_definitions

> models::ListMerchantCustomAttributeDefinitionsResponse list_merchant_custom_attribute_definitions(visibility_filter, limit, cursor)
ListMerchantCustomAttributeDefinitions

Lists the merchant-related [custom attribute definitions](entity:CustomAttributeDefinition) that belong to a Square seller account. When all response pages are retrieved, the results include all custom attribute definitions that are visible to the requesting application, including those that are created by other applications and set to `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**visibility_filter** | Option<[**VisibilityFilter**](.md)> | Filters the `CustomAttributeDefinition` results by their `visibility` values. |  |
**limit** | Option<**i32**> | The maximum number of results to return in a single paged response. This limit is advisory. The response might contain more or fewer results. The minimum value is 1 and the maximum value is 100. The default value is 20. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**cursor** | Option<**String**> | The cursor returned in the paged response from the previous call to this endpoint. Provide this cursor to retrieve the next page of results for your original request. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |

### Return type

[**models::ListMerchantCustomAttributeDefinitionsResponse**](ListMerchantCustomAttributeDefinitionsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_merchant_custom_attributes

> models::ListMerchantCustomAttributesResponse list_merchant_custom_attributes(merchant_id, visibility_filter, limit, cursor, with_definitions)
ListMerchantCustomAttributes

Lists the [custom attributes](entity:CustomAttribute) associated with a merchant. You can use the `with_definitions` query parameter to also retrieve custom attribute definitions in the same call. When all response pages are retrieved, the results include all custom attributes that are visible to the requesting application, including those that are owned by other applications and set to `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**merchant_id** | **String** | The ID of the target [merchant](entity:Merchant). | [required] |
**visibility_filter** | Option<[**VisibilityFilter**](.md)> | Filters the `CustomAttributeDefinition` results by their `visibility` values. |  |
**limit** | Option<**i32**> | The maximum number of results to return in a single paged response. This limit is advisory. The response might contain more or fewer results. The minimum value is 1 and the maximum value is 100. The default value is 20. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**cursor** | Option<**String**> | The cursor returned in the paged response from the previous call to this endpoint. Provide this cursor to retrieve the next page of results for your original request. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**with_definitions** | Option<**bool**> | Indicates whether to return the [custom attribute definition](entity:CustomAttributeDefinition) in the `definition` field of each custom attribute. Set this parameter to `true` to get the name and description of each custom attribute, information about the data type, or other definition details. The default value is `false`. |  |[default to false]

### Return type

[**models::ListMerchantCustomAttributesResponse**](ListMerchantCustomAttributesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_merchant_custom_attribute

> models::RetrieveMerchantCustomAttributeResponse retrieve_merchant_custom_attribute(merchant_id, key, with_definition, version)
RetrieveMerchantCustomAttribute

Retrieves a [custom attribute](entity:CustomAttribute) associated with a merchant. You can use the `with_definition` query parameter to also retrieve the custom attribute definition in the same call. To retrieve a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**merchant_id** | **String** | The ID of the target [merchant](entity:Merchant). | [required] |
**key** | **String** | The key of the custom attribute to retrieve. This key must match the `key` of a custom attribute definition in the Square seller account. If the requesting application is not the definition owner, you must use the qualified key. | [required] |
**with_definition** | Option<**bool**> | Indicates whether to return the [custom attribute definition](entity:CustomAttributeDefinition) in the `definition` field of the custom attribute. Set this parameter to `true` to get the name and description of the custom attribute, information about the data type, or other definition details. The default value is `false`. |  |[default to false]
**version** | Option<**i32**> | The current version of the custom attribute, which is used for strongly consistent reads to guarantee that you receive the most up-to-date data. When included in the request, Square returns the specified version or a higher version if one exists. If the specified version is higher than the current version, Square returns a `BAD_REQUEST` error. |  |

### Return type

[**models::RetrieveMerchantCustomAttributeResponse**](RetrieveMerchantCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_merchant_custom_attribute_definition

> models::RetrieveMerchantCustomAttributeDefinitionResponse retrieve_merchant_custom_attribute_definition(key, version)
RetrieveMerchantCustomAttributeDefinition

Retrieves a merchant-related [custom attribute definition](entity:CustomAttributeDefinition) from a Square seller account. To retrieve a custom attribute definition created by another application, the `visibility` setting must be `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to retrieve. If the requesting application is not the definition owner, you must use the qualified key. | [required] |
**version** | Option<**i32**> | The current version of the custom attribute definition, which is used for strongly consistent reads to guarantee that you receive the most up-to-date data. When included in the request, Square returns the specified version or a higher version if one exists. If the specified version is higher than the current version, Square returns a `BAD_REQUEST` error. |  |

### Return type

[**models::RetrieveMerchantCustomAttributeDefinitionResponse**](RetrieveMerchantCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_merchant_custom_attribute_definition

> models::UpdateMerchantCustomAttributeDefinitionResponse update_merchant_custom_attribute_definition(key, update_merchant_custom_attribute_definition_request)
UpdateMerchantCustomAttributeDefinition

Updates a merchant-related [custom attribute definition](entity:CustomAttributeDefinition) for a Square seller account. Use this endpoint to update the following fields: `name`, `description`, `visibility`, or the `schema` for a `Selection` data type. Only the definition owner can update a custom attribute definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to update. | [required] |
**update_merchant_custom_attribute_definition_request** | [**UpdateMerchantCustomAttributeDefinitionRequest**](UpdateMerchantCustomAttributeDefinitionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateMerchantCustomAttributeDefinitionResponse**](UpdateMerchantCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_merchant_custom_attribute

> models::UpsertMerchantCustomAttributeResponse upsert_merchant_custom_attribute(merchant_id, key, upsert_merchant_custom_attribute_request)
UpsertMerchantCustomAttribute

Creates or updates a [custom attribute](entity:CustomAttribute) for a merchant. Use this endpoint to set the value of a custom attribute for a specified merchant. A custom attribute is based on a custom attribute definition in a Square seller account, which is created using the [CreateMerchantCustomAttributeDefinition](api-endpoint:MerchantCustomAttributes-CreateMerchantCustomAttributeDefinition) endpoint. To create or update a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**merchant_id** | **String** | The ID of the target [merchant](entity:Merchant). | [required] |
**key** | **String** | The key of the custom attribute to create or update. This key must match the `key` of a custom attribute definition in the Square seller account. If the requesting application is not the definition owner, you must use the qualified key. | [required] |
**upsert_merchant_custom_attribute_request** | [**UpsertMerchantCustomAttributeRequest**](UpsertMerchantCustomAttributeRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpsertMerchantCustomAttributeResponse**](UpsertMerchantCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

