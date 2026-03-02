# \WebhookSubscriptionsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook_subscription**](WebhookSubscriptionsApi.md#create_webhook_subscription) | **POST** /v2/webhooks/subscriptions | CreateWebhookSubscription
[**delete_webhook_subscription**](WebhookSubscriptionsApi.md#delete_webhook_subscription) | **DELETE** /v2/webhooks/subscriptions/{subscription_id} | DeleteWebhookSubscription
[**list_webhook_event_types**](WebhookSubscriptionsApi.md#list_webhook_event_types) | **GET** /v2/webhooks/event-types | ListWebhookEventTypes
[**list_webhook_subscriptions**](WebhookSubscriptionsApi.md#list_webhook_subscriptions) | **GET** /v2/webhooks/subscriptions | ListWebhookSubscriptions
[**retrieve_webhook_subscription**](WebhookSubscriptionsApi.md#retrieve_webhook_subscription) | **GET** /v2/webhooks/subscriptions/{subscription_id} | RetrieveWebhookSubscription
[**test_webhook_subscription**](WebhookSubscriptionsApi.md#test_webhook_subscription) | **POST** /v2/webhooks/subscriptions/{subscription_id}/test | TestWebhookSubscription
[**update_webhook_subscription**](WebhookSubscriptionsApi.md#update_webhook_subscription) | **PUT** /v2/webhooks/subscriptions/{subscription_id} | UpdateWebhookSubscription
[**update_webhook_subscription_signature_key**](WebhookSubscriptionsApi.md#update_webhook_subscription_signature_key) | **POST** /v2/webhooks/subscriptions/{subscription_id}/signature-key | UpdateWebhookSubscriptionSignatureKey



## create_webhook_subscription

> models::CreateWebhookSubscriptionResponse create_webhook_subscription(create_webhook_subscription_request)
CreateWebhookSubscription

Creates a webhook subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_webhook_subscription_request** | [**CreateWebhookSubscriptionRequest**](CreateWebhookSubscriptionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateWebhookSubscriptionResponse**](CreateWebhookSubscriptionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook_subscription

> models::DeleteWebhookSubscriptionResponse delete_webhook_subscription(subscription_id)
DeleteWebhookSubscription

Deletes a webhook subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | [REQUIRED] The ID of the [Subscription](entity:WebhookSubscription) to delete. | [required] |

### Return type

[**models::DeleteWebhookSubscriptionResponse**](DeleteWebhookSubscriptionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_webhook_event_types

> models::ListWebhookEventTypesResponse list_webhook_event_types(api_version)
ListWebhookEventTypes

Lists all webhook event types that can be subscribed to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | Option<**String**> | The API version for which to list event types. Setting this field overrides the default version used by the application. |  |

### Return type

[**models::ListWebhookEventTypesResponse**](ListWebhookEventTypesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_webhook_subscriptions

> models::ListWebhookSubscriptionsResponse list_webhook_subscriptions(cursor, include_disabled, sort_order, limit)
ListWebhookSubscriptions

Lists all webhook subscriptions owned by your application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this to retrieve the next set of results for your original query.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**include_disabled** | Option<**bool**> | Includes disabled [Subscription](entity:WebhookSubscription)s. By default, all enabled [Subscription](entity:WebhookSubscription)s are returned. |  |[default to false]
**sort_order** | Option<[**SortOrder**](.md)> | Sorts the returned list by when the [Subscription](entity:WebhookSubscription) was created with the specified order. This field defaults to ASC. |  |
**limit** | Option<**i32**> | The maximum number of results to be returned in a single page. It is possible to receive fewer results than the specified limit on a given page. The default value of 100 is also the maximum allowed value.  Default: 100 |  |

### Return type

[**models::ListWebhookSubscriptionsResponse**](ListWebhookSubscriptionsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_webhook_subscription

> models::RetrieveWebhookSubscriptionResponse retrieve_webhook_subscription(subscription_id)
RetrieveWebhookSubscription

Retrieves a webhook subscription identified by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | [REQUIRED] The ID of the [Subscription](entity:WebhookSubscription) to retrieve. | [required] |

### Return type

[**models::RetrieveWebhookSubscriptionResponse**](RetrieveWebhookSubscriptionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_webhook_subscription

> models::TestWebhookSubscriptionResponse test_webhook_subscription(subscription_id, test_webhook_subscription_request)
TestWebhookSubscription

Tests a webhook subscription by sending a test event to the notification URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | [REQUIRED] The ID of the [Subscription](entity:WebhookSubscription) to test. | [required] |
**test_webhook_subscription_request** | [**TestWebhookSubscriptionRequest**](TestWebhookSubscriptionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::TestWebhookSubscriptionResponse**](TestWebhookSubscriptionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook_subscription

> models::UpdateWebhookSubscriptionResponse update_webhook_subscription(subscription_id, update_webhook_subscription_request)
UpdateWebhookSubscription

Updates a webhook subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | [REQUIRED] The ID of the [Subscription](entity:WebhookSubscription) to update. | [required] |
**update_webhook_subscription_request** | [**UpdateWebhookSubscriptionRequest**](UpdateWebhookSubscriptionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateWebhookSubscriptionResponse**](UpdateWebhookSubscriptionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook_subscription_signature_key

> models::UpdateWebhookSubscriptionSignatureKeyResponse update_webhook_subscription_signature_key(subscription_id, update_webhook_subscription_signature_key_request)
UpdateWebhookSubscriptionSignatureKey

Updates a webhook subscription by replacing the existing signature key with a new one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | [REQUIRED] The ID of the [Subscription](entity:WebhookSubscription) to update. | [required] |
**update_webhook_subscription_signature_key_request** | [**UpdateWebhookSubscriptionSignatureKeyRequest**](UpdateWebhookSubscriptionSignatureKeyRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateWebhookSubscriptionSignatureKeyResponse**](UpdateWebhookSubscriptionSignatureKeyResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

