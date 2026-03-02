# \RefundsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_payment_refund**](RefundsApi.md#get_payment_refund) | **GET** /v2/refunds/{refund_id} | GetPaymentRefund
[**list_payment_refunds**](RefundsApi.md#list_payment_refunds) | **GET** /v2/refunds | ListPaymentRefunds
[**refund_payment**](RefundsApi.md#refund_payment) | **POST** /v2/refunds | RefundPayment



## get_payment_refund

> models::GetPaymentRefundResponse get_payment_refund(refund_id)
GetPaymentRefund

Retrieves a specific refund using the `refund_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refund_id** | **String** | The unique ID for the desired `PaymentRefund`. | [required] |

### Return type

[**models::GetPaymentRefundResponse**](GetPaymentRefundResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payment_refunds

> models::ListPaymentRefundsResponse list_payment_refunds(begin_time, end_time, sort_order, cursor, location_id, status, source_type, limit, updated_at_begin_time, updated_at_end_time, sort_field)
ListPaymentRefunds

Retrieves a list of refunds for the account making the request.  Results are eventually consistent, and new refunds or changes to refunds might take several seconds to appear.  The maximum results per page is 100.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**begin_time** | Option<**String**> | Indicates the start of the time range to retrieve each `PaymentRefund` for, in RFC 3339  format.  The range is determined using the `created_at` field for each `PaymentRefund`.   Default: The current time minus one year. |  |
**end_time** | Option<**String**> | Indicates the end of the time range to retrieve each `PaymentRefund` for, in RFC 3339  format.  The range is determined using the `created_at` field for each `PaymentRefund`.  Default: The current time. |  |
**sort_order** | Option<**String**> | The order in which results are listed by `PaymentRefund.created_at`: - `ASC` - Oldest to newest. - `DESC` - Newest to oldest (default). |  |
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this cursor to retrieve the next set of results for the original query.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**location_id** | Option<**String**> | Limit results to the location supplied. By default, results are returned for all locations associated with the seller. |  |
**status** | Option<**String**> | If provided, only refunds with the given status are returned. For a list of refund status values, see [PaymentRefund](entity:PaymentRefund).  Default: If omitted, refunds are returned regardless of their status. |  |
**source_type** | Option<**String**> | If provided, only returns refunds whose payments have the indicated source type. Current values include `CARD`, `BANK_ACCOUNT`, `WALLET`, `CASH`, and `EXTERNAL`. For information about these payment source types, see [Take Payments](https://developer.squareup.com/docs/payments-api/take-payments).  Default: If omitted, refunds are returned regardless of the source type. |  |
**limit** | Option<**i32**> | The maximum number of results to be returned in a single page.  It is possible to receive fewer results than the specified limit on a given page.  If the supplied value is greater than 100, no more than 100 results are returned.  Default: 100 |  |
**updated_at_begin_time** | Option<**String**> | Indicates the start of the time range to retrieve each `PaymentRefund` for, in RFC 3339 format.  The range is determined using the `updated_at` field for each `PaymentRefund`.  Default: If omitted, the time range starts at `begin_time`. |  |
**updated_at_end_time** | Option<**String**> | Indicates the end of the time range to retrieve each `PaymentRefund` for, in RFC 3339 format.  The range is determined using the `updated_at` field for each `PaymentRefund`.  Default: The current time. |  |
**sort_field** | Option<[**ListPaymentRefundsRequestSortField**](.md)> | The field used to sort results by. The default is `CREATED_AT`. |  |

### Return type

[**models::ListPaymentRefundsResponse**](ListPaymentRefundsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refund_payment

> models::RefundPaymentResponse refund_payment(refund_payment_request)
RefundPayment

Refunds a payment. You can refund the entire payment amount or a portion of it. You can use this endpoint to refund a card payment or record a  refund of a cash or external payment. For more information, see [Refund Payment](https://developer.squareup.com/docs/payments-api/refund-payments).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refund_payment_request** | [**RefundPaymentRequest**](RefundPaymentRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::RefundPaymentResponse**](RefundPaymentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

