# \PayoutsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_payout**](PayoutsApi.md#get_payout) | **GET** /v2/payouts/{payout_id} | GetPayout
[**list_payout_entries**](PayoutsApi.md#list_payout_entries) | **GET** /v2/payouts/{payout_id}/payout-entries | ListPayoutEntries
[**list_payouts**](PayoutsApi.md#list_payouts) | **GET** /v2/payouts | ListPayouts



## get_payout

> models::GetPayoutResponse get_payout(payout_id)
GetPayout

Retrieves details of a specific payout identified by a payout ID. To call this endpoint, set `PAYOUTS_READ` for the OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payout_id** | **String** | The ID of the payout to retrieve the information for. | [required] |

### Return type

[**models::GetPayoutResponse**](GetPayoutResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payout_entries

> models::ListPayoutEntriesResponse list_payout_entries(payout_id, sort_order, cursor, limit)
ListPayoutEntries

Retrieves a list of all payout entries for a specific payout. To call this endpoint, set `PAYOUTS_READ` for the OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payout_id** | **String** | The ID of the payout to retrieve the information for. | [required] |
**sort_order** | Option<[**SortOrder**](.md)> | The order in which payout entries are listed. |  |
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this cursor to retrieve the next set of results for the original query. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). If request parameters change between requests, subsequent results may contain duplicates or missing records. |  |
**limit** | Option<**i32**> | The maximum number of results to be returned in a single page. It is possible to receive fewer results than the specified limit on a given page. The default value of 100 is also the maximum allowed value. If the provided value is greater than 100, it is ignored and the default value is used instead. Default: `100` |  |

### Return type

[**models::ListPayoutEntriesResponse**](ListPayoutEntriesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payouts

> models::ListPayoutsResponse list_payouts(location_id, status, begin_time, end_time, sort_order, cursor, limit)
ListPayouts

Retrieves a list of all payouts for the default location. You can filter payouts by location ID, status, time range, and order them in ascending or descending order. To call this endpoint, set `PAYOUTS_READ` for the OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | Option<**String**> | The ID of the location for which to list the payouts. By default, payouts are returned for the default (main) location associated with the seller. |  |
**status** | Option<[**PayoutStatus**](.md)> | If provided, only payouts with the given status are returned. |  |
**begin_time** | Option<**String**> | The timestamp for the beginning of the payout creation time, in RFC 3339 format. Inclusive. Default: The current time minus one year. |  |
**end_time** | Option<**String**> | The timestamp for the end of the payout creation time, in RFC 3339 format. Default: The current time. |  |
**sort_order** | Option<[**SortOrder**](.md)> | The order in which payouts are listed. |  |
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this cursor to retrieve the next set of results for the original query. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). If request parameters change between requests, subsequent results may contain duplicates or missing records. |  |
**limit** | Option<**i32**> | The maximum number of results to be returned in a single page. It is possible to receive fewer results than the specified limit on a given page. The default value of 100 is also the maximum allowed value. If the provided value is greater than 100, it is ignored and the default value is used instead. Default: `100` |  |

### Return type

[**models::ListPayoutsResponse**](ListPayoutsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

