# \LocationCustomAttributesApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_delete_location_custom_attributes**](LocationCustomAttributesApi.md#bulk_delete_location_custom_attributes) | **POST** /v2/locations/custom-attributes/bulk-delete | BulkDeleteLocationCustomAttributes
[**bulk_upsert_location_custom_attributes**](LocationCustomAttributesApi.md#bulk_upsert_location_custom_attributes) | **POST** /v2/locations/custom-attributes/bulk-upsert | BulkUpsertLocationCustomAttributes
[**create_location_custom_attribute_definition**](LocationCustomAttributesApi.md#create_location_custom_attribute_definition) | **POST** /v2/locations/custom-attribute-definitions | CreateLocationCustomAttributeDefinition
[**delete_location_custom_attribute**](LocationCustomAttributesApi.md#delete_location_custom_attribute) | **DELETE** /v2/locations/{location_id}/custom-attributes/{key} | DeleteLocationCustomAttribute
[**delete_location_custom_attribute_definition**](LocationCustomAttributesApi.md#delete_location_custom_attribute_definition) | **DELETE** /v2/locations/custom-attribute-definitions/{key} | DeleteLocationCustomAttributeDefinition
[**list_location_custom_attribute_definitions**](LocationCustomAttributesApi.md#list_location_custom_attribute_definitions) | **GET** /v2/locations/custom-attribute-definitions | ListLocationCustomAttributeDefinitions
[**list_location_custom_attributes**](LocationCustomAttributesApi.md#list_location_custom_attributes) | **GET** /v2/locations/{location_id}/custom-attributes | ListLocationCustomAttributes
[**retrieve_location_custom_attribute**](LocationCustomAttributesApi.md#retrieve_location_custom_attribute) | **GET** /v2/locations/{location_id}/custom-attributes/{key} | RetrieveLocationCustomAttribute
[**retrieve_location_custom_attribute_definition**](LocationCustomAttributesApi.md#retrieve_location_custom_attribute_definition) | **GET** /v2/locations/custom-attribute-definitions/{key} | RetrieveLocationCustomAttributeDefinition
[**update_location_custom_attribute_definition**](LocationCustomAttributesApi.md#update_location_custom_attribute_definition) | **PUT** /v2/locations/custom-attribute-definitions/{key} | UpdateLocationCustomAttributeDefinition
[**upsert_location_custom_attribute**](LocationCustomAttributesApi.md#upsert_location_custom_attribute) | **POST** /v2/locations/{location_id}/custom-attributes/{key} | UpsertLocationCustomAttribute



## bulk_delete_location_custom_attributes

> models::BulkDeleteLocationCustomAttributesResponse bulk_delete_location_custom_attributes(bulk_delete_location_custom_attributes_request)
BulkDeleteLocationCustomAttributes

Deletes [custom attributes](entity:CustomAttribute) for locations as a bulk operation. To delete a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_delete_location_custom_attributes_request** | [**BulkDeleteLocationCustomAttributesRequest**](BulkDeleteLocationCustomAttributesRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkDeleteLocationCustomAttributesResponse**](BulkDeleteLocationCustomAttributesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_upsert_location_custom_attributes

> models::BulkUpsertLocationCustomAttributesResponse bulk_upsert_location_custom_attributes(bulk_upsert_location_custom_attributes_request)
BulkUpsertLocationCustomAttributes

Creates or updates [custom attributes](entity:CustomAttribute) for locations as a bulk operation. Use this endpoint to set the value of one or more custom attributes for one or more locations. A custom attribute is based on a custom attribute definition in a Square seller account, which is created using the [CreateLocationCustomAttributeDefinition](api-endpoint:LocationCustomAttributes-CreateLocationCustomAttributeDefinition) endpoint. This `BulkUpsertLocationCustomAttributes` endpoint accepts a map of 1 to 25 individual upsert requests and returns a map of individual upsert responses. Each upsert request has a unique ID and provides a location ID and custom attribute. Each upsert response is returned with the ID of the corresponding request. To create or update a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_upsert_location_custom_attributes_request** | [**BulkUpsertLocationCustomAttributesRequest**](BulkUpsertLocationCustomAttributesRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkUpsertLocationCustomAttributesResponse**](BulkUpsertLocationCustomAttributesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_location_custom_attribute_definition

> models::CreateLocationCustomAttributeDefinitionResponse create_location_custom_attribute_definition(create_location_custom_attribute_definition_request)
CreateLocationCustomAttributeDefinition

Creates a location-related [custom attribute definition](entity:CustomAttributeDefinition) for a Square seller account. Use this endpoint to define a custom attribute that can be associated with locations. A custom attribute definition specifies the `key`, `visibility`, `schema`, and other properties for a custom attribute. After the definition is created, you can call [UpsertLocationCustomAttribute](api-endpoint:LocationCustomAttributes-UpsertLocationCustomAttribute) or [BulkUpsertLocationCustomAttributes](api-endpoint:LocationCustomAttributes-BulkUpsertLocationCustomAttributes) to set the custom attribute for locations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_location_custom_attribute_definition_request** | [**CreateLocationCustomAttributeDefinitionRequest**](CreateLocationCustomAttributeDefinitionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateLocationCustomAttributeDefinitionResponse**](CreateLocationCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_location_custom_attribute

> models::DeleteLocationCustomAttributeResponse delete_location_custom_attribute(location_id, key)
DeleteLocationCustomAttribute

Deletes a [custom attribute](entity:CustomAttribute) associated with a location. To delete a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the target [location](entity:Location). | [required] |
**key** | **String** | The key of the custom attribute to delete. This key must match the `key` of a custom attribute definition in the Square seller account. If the requesting application is not the definition owner, you must use the qualified key. | [required] |

### Return type

[**models::DeleteLocationCustomAttributeResponse**](DeleteLocationCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_location_custom_attribute_definition

> models::DeleteLocationCustomAttributeDefinitionResponse delete_location_custom_attribute_definition(key)
DeleteLocationCustomAttributeDefinition

Deletes a location-related [custom attribute definition](entity:CustomAttributeDefinition) from a Square seller account. Deleting a custom attribute definition also deletes the corresponding custom attribute from all locations. Only the definition owner can delete a custom attribute definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to delete. | [required] |

### Return type

[**models::DeleteLocationCustomAttributeDefinitionResponse**](DeleteLocationCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_location_custom_attribute_definitions

> models::ListLocationCustomAttributeDefinitionsResponse list_location_custom_attribute_definitions(visibility_filter, limit, cursor)
ListLocationCustomAttributeDefinitions

Lists the location-related [custom attribute definitions](entity:CustomAttributeDefinition) that belong to a Square seller account. When all response pages are retrieved, the results include all custom attribute definitions that are visible to the requesting application, including those that are created by other applications and set to `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**visibility_filter** | Option<[**VisibilityFilter**](.md)> | Filters the `CustomAttributeDefinition` results by their `visibility` values. |  |
**limit** | Option<**i32**> | The maximum number of results to return in a single paged response. This limit is advisory. The response might contain more or fewer results. The minimum value is 1 and the maximum value is 100. The default value is 20. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**cursor** | Option<**String**> | The cursor returned in the paged response from the previous call to this endpoint. Provide this cursor to retrieve the next page of results for your original request. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |

### Return type

[**models::ListLocationCustomAttributeDefinitionsResponse**](ListLocationCustomAttributeDefinitionsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_location_custom_attributes

> models::ListLocationCustomAttributesResponse list_location_custom_attributes(location_id, visibility_filter, limit, cursor, with_definitions)
ListLocationCustomAttributes

Lists the [custom attributes](entity:CustomAttribute) associated with a location. You can use the `with_definitions` query parameter to also retrieve custom attribute definitions in the same call. When all response pages are retrieved, the results include all custom attributes that are visible to the requesting application, including those that are owned by other applications and set to `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the target [location](entity:Location). | [required] |
**visibility_filter** | Option<[**VisibilityFilter**](.md)> | Filters the `CustomAttributeDefinition` results by their `visibility` values. |  |
**limit** | Option<**i32**> | The maximum number of results to return in a single paged response. This limit is advisory. The response might contain more or fewer results. The minimum value is 1 and the maximum value is 100. The default value is 20. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**cursor** | Option<**String**> | The cursor returned in the paged response from the previous call to this endpoint. Provide this cursor to retrieve the next page of results for your original request. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**with_definitions** | Option<**bool**> | Indicates whether to return the [custom attribute definition](entity:CustomAttributeDefinition) in the `definition` field of each custom attribute. Set this parameter to `true` to get the name and description of each custom attribute, information about the data type, or other definition details. The default value is `false`. |  |[default to false]

### Return type

[**models::ListLocationCustomAttributesResponse**](ListLocationCustomAttributesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_location_custom_attribute

> models::RetrieveLocationCustomAttributeResponse retrieve_location_custom_attribute(location_id, key, with_definition, version)
RetrieveLocationCustomAttribute

Retrieves a [custom attribute](entity:CustomAttribute) associated with a location. You can use the `with_definition` query parameter to also retrieve the custom attribute definition in the same call. To retrieve a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the target [location](entity:Location). | [required] |
**key** | **String** | The key of the custom attribute to retrieve. This key must match the `key` of a custom attribute definition in the Square seller account. If the requesting application is not the definition owner, you must use the qualified key. | [required] |
**with_definition** | Option<**bool**> | Indicates whether to return the [custom attribute definition](entity:CustomAttributeDefinition) in the `definition` field of the custom attribute. Set this parameter to `true` to get the name and description of the custom attribute, information about the data type, or other definition details. The default value is `false`. |  |[default to false]
**version** | Option<**i32**> | The current version of the custom attribute, which is used for strongly consistent reads to guarantee that you receive the most up-to-date data. When included in the request, Square returns the specified version or a higher version if one exists. If the specified version is higher than the current version, Square returns a `BAD_REQUEST` error. |  |

### Return type

[**models::RetrieveLocationCustomAttributeResponse**](RetrieveLocationCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_location_custom_attribute_definition

> models::RetrieveLocationCustomAttributeDefinitionResponse retrieve_location_custom_attribute_definition(key, version)
RetrieveLocationCustomAttributeDefinition

Retrieves a location-related [custom attribute definition](entity:CustomAttributeDefinition) from a Square seller account. To retrieve a custom attribute definition created by another application, the `visibility` setting must be `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to retrieve. If the requesting application is not the definition owner, you must use the qualified key. | [required] |
**version** | Option<**i32**> | The current version of the custom attribute definition, which is used for strongly consistent reads to guarantee that you receive the most up-to-date data. When included in the request, Square returns the specified version or a higher version if one exists. If the specified version is higher than the current version, Square returns a `BAD_REQUEST` error. |  |

### Return type

[**models::RetrieveLocationCustomAttributeDefinitionResponse**](RetrieveLocationCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_location_custom_attribute_definition

> models::UpdateLocationCustomAttributeDefinitionResponse update_location_custom_attribute_definition(key, update_location_custom_attribute_definition_request)
UpdateLocationCustomAttributeDefinition

Updates a location-related [custom attribute definition](entity:CustomAttributeDefinition) for a Square seller account. Use this endpoint to update the following fields: `name`, `description`, `visibility`, or the `schema` for a `Selection` data type. Only the definition owner can update a custom attribute definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to update. | [required] |
**update_location_custom_attribute_definition_request** | [**UpdateLocationCustomAttributeDefinitionRequest**](UpdateLocationCustomAttributeDefinitionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateLocationCustomAttributeDefinitionResponse**](UpdateLocationCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_location_custom_attribute

> models::UpsertLocationCustomAttributeResponse upsert_location_custom_attribute(location_id, key, upsert_location_custom_attribute_request)
UpsertLocationCustomAttribute

Creates or updates a [custom attribute](entity:CustomAttribute) for a location. Use this endpoint to set the value of a custom attribute for a specified location. A custom attribute is based on a custom attribute definition in a Square seller account, which is created using the [CreateLocationCustomAttributeDefinition](api-endpoint:LocationCustomAttributes-CreateLocationCustomAttributeDefinition) endpoint. To create or update a custom attribute owned by another application, the `visibility` setting must be `VISIBILITY_READ_WRITE_VALUES`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the target [location](entity:Location). | [required] |
**key** | **String** | The key of the custom attribute to create or update. This key must match the `key` of a custom attribute definition in the Square seller account. If the requesting application is not the definition owner, you must use the qualified key. | [required] |
**upsert_location_custom_attribute_request** | [**UpsertLocationCustomAttributeRequest**](UpsertLocationCustomAttributeRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpsertLocationCustomAttributeResponse**](UpsertLocationCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

