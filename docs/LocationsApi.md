# \LocationsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_location**](LocationsApi.md#create_location) | **POST** /v2/locations | CreateLocation
[**list_locations**](LocationsApi.md#list_locations) | **GET** /v2/locations | ListLocations
[**retrieve_location**](LocationsApi.md#retrieve_location) | **GET** /v2/locations/{location_id} | RetrieveLocation
[**update_location**](LocationsApi.md#update_location) | **PUT** /v2/locations/{location_id} | UpdateLocation



## create_location

> models::CreateLocationResponse create_location(create_location_request)
CreateLocation

Creates a [location](https://developer.squareup.com/docs/locations-api). Creating new locations allows for separate configuration of receipt layouts, item prices, and sales reports. Developers can use locations to separate sales activity through applications that integrate with Square from sales activity elsewhere in a seller's account. Locations created programmatically with the Locations API last forever and are visible to the seller for their own management. Therefore, ensure that each location has a sensible and unique name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_location_request** | [**CreateLocationRequest**](CreateLocationRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateLocationResponse**](CreateLocationResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_locations

> models::ListLocationsResponse list_locations()
ListLocations

Provides details about all of the seller's [locations](https://developer.squareup.com/docs/locations-api), including those with an inactive status. Locations are listed alphabetically by `name`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListLocationsResponse**](ListLocationsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_location

> models::RetrieveLocationResponse retrieve_location(location_id)
RetrieveLocation

Retrieves details of a single location. Specify \"main\" as the location ID to retrieve details of the [main location](https://developer.squareup.com/docs/locations-api#about-the-main-location).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the location to retrieve. Specify the string \"main\" to return the main location. | [required] |

### Return type

[**models::RetrieveLocationResponse**](RetrieveLocationResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_location

> models::UpdateLocationResponse update_location(location_id, update_location_request)
UpdateLocation

Updates a [location](https://developer.squareup.com/docs/locations-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the location to update. | [required] |
**update_location_request** | [**UpdateLocationRequest**](UpdateLocationRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateLocationResponse**](UpdateLocationResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

