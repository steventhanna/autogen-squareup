# \BookingCustomAttributesApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_delete_booking_custom_attributes**](BookingCustomAttributesApi.md#bulk_delete_booking_custom_attributes) | **POST** /v2/bookings/custom-attributes/bulk-delete | BulkDeleteBookingCustomAttributes
[**bulk_upsert_booking_custom_attributes**](BookingCustomAttributesApi.md#bulk_upsert_booking_custom_attributes) | **POST** /v2/bookings/custom-attributes/bulk-upsert | BulkUpsertBookingCustomAttributes
[**create_booking_custom_attribute_definition**](BookingCustomAttributesApi.md#create_booking_custom_attribute_definition) | **POST** /v2/bookings/custom-attribute-definitions | CreateBookingCustomAttributeDefinition
[**delete_booking_custom_attribute**](BookingCustomAttributesApi.md#delete_booking_custom_attribute) | **DELETE** /v2/bookings/{booking_id}/custom-attributes/{key} | DeleteBookingCustomAttribute
[**delete_booking_custom_attribute_definition**](BookingCustomAttributesApi.md#delete_booking_custom_attribute_definition) | **DELETE** /v2/bookings/custom-attribute-definitions/{key} | DeleteBookingCustomAttributeDefinition
[**list_booking_custom_attribute_definitions**](BookingCustomAttributesApi.md#list_booking_custom_attribute_definitions) | **GET** /v2/bookings/custom-attribute-definitions | ListBookingCustomAttributeDefinitions
[**list_booking_custom_attributes**](BookingCustomAttributesApi.md#list_booking_custom_attributes) | **GET** /v2/bookings/{booking_id}/custom-attributes | ListBookingCustomAttributes
[**retrieve_booking_custom_attribute**](BookingCustomAttributesApi.md#retrieve_booking_custom_attribute) | **GET** /v2/bookings/{booking_id}/custom-attributes/{key} | RetrieveBookingCustomAttribute
[**retrieve_booking_custom_attribute_definition**](BookingCustomAttributesApi.md#retrieve_booking_custom_attribute_definition) | **GET** /v2/bookings/custom-attribute-definitions/{key} | RetrieveBookingCustomAttributeDefinition
[**update_booking_custom_attribute_definition**](BookingCustomAttributesApi.md#update_booking_custom_attribute_definition) | **PUT** /v2/bookings/custom-attribute-definitions/{key} | UpdateBookingCustomAttributeDefinition
[**upsert_booking_custom_attribute**](BookingCustomAttributesApi.md#upsert_booking_custom_attribute) | **PUT** /v2/bookings/{booking_id}/custom-attributes/{key} | UpsertBookingCustomAttribute



## bulk_delete_booking_custom_attributes

> models::BulkDeleteBookingCustomAttributesResponse bulk_delete_booking_custom_attributes(bulk_delete_booking_custom_attributes_request)
BulkDeleteBookingCustomAttributes

Bulk deletes bookings custom attributes.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_WRITE` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_WRITE` and `APPOINTMENTS_WRITE` for the OAuth scope.  For calls to this endpoint with seller-level permissions to succeed, the seller must have subscribed to *Appointments Plus* or *Appointments Premium*.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_delete_booking_custom_attributes_request** | [**BulkDeleteBookingCustomAttributesRequest**](BulkDeleteBookingCustomAttributesRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkDeleteBookingCustomAttributesResponse**](BulkDeleteBookingCustomAttributesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_upsert_booking_custom_attributes

> models::BulkUpsertBookingCustomAttributesResponse bulk_upsert_booking_custom_attributes(bulk_upsert_booking_custom_attributes_request)
BulkUpsertBookingCustomAttributes

Bulk upserts bookings custom attributes.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_WRITE` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_WRITE` and `APPOINTMENTS_WRITE` for the OAuth scope.  For calls to this endpoint with seller-level permissions to succeed, the seller must have subscribed to *Appointments Plus* or *Appointments Premium*.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_upsert_booking_custom_attributes_request** | [**BulkUpsertBookingCustomAttributesRequest**](BulkUpsertBookingCustomAttributesRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkUpsertBookingCustomAttributesResponse**](BulkUpsertBookingCustomAttributesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_booking_custom_attribute_definition

> models::CreateBookingCustomAttributeDefinitionResponse create_booking_custom_attribute_definition(create_booking_custom_attribute_definition_request)
CreateBookingCustomAttributeDefinition

Creates a bookings custom attribute definition.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_WRITE` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_WRITE` and `APPOINTMENTS_WRITE` for the OAuth scope.  For calls to this endpoint with seller-level permissions to succeed, the seller must have subscribed to *Appointments Plus* or *Appointments Premium*.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_booking_custom_attribute_definition_request** | [**CreateBookingCustomAttributeDefinitionRequest**](CreateBookingCustomAttributeDefinitionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateBookingCustomAttributeDefinitionResponse**](CreateBookingCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_booking_custom_attribute

> models::DeleteBookingCustomAttributeResponse delete_booking_custom_attribute(booking_id, key)
DeleteBookingCustomAttribute

Deletes a bookings custom attribute.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_WRITE` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_WRITE` and `APPOINTMENTS_WRITE` for the OAuth scope.  For calls to this endpoint with seller-level permissions to succeed, the seller must have subscribed to *Appointments Plus* or *Appointments Premium*.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**booking_id** | **String** | The ID of the target [booking](entity:Booking). | [required] |
**key** | **String** | The key of the custom attribute to delete. This key must match the `key` of a custom attribute definition in the Square seller account. If the requesting application is not the definition owner, you must use the qualified key. | [required] |

### Return type

[**models::DeleteBookingCustomAttributeResponse**](DeleteBookingCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_booking_custom_attribute_definition

> models::DeleteBookingCustomAttributeDefinitionResponse delete_booking_custom_attribute_definition(key)
DeleteBookingCustomAttributeDefinition

Deletes a bookings custom attribute definition.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_WRITE` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_WRITE` and `APPOINTMENTS_WRITE` for the OAuth scope.  For calls to this endpoint with seller-level permissions to succeed, the seller must have subscribed to *Appointments Plus* or *Appointments Premium*.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to delete. | [required] |

### Return type

[**models::DeleteBookingCustomAttributeDefinitionResponse**](DeleteBookingCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_booking_custom_attribute_definitions

> models::ListBookingCustomAttributeDefinitionsResponse list_booking_custom_attribute_definitions(limit, cursor)
ListBookingCustomAttributeDefinitions

Get all bookings custom attribute definitions.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_READ` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_READ` and `APPOINTMENTS_READ` for the OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum number of results to return in a single paged response. This limit is advisory. The response might contain more or fewer results. The minimum value is 1 and the maximum value is 100. The default value is 20. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**cursor** | Option<**String**> | The cursor returned in the paged response from the previous call to this endpoint. Provide this cursor to retrieve the next page of results for your original request. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |

### Return type

[**models::ListBookingCustomAttributeDefinitionsResponse**](ListBookingCustomAttributeDefinitionsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_booking_custom_attributes

> models::ListBookingCustomAttributesResponse list_booking_custom_attributes(booking_id, limit, cursor, with_definitions)
ListBookingCustomAttributes

Lists a booking's custom attributes.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_READ` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_READ` and `APPOINTMENTS_READ` for the OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**booking_id** | **String** | The ID of the target [booking](entity:Booking). | [required] |
**limit** | Option<**i32**> | The maximum number of results to return in a single paged response. This limit is advisory. The response might contain more or fewer results. The minimum value is 1 and the maximum value is 100. The default value is 20. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**cursor** | Option<**String**> | The cursor returned in the paged response from the previous call to this endpoint. Provide this cursor to retrieve the next page of results for your original request. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**with_definitions** | Option<**bool**> | Indicates whether to return the [custom attribute definition](entity:CustomAttributeDefinition) in the `definition` field of each custom attribute. Set this parameter to `true` to get the name and description of each custom attribute, information about the data type, or other definition details. The default value is `false`. |  |[default to false]

### Return type

[**models::ListBookingCustomAttributesResponse**](ListBookingCustomAttributesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_booking_custom_attribute

> models::RetrieveBookingCustomAttributeResponse retrieve_booking_custom_attribute(booking_id, key, with_definition, version)
RetrieveBookingCustomAttribute

Retrieves a bookings custom attribute.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_READ` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_READ` and `APPOINTMENTS_READ` for the OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**booking_id** | **String** | The ID of the target [booking](entity:Booking). | [required] |
**key** | **String** | The key of the custom attribute to retrieve. This key must match the `key` of a custom attribute definition in the Square seller account. If the requesting application is not the definition owner, you must use the qualified key. | [required] |
**with_definition** | Option<**bool**> | Indicates whether to return the [custom attribute definition](entity:CustomAttributeDefinition) in the `definition` field of the custom attribute. Set this parameter to `true` to get the name and description of the custom attribute, information about the data type, or other definition details. The default value is `false`. |  |[default to false]
**version** | Option<**i32**> | The current version of the custom attribute, which is used for strongly consistent reads to guarantee that you receive the most up-to-date data. When included in the request, Square returns the specified version or a higher version if one exists. If the specified version is higher than the current version, Square returns a `BAD_REQUEST` error. |  |

### Return type

[**models::RetrieveBookingCustomAttributeResponse**](RetrieveBookingCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_booking_custom_attribute_definition

> models::RetrieveBookingCustomAttributeDefinitionResponse retrieve_booking_custom_attribute_definition(key, version)
RetrieveBookingCustomAttributeDefinition

Retrieves a bookings custom attribute definition.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_READ` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_READ` and `APPOINTMENTS_READ` for the OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to retrieve. If the requesting application is not the definition owner, you must use the qualified key. | [required] |
**version** | Option<**i32**> | The current version of the custom attribute definition, which is used for strongly consistent reads to guarantee that you receive the most up-to-date data. When included in the request, Square returns the specified version or a higher version if one exists. If the specified version is higher than the current version, Square returns a `BAD_REQUEST` error. |  |

### Return type

[**models::RetrieveBookingCustomAttributeDefinitionResponse**](RetrieveBookingCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_booking_custom_attribute_definition

> models::UpdateBookingCustomAttributeDefinitionResponse update_booking_custom_attribute_definition(key, update_booking_custom_attribute_definition_request)
UpdateBookingCustomAttributeDefinition

Updates a bookings custom attribute definition.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_WRITE` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_WRITE` and `APPOINTMENTS_WRITE` for the OAuth scope.  For calls to this endpoint with seller-level permissions to succeed, the seller must have subscribed to *Appointments Plus* or *Appointments Premium*.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The key of the custom attribute definition to update. | [required] |
**update_booking_custom_attribute_definition_request** | [**UpdateBookingCustomAttributeDefinitionRequest**](UpdateBookingCustomAttributeDefinitionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateBookingCustomAttributeDefinitionResponse**](UpdateBookingCustomAttributeDefinitionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_booking_custom_attribute

> models::UpsertBookingCustomAttributeResponse upsert_booking_custom_attribute(booking_id, key, upsert_booking_custom_attribute_request)
UpsertBookingCustomAttribute

Upserts a bookings custom attribute.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_WRITE` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_WRITE` and `APPOINTMENTS_WRITE` for the OAuth scope.  For calls to this endpoint with seller-level permissions to succeed, the seller must have subscribed to *Appointments Plus* or *Appointments Premium*.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**booking_id** | **String** | The ID of the target [booking](entity:Booking). | [required] |
**key** | **String** | The key of the custom attribute to create or update. This key must match the `key` of a custom attribute definition in the Square seller account. If the requesting application is not the definition owner, you must use the qualified key. | [required] |
**upsert_booking_custom_attribute_request** | [**UpsertBookingCustomAttributeRequest**](UpsertBookingCustomAttributeRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpsertBookingCustomAttributeResponse**](UpsertBookingCustomAttributeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

