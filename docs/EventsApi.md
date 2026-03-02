# \EventsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**disable_events**](EventsApi.md#disable_events) | **PUT** /v2/events/disable | DisableEvents
[**enable_events**](EventsApi.md#enable_events) | **PUT** /v2/events/enable | EnableEvents
[**list_event_types**](EventsApi.md#list_event_types) | **GET** /v2/events/types | ListEventTypes
[**search_events**](EventsApi.md#search_events) | **POST** /v2/events | SearchEvents



## disable_events

> models::DisableEventsResponse disable_events()
DisableEvents

Disables events to prevent them from being searchable. All events are disabled by default. You must enable events to make them searchable. Disabling events for a specific time period prevents them from being searchable, even if you re-enable them later.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DisableEventsResponse**](DisableEventsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_events

> models::EnableEventsResponse enable_events()
EnableEvents

Enables events to make them searchable. Only events that occur while in the enabled state are searchable.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::EnableEventsResponse**](EnableEventsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_event_types

> models::ListEventTypesResponse list_event_types(api_version)
ListEventTypes

Lists all event types that you can subscribe to as webhooks or query using the Events API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | Option<**String**> | The API version for which to list event types. Setting this field overrides the default version used by the application. |  |

### Return type

[**models::ListEventTypesResponse**](ListEventTypesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_events

> models::SearchEventsResponse search_events(search_events_request)
SearchEvents

Search for Square API events that occur within a 28-day timeframe.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_events_request** | [**SearchEventsRequest**](SearchEventsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchEventsResponse**](SearchEventsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

