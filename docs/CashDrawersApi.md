# \CashDrawersApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_cash_drawer_shift_events**](CashDrawersApi.md#list_cash_drawer_shift_events) | **GET** /v2/cash-drawers/shifts/{shift_id}/events | ListCashDrawerShiftEvents
[**list_cash_drawer_shifts**](CashDrawersApi.md#list_cash_drawer_shifts) | **GET** /v2/cash-drawers/shifts | ListCashDrawerShifts
[**retrieve_cash_drawer_shift**](CashDrawersApi.md#retrieve_cash_drawer_shift) | **GET** /v2/cash-drawers/shifts/{shift_id} | RetrieveCashDrawerShift



## list_cash_drawer_shift_events

> models::ListCashDrawerShiftEventsResponse list_cash_drawer_shift_events(location_id, shift_id, limit, cursor)
ListCashDrawerShiftEvents

Provides a paginated list of events for a single cash drawer shift.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the location to list cash drawer shifts for. | [required] |
**shift_id** | **String** | The shift ID. | [required] |
**limit** | Option<**i32**> | Number of resources to be returned in a page of results (200 by default, 1000 max). |  |
**cursor** | Option<**String**> | Opaque cursor for fetching the next page of results. |  |

### Return type

[**models::ListCashDrawerShiftEventsResponse**](ListCashDrawerShiftEventsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cash_drawer_shifts

> models::ListCashDrawerShiftsResponse list_cash_drawer_shifts(location_id, sort_order, begin_time, end_time, limit, cursor)
ListCashDrawerShifts

Provides the details for all of the cash drawer shifts for a location in a date range.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the location to query for a list of cash drawer shifts. | [required] |
**sort_order** | Option<[**SortOrder**](.md)> | The order in which cash drawer shifts are listed in the response, based on their opened_at field. Default value: ASC |  |
**begin_time** | Option<**String**> | The inclusive start time of the query on opened_at, in ISO 8601 format. |  |
**end_time** | Option<**String**> | The exclusive end date of the query on opened_at, in ISO 8601 format. |  |
**limit** | Option<**i32**> | Number of cash drawer shift events in a page of results (200 by default, 1000 max). |  |
**cursor** | Option<**String**> | Opaque cursor for fetching the next page of results. |  |

### Return type

[**models::ListCashDrawerShiftsResponse**](ListCashDrawerShiftsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_cash_drawer_shift

> models::RetrieveCashDrawerShiftResponse retrieve_cash_drawer_shift(location_id, shift_id)
RetrieveCashDrawerShift

Provides the summary details for a single cash drawer shift. See [ListCashDrawerShiftEvents](api-endpoint:CashDrawers-ListCashDrawerShiftEvents) for a list of cash drawer shift events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the location to retrieve cash drawer shifts from. | [required] |
**shift_id** | **String** | The shift ID. | [required] |

### Return type

[**models::RetrieveCashDrawerShiftResponse**](RetrieveCashDrawerShiftResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

