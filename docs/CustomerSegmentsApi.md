# \CustomerSegmentsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_customer_segments**](CustomerSegmentsApi.md#list_customer_segments) | **GET** /v2/customers/segments | ListCustomerSegments
[**retrieve_customer_segment**](CustomerSegmentsApi.md#retrieve_customer_segment) | **GET** /v2/customers/segments/{segment_id} | RetrieveCustomerSegment



## list_customer_segments

> models::ListCustomerSegmentsResponse list_customer_segments(cursor, limit)
ListCustomerSegments

Retrieves the list of customer segments of a business.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | A pagination cursor returned by previous calls to `ListCustomerSegments`. This cursor is used to retrieve the next set of query results.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**limit** | Option<**i32**> | The maximum number of results to return in a single page. This limit is advisory. The response might contain more or fewer results. If the specified limit is less than 1 or greater than 50, Square returns a `400 VALUE_TOO_LOW` or `400 VALUE_TOO_HIGH` error. The default value is 50.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |

### Return type

[**models::ListCustomerSegmentsResponse**](ListCustomerSegmentsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_customer_segment

> models::RetrieveCustomerSegmentResponse retrieve_customer_segment(segment_id)
RetrieveCustomerSegment

Retrieves a specific customer segment as identified by the `segment_id` value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | **String** | The Square-issued ID of the customer segment. | [required] |

### Return type

[**models::RetrieveCustomerSegmentResponse**](RetrieveCustomerSegmentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

