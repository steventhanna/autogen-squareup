# \PaymentsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_payment**](PaymentsApi.md#cancel_payment) | **POST** /v2/payments/{payment_id}/cancel | CancelPayment
[**cancel_payment_by_idempotency_key**](PaymentsApi.md#cancel_payment_by_idempotency_key) | **POST** /v2/payments/cancel | CancelPaymentByIdempotencyKey
[**complete_payment**](PaymentsApi.md#complete_payment) | **POST** /v2/payments/{payment_id}/complete | CompletePayment
[**create_payment**](PaymentsApi.md#create_payment) | **POST** /v2/payments | CreatePayment
[**get_payment**](PaymentsApi.md#get_payment) | **GET** /v2/payments/{payment_id} | GetPayment
[**list_payments**](PaymentsApi.md#list_payments) | **GET** /v2/payments | ListPayments
[**update_payment**](PaymentsApi.md#update_payment) | **PUT** /v2/payments/{payment_id} | UpdatePayment



## cancel_payment

> models::CancelPaymentResponse cancel_payment(payment_id)
CancelPayment

Cancels (voids) a payment. You can use this endpoint to cancel a payment with  the APPROVED `status`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **String** | The ID of the payment to cancel. | [required] |

### Return type

[**models::CancelPaymentResponse**](CancelPaymentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_payment_by_idempotency_key

> models::CancelPaymentByIdempotencyKeyResponse cancel_payment_by_idempotency_key(cancel_payment_by_idempotency_key_request)
CancelPaymentByIdempotencyKey

Cancels (voids) a payment identified by the idempotency key that is specified in the request.  Use this method when the status of a `CreatePayment` request is unknown (for example, after you send a `CreatePayment` request, a network error occurs and you do not get a response). In this case, you can direct Square to cancel the payment using this endpoint. In the request, you provide the same idempotency key that you provided in your `CreatePayment` request that you want to cancel. After canceling the payment, you can submit your `CreatePayment` request again.  Note that if no payment with the specified idempotency key is found, no action is taken and the endpoint returns successfully.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cancel_payment_by_idempotency_key_request** | [**CancelPaymentByIdempotencyKeyRequest**](CancelPaymentByIdempotencyKeyRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CancelPaymentByIdempotencyKeyResponse**](CancelPaymentByIdempotencyKeyResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_payment

> models::CompletePaymentResponse complete_payment(payment_id, complete_payment_request)
CompletePayment

Completes (captures) a payment. By default, payments are set to complete immediately after they are created.  You can use this endpoint to complete a payment with the APPROVED `status`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **String** | The unique ID identifying the payment to be completed. | [required] |
**complete_payment_request** | [**CompletePaymentRequest**](CompletePaymentRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CompletePaymentResponse**](CompletePaymentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_payment

> models::CreatePaymentResponse create_payment(create_payment_request)
CreatePayment

Creates a payment using the provided source. You can use this endpoint  to charge a card (credit/debit card or     Square gift card) or record a payment that the seller received outside of Square  (cash payment from a buyer or a payment that an external entity  processed on behalf of the seller).  The endpoint creates a  `Payment` object and returns it in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payment_request** | [**CreatePaymentRequest**](CreatePaymentRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreatePaymentResponse**](CreatePaymentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment

> models::GetPaymentResponse get_payment(payment_id)
GetPayment

Retrieves details for a specific payment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **String** | A unique ID for the desired payment. | [required] |

### Return type

[**models::GetPaymentResponse**](GetPaymentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payments

> models::ListPaymentsResponse list_payments(begin_time, end_time, sort_order, cursor, location_id, total, last_4, card_brand, limit, is_offline_payment, offline_begin_time, offline_end_time, updated_at_begin_time, updated_at_end_time, sort_field)
ListPayments

Retrieves a list of payments taken by the account making the request.  Results are eventually consistent, and new payments or changes to payments might take several seconds to appear.  The maximum results per page is 100.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**begin_time** | Option<**String**> | Indicates the start of the time range to retrieve payments for, in RFC 3339 format. The range is determined using the `created_at` field for each Payment. Inclusive. Default: The current time minus one year. |  |
**end_time** | Option<**String**> | Indicates the end of the time range to retrieve payments for, in RFC 3339 format.  The range is determined using the `created_at` field for each Payment.  Default: The current time. |  |
**sort_order** | Option<**String**> | The order in which results are listed by `ListPaymentsRequest.sort_field`: - `ASC` - Oldest to newest. - `DESC` - Newest to oldest (default). |  |
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this cursor to retrieve the next set of results for the original query.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**location_id** | Option<**String**> | Limit results to the location supplied. By default, results are returned for the default (main) location associated with the seller. |  |
**total** | Option<**i64**> | The exact amount in the `total_money` for a payment. |  |
**last_4** | Option<**String**> | The last four digits of a payment card. |  |
**card_brand** | Option<**String**> | The brand of the payment card (for example, VISA). |  |
**limit** | Option<**i32**> | The maximum number of results to be returned in a single page. It is possible to receive fewer results than the specified limit on a given page.  The default value of 100 is also the maximum allowed value. If the provided value is  greater than 100, it is ignored and the default value is used instead.  Default: `100` |  |
**is_offline_payment** | Option<**bool**> | Whether the payment was taken offline or not. |  |[default to false]
**offline_begin_time** | Option<**String**> | Indicates the start of the time range for which to retrieve offline payments, in RFC 3339 format for timestamps. The range is determined using the `offline_payment_details.client_created_at` field for each Payment. If set, payments without a value set in `offline_payment_details.client_created_at` will not be returned.  Default: The current time. |  |
**offline_end_time** | Option<**String**> | Indicates the end of the time range for which to retrieve offline payments, in RFC 3339 format for timestamps. The range is determined using the `offline_payment_details.client_created_at` field for each Payment. If set, payments without a value set in `offline_payment_details.client_created_at` will not be returned.  Default: The current time. |  |
**updated_at_begin_time** | Option<**String**> | Indicates the start of the time range to retrieve payments for, in RFC 3339 format.  The range is determined using the `updated_at` field for each Payment. |  |
**updated_at_end_time** | Option<**String**> | Indicates the end of the time range to retrieve payments for, in RFC 3339 format.  The range is determined using the `updated_at` field for each Payment. |  |
**sort_field** | Option<[**ListPaymentsRequestSortField**](.md)> | The field used to sort results by. The default is `CREATED_AT`. |  |

### Return type

[**models::ListPaymentsResponse**](ListPaymentsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_payment

> models::UpdatePaymentResponse update_payment(payment_id, update_payment_request)
UpdatePayment

Updates a payment with the APPROVED status. You can update the `amount_money` and `tip_money` using this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **String** | The ID of the payment to update. | [required] |
**update_payment_request** | [**UpdatePaymentRequest**](UpdatePaymentRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdatePaymentResponse**](UpdatePaymentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

