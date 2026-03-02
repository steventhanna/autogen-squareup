# \TerminalApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_terminal_action**](TerminalApi.md#cancel_terminal_action) | **POST** /v2/terminals/actions/{action_id}/cancel | CancelTerminalAction
[**cancel_terminal_checkout**](TerminalApi.md#cancel_terminal_checkout) | **POST** /v2/terminals/checkouts/{checkout_id}/cancel | CancelTerminalCheckout
[**cancel_terminal_refund**](TerminalApi.md#cancel_terminal_refund) | **POST** /v2/terminals/refunds/{terminal_refund_id}/cancel | CancelTerminalRefund
[**create_terminal_action**](TerminalApi.md#create_terminal_action) | **POST** /v2/terminals/actions | CreateTerminalAction
[**create_terminal_checkout**](TerminalApi.md#create_terminal_checkout) | **POST** /v2/terminals/checkouts | CreateTerminalCheckout
[**create_terminal_refund**](TerminalApi.md#create_terminal_refund) | **POST** /v2/terminals/refunds | CreateTerminalRefund
[**dismiss_terminal_action**](TerminalApi.md#dismiss_terminal_action) | **POST** /v2/terminals/actions/{action_id}/dismiss | DismissTerminalAction
[**dismiss_terminal_checkout**](TerminalApi.md#dismiss_terminal_checkout) | **POST** /v2/terminals/checkouts/{checkout_id}/dismiss | DismissTerminalCheckout
[**dismiss_terminal_refund**](TerminalApi.md#dismiss_terminal_refund) | **POST** /v2/terminals/refunds/{terminal_refund_id}/dismiss | DismissTerminalRefund
[**get_terminal_action**](TerminalApi.md#get_terminal_action) | **GET** /v2/terminals/actions/{action_id} | GetTerminalAction
[**get_terminal_checkout**](TerminalApi.md#get_terminal_checkout) | **GET** /v2/terminals/checkouts/{checkout_id} | GetTerminalCheckout
[**get_terminal_refund**](TerminalApi.md#get_terminal_refund) | **GET** /v2/terminals/refunds/{terminal_refund_id} | GetTerminalRefund
[**search_terminal_actions**](TerminalApi.md#search_terminal_actions) | **POST** /v2/terminals/actions/search | SearchTerminalActions
[**search_terminal_checkouts**](TerminalApi.md#search_terminal_checkouts) | **POST** /v2/terminals/checkouts/search | SearchTerminalCheckouts
[**search_terminal_refunds**](TerminalApi.md#search_terminal_refunds) | **POST** /v2/terminals/refunds/search | SearchTerminalRefunds



## cancel_terminal_action

> models::CancelTerminalActionResponse cancel_terminal_action(action_id)
CancelTerminalAction

Cancels a Terminal action request if the status of the request permits it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | Unique ID for the desired `TerminalAction`. | [required] |

### Return type

[**models::CancelTerminalActionResponse**](CancelTerminalActionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_terminal_checkout

> models::CancelTerminalCheckoutResponse cancel_terminal_checkout(checkout_id)
CancelTerminalCheckout

Cancels a Terminal checkout request if the status of the request permits it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**checkout_id** | **String** | The unique ID for the desired `TerminalCheckout`. | [required] |

### Return type

[**models::CancelTerminalCheckoutResponse**](CancelTerminalCheckoutResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_terminal_refund

> models::CancelTerminalRefundResponse cancel_terminal_refund(terminal_refund_id)
CancelTerminalRefund

Cancels an Interac Terminal refund request by refund request ID if the status of the request permits it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terminal_refund_id** | **String** | The unique ID for the desired `TerminalRefund`. | [required] |

### Return type

[**models::CancelTerminalRefundResponse**](CancelTerminalRefundResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_terminal_action

> models::CreateTerminalActionResponse create_terminal_action(create_terminal_action_request)
CreateTerminalAction

Creates a Terminal action request and sends it to the specified device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_terminal_action_request** | [**CreateTerminalActionRequest**](CreateTerminalActionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateTerminalActionResponse**](CreateTerminalActionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_terminal_checkout

> models::CreateTerminalCheckoutResponse create_terminal_checkout(create_terminal_checkout_request)
CreateTerminalCheckout

Creates a Terminal checkout request and sends it to the specified device to take a payment for the requested amount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_terminal_checkout_request** | [**CreateTerminalCheckoutRequest**](CreateTerminalCheckoutRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateTerminalCheckoutResponse**](CreateTerminalCheckoutResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_terminal_refund

> models::CreateTerminalRefundResponse create_terminal_refund(create_terminal_refund_request)
CreateTerminalRefund

Creates a request to refund an Interac payment completed on a Square Terminal. Refunds for Interac payments on a Square Terminal are supported only for Interac debit cards in Canada. Other refunds for Terminal payments should use the Refunds API. For more information, see [Refunds API](api:Refunds).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_terminal_refund_request** | [**CreateTerminalRefundRequest**](CreateTerminalRefundRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateTerminalRefundResponse**](CreateTerminalRefundResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dismiss_terminal_action

> models::DismissTerminalActionResponse dismiss_terminal_action(action_id)
DismissTerminalAction

Dismisses a Terminal action request if the status and type of the request permits it.  See [Link and Dismiss Actions](https://developer.squareup.com/docs/terminal-api/advanced-features/custom-workflows/link-and-dismiss-actions) for more details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | Unique ID for the `TerminalAction` associated with the action to be dismissed. | [required] |

### Return type

[**models::DismissTerminalActionResponse**](DismissTerminalActionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dismiss_terminal_checkout

> models::DismissTerminalCheckoutResponse dismiss_terminal_checkout(checkout_id)
DismissTerminalCheckout

Dismisses a Terminal checkout request if the status and type of the request permits it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**checkout_id** | **String** | Unique ID for the `TerminalCheckout` associated with the checkout to be dismissed. | [required] |

### Return type

[**models::DismissTerminalCheckoutResponse**](DismissTerminalCheckoutResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dismiss_terminal_refund

> models::DismissTerminalRefundResponse dismiss_terminal_refund(terminal_refund_id)
DismissTerminalRefund

Dismisses a Terminal refund request if the status and type of the request permits it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terminal_refund_id** | **String** | Unique ID for the `TerminalRefund` associated with the refund to be dismissed. | [required] |

### Return type

[**models::DismissTerminalRefundResponse**](DismissTerminalRefundResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_terminal_action

> models::GetTerminalActionResponse get_terminal_action(action_id)
GetTerminalAction

Retrieves a Terminal action request by `action_id`. Terminal action requests are available for 30 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | Unique ID for the desired `TerminalAction`. | [required] |

### Return type

[**models::GetTerminalActionResponse**](GetTerminalActionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_terminal_checkout

> models::GetTerminalCheckoutResponse get_terminal_checkout(checkout_id)
GetTerminalCheckout

Retrieves a Terminal checkout request by `checkout_id`. Terminal checkout requests are available for 30 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**checkout_id** | **String** | The unique ID for the desired `TerminalCheckout`. | [required] |

### Return type

[**models::GetTerminalCheckoutResponse**](GetTerminalCheckoutResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_terminal_refund

> models::GetTerminalRefundResponse get_terminal_refund(terminal_refund_id)
GetTerminalRefund

Retrieves an Interac Terminal refund object by ID. Terminal refund objects are available for 30 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terminal_refund_id** | **String** | The unique ID for the desired `TerminalRefund`. | [required] |

### Return type

[**models::GetTerminalRefundResponse**](GetTerminalRefundResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_terminal_actions

> models::SearchTerminalActionsResponse search_terminal_actions(search_terminal_actions_request)
SearchTerminalActions

Retrieves a filtered list of Terminal action requests created by the account making the request. Terminal action requests are available for 30 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_terminal_actions_request** | [**SearchTerminalActionsRequest**](SearchTerminalActionsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchTerminalActionsResponse**](SearchTerminalActionsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_terminal_checkouts

> models::SearchTerminalCheckoutsResponse search_terminal_checkouts(search_terminal_checkouts_request)
SearchTerminalCheckouts

Returns a filtered list of Terminal checkout requests created by the application making the request. Only Terminal checkout requests created for the merchant scoped to the OAuth token are returned. Terminal checkout requests are available for 30 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_terminal_checkouts_request** | [**SearchTerminalCheckoutsRequest**](SearchTerminalCheckoutsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchTerminalCheckoutsResponse**](SearchTerminalCheckoutsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_terminal_refunds

> models::SearchTerminalRefundsResponse search_terminal_refunds(search_terminal_refunds_request)
SearchTerminalRefunds

Retrieves a filtered list of Interac Terminal refund requests created by the seller making the request. Terminal refund requests are available for 30 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_terminal_refunds_request** | [**SearchTerminalRefundsRequest**](SearchTerminalRefundsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchTerminalRefundsResponse**](SearchTerminalRefundsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

