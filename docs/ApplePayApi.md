# \ApplePayApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**register_domain**](ApplePayApi.md#register_domain) | **POST** /v2/apple-pay/domains | RegisterDomain



## register_domain

> models::RegisterDomainResponse register_domain(register_domain_request)
RegisterDomain

Activates a domain for use with Apple Pay on the Web and Square. A validation is performed on this domain by Apple to ensure that it is properly set up as an Apple Pay enabled domain.  This endpoint provides an easy way for platform developers to bulk activate Apple Pay on the Web with Square for merchants using their platform.  Note: You will need to host a valid domain verification file on your domain to support Apple Pay.  The current version of this file is always available at https://app.squareup.com/digital-wallets/apple-pay/apple-developer-merchantid-domain-association, and should be hosted at `.well_known/apple-developer-merchantid-domain-association` on your domain.  This file is subject to change; we strongly recommend checking for updates regularly and avoiding long-lived caches that might not keep in sync with the correct file version.  To learn more about the Web Payments SDK and how to add Apple Pay, see [Take an Apple Pay Payment](https://developer.squareup.com/docs/web-payments/apple-pay).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_domain_request** | [**RegisterDomainRequest**](RegisterDomainRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::RegisterDomainResponse**](RegisterDomainResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

