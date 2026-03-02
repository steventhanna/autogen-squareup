# \CheckoutApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_checkout**](CheckoutApi.md#create_checkout) | **POST** /v2/locations/{location_id}/checkouts | CreateCheckout
[**create_payment_link**](CheckoutApi.md#create_payment_link) | **POST** /v2/online-checkout/payment-links | CreatePaymentLink
[**delete_payment_link**](CheckoutApi.md#delete_payment_link) | **DELETE** /v2/online-checkout/payment-links/{id} | DeletePaymentLink
[**list_payment_links**](CheckoutApi.md#list_payment_links) | **GET** /v2/online-checkout/payment-links | ListPaymentLinks
[**retrieve_location_settings**](CheckoutApi.md#retrieve_location_settings) | **GET** /v2/online-checkout/location-settings/{location_id} | RetrieveLocationSettings
[**retrieve_merchant_settings**](CheckoutApi.md#retrieve_merchant_settings) | **GET** /v2/online-checkout/merchant-settings | RetrieveMerchantSettings
[**retrieve_payment_link**](CheckoutApi.md#retrieve_payment_link) | **GET** /v2/online-checkout/payment-links/{id} | RetrievePaymentLink
[**update_location_settings**](CheckoutApi.md#update_location_settings) | **PUT** /v2/online-checkout/location-settings/{location_id} | UpdateLocationSettings
[**update_merchant_settings**](CheckoutApi.md#update_merchant_settings) | **PUT** /v2/online-checkout/merchant-settings | UpdateMerchantSettings
[**update_payment_link**](CheckoutApi.md#update_payment_link) | **PUT** /v2/online-checkout/payment-links/{id} | UpdatePaymentLink



## create_checkout

> models::CreateCheckoutResponse create_checkout(location_id, create_checkout_request)
CreateCheckout

Links a `checkoutId` to a `checkout_page_url` that customers are directed to in order to provide their payment information using a payment processing workflow hosted on connect.squareup.com.    NOTE: The Checkout API has been updated with new features.  For more information, see [Checkout API highlights](https://developer.squareup.com/docs/checkout-api#checkout-api-highlights).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the business location to associate the checkout with. | [required] |
**create_checkout_request** | [**CreateCheckoutRequest**](CreateCheckoutRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateCheckoutResponse**](CreateCheckoutResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_payment_link

> models::CreatePaymentLinkResponse create_payment_link(create_payment_link_request)
CreatePaymentLink

Creates a Square-hosted checkout page. Applications can share the resulting payment link with their buyer to pay for goods and services.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payment_link_request** | [**CreatePaymentLinkRequest**](CreatePaymentLinkRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreatePaymentLinkResponse**](CreatePaymentLinkResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_payment_link

> models::DeletePaymentLinkResponse delete_payment_link(id)
DeletePaymentLink

Deletes a payment link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the payment link to delete. | [required] |

### Return type

[**models::DeletePaymentLinkResponse**](DeletePaymentLinkResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payment_links

> models::ListPaymentLinksResponse list_payment_links(cursor, limit)
ListPaymentLinks

Lists all payment links.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this cursor to retrieve the next set of results for the original query. If a cursor is not provided, the endpoint returns the first page of the results. For more  information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**limit** | Option<**i32**> | A limit on the number of results to return per page. The limit is advisory and the implementation might return more or less results. If the supplied limit is negative, zero, or greater than the maximum limit of 1000, it is ignored.  Default value: `100` |  |

### Return type

[**models::ListPaymentLinksResponse**](ListPaymentLinksResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_location_settings

> models::RetrieveLocationSettingsResponse retrieve_location_settings(location_id)
RetrieveLocationSettings

Retrieves the location-level settings for a Square-hosted checkout page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the location for which to retrieve settings. | [required] |

### Return type

[**models::RetrieveLocationSettingsResponse**](RetrieveLocationSettingsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_merchant_settings

> models::RetrieveMerchantSettingsResponse retrieve_merchant_settings()
RetrieveMerchantSettings

Retrieves the merchant-level settings for a Square-hosted checkout page.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RetrieveMerchantSettingsResponse**](RetrieveMerchantSettingsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_payment_link

> models::RetrievePaymentLinkResponse retrieve_payment_link(id)
RetrievePaymentLink

Retrieves a payment link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of link to retrieve. | [required] |

### Return type

[**models::RetrievePaymentLinkResponse**](RetrievePaymentLinkResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_location_settings

> models::UpdateLocationSettingsResponse update_location_settings(location_id, update_location_settings_request)
UpdateLocationSettings

Updates the location-level settings for a Square-hosted checkout page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the location for which to retrieve settings. | [required] |
**update_location_settings_request** | [**UpdateLocationSettingsRequest**](UpdateLocationSettingsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateLocationSettingsResponse**](UpdateLocationSettingsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_merchant_settings

> models::UpdateMerchantSettingsResponse update_merchant_settings(update_merchant_settings_request)
UpdateMerchantSettings

Updates the merchant-level settings for a Square-hosted checkout page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_merchant_settings_request** | [**UpdateMerchantSettingsRequest**](UpdateMerchantSettingsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateMerchantSettingsResponse**](UpdateMerchantSettingsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_payment_link

> models::UpdatePaymentLinkResponse update_payment_link(id, update_payment_link_request)
UpdatePaymentLink

Updates a payment link. You can update the `payment_link` fields such as `description`, `checkout_options`, and  `pre_populated_data`. You cannot update other fields such as the `order_id`, `version`, `URL`, or `timestamp` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the payment link to update. | [required] |
**update_payment_link_request** | [**UpdatePaymentLinkRequest**](UpdatePaymentLinkRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdatePaymentLinkResponse**](UpdatePaymentLinkResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

