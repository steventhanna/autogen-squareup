# \SubscriptionsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_swap_plan**](SubscriptionsApi.md#bulk_swap_plan) | **POST** /v2/subscriptions/bulk-swap-plan | BulkSwapPlan
[**cancel_subscription**](SubscriptionsApi.md#cancel_subscription) | **POST** /v2/subscriptions/{subscription_id}/cancel | CancelSubscription
[**change_billing_anchor_date**](SubscriptionsApi.md#change_billing_anchor_date) | **POST** /v2/subscriptions/{subscription_id}/billing-anchor | ChangeBillingAnchorDate
[**create_subscription**](SubscriptionsApi.md#create_subscription) | **POST** /v2/subscriptions | CreateSubscription
[**delete_subscription_action**](SubscriptionsApi.md#delete_subscription_action) | **DELETE** /v2/subscriptions/{subscription_id}/actions/{action_id} | DeleteSubscriptionAction
[**list_subscription_events**](SubscriptionsApi.md#list_subscription_events) | **GET** /v2/subscriptions/{subscription_id}/events | ListSubscriptionEvents
[**pause_subscription**](SubscriptionsApi.md#pause_subscription) | **POST** /v2/subscriptions/{subscription_id}/pause | PauseSubscription
[**resume_subscription**](SubscriptionsApi.md#resume_subscription) | **POST** /v2/subscriptions/{subscription_id}/resume | ResumeSubscription
[**retrieve_subscription**](SubscriptionsApi.md#retrieve_subscription) | **GET** /v2/subscriptions/{subscription_id} | RetrieveSubscription
[**search_subscriptions**](SubscriptionsApi.md#search_subscriptions) | **POST** /v2/subscriptions/search | SearchSubscriptions
[**swap_plan**](SubscriptionsApi.md#swap_plan) | **POST** /v2/subscriptions/{subscription_id}/swap-plan | SwapPlan
[**update_subscription**](SubscriptionsApi.md#update_subscription) | **PUT** /v2/subscriptions/{subscription_id} | UpdateSubscription



## bulk_swap_plan

> models::BulkSwapPlanResponse bulk_swap_plan(bulk_swap_plan_request)
BulkSwapPlan

Schedules a plan variation change for all active subscriptions under a given plan variation. For more information, see [Swap Subscription Plan Variations](https://developer.squareup.com/docs/subscriptions-api/swap-plan-variations).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_swap_plan_request** | [**BulkSwapPlanRequest**](BulkSwapPlanRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkSwapPlanResponse**](BulkSwapPlanResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_subscription

> models::CancelSubscriptionResponse cancel_subscription(subscription_id)
CancelSubscription

Schedules a `CANCEL` action to cancel an active subscription. This  sets the `canceled_date` field to the end of the active billing period. After this date,  the subscription status changes from ACTIVE to CANCELED.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | The ID of the subscription to cancel. | [required] |

### Return type

[**models::CancelSubscriptionResponse**](CancelSubscriptionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_billing_anchor_date

> models::ChangeBillingAnchorDateResponse change_billing_anchor_date(subscription_id, change_billing_anchor_date_request)
ChangeBillingAnchorDate

Changes the [billing anchor date](https://developer.squareup.com/docs/subscriptions-api/subscription-billing#billing-dates) for a subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | The ID of the subscription to update the billing anchor date. | [required] |
**change_billing_anchor_date_request** | [**ChangeBillingAnchorDateRequest**](ChangeBillingAnchorDateRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::ChangeBillingAnchorDateResponse**](ChangeBillingAnchorDateResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subscription

> models::CreateSubscriptionResponse create_subscription(create_subscription_request)
CreateSubscription

Enrolls a customer in a subscription.  If you provide a card on file in the request, Square charges the card for the subscription. Otherwise, Square sends an invoice to the customer's email address. The subscription starts immediately, unless the request includes the optional `start_date`. Each individual subscription is associated with a particular location.  For more information, see [Create a subscription](https://developer.squareup.com/docs/subscriptions-api/manage-subscriptions#create-a-subscription).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_subscription_request** | [**CreateSubscriptionRequest**](CreateSubscriptionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateSubscriptionResponse**](CreateSubscriptionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subscription_action

> models::DeleteSubscriptionActionResponse delete_subscription_action(subscription_id, action_id)
DeleteSubscriptionAction

Deletes a scheduled action for a subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | The ID of the subscription the targeted action is to act upon. | [required] |
**action_id** | **String** | The ID of the targeted action to be deleted. | [required] |

### Return type

[**models::DeleteSubscriptionActionResponse**](DeleteSubscriptionActionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_subscription_events

> models::ListSubscriptionEventsResponse list_subscription_events(subscription_id, cursor, limit)
ListSubscriptionEvents

Lists all [events](https://developer.squareup.com/docs/subscriptions-api/actions-events) for a specific subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | The ID of the subscription to retrieve the events for. | [required] |
**cursor** | Option<**String**> | When the total number of resulting subscription events exceeds the limit of a paged response,  specify the cursor returned from a preceding response here to fetch the next set of results. If the cursor is unset, the response contains the last page of the results.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**limit** | Option<**i32**> | The upper limit on the number of subscription events to return in a paged response. |  |

### Return type

[**models::ListSubscriptionEventsResponse**](ListSubscriptionEventsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pause_subscription

> models::PauseSubscriptionResponse pause_subscription(subscription_id, pause_subscription_request)
PauseSubscription

Schedules a `PAUSE` action to pause an active subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | The ID of the subscription to pause. | [required] |
**pause_subscription_request** | [**PauseSubscriptionRequest**](PauseSubscriptionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::PauseSubscriptionResponse**](PauseSubscriptionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_subscription

> models::ResumeSubscriptionResponse resume_subscription(subscription_id, resume_subscription_request)
ResumeSubscription

Schedules a `RESUME` action to resume a paused or a deactivated subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | The ID of the subscription to resume. | [required] |
**resume_subscription_request** | [**ResumeSubscriptionRequest**](ResumeSubscriptionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::ResumeSubscriptionResponse**](ResumeSubscriptionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_subscription

> models::RetrieveSubscriptionResponse retrieve_subscription(subscription_id, include)
RetrieveSubscription

Retrieves a specific subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | The ID of the subscription to retrieve. | [required] |
**include** | Option<**String**> | A query parameter to specify related information to be included in the response.   The supported query parameter values are:   - `actions`: to include scheduled actions on the targeted subscription. |  |

### Return type

[**models::RetrieveSubscriptionResponse**](RetrieveSubscriptionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_subscriptions

> models::SearchSubscriptionsResponse search_subscriptions(search_subscriptions_request)
SearchSubscriptions

Searches for subscriptions.  Results are ordered chronologically by subscription creation date. If the request specifies more than one location ID, the endpoint orders the result by location ID, and then by creation date within each location. If no locations are given in the query, all locations are searched.  You can also optionally specify `customer_ids` to search by customer. If left unset, all customers associated with the specified locations are returned. If the request specifies customer IDs, the endpoint orders results first by location, within location by customer ID, and within customer by subscription creation date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_subscriptions_request** | [**SearchSubscriptionsRequest**](SearchSubscriptionsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchSubscriptionsResponse**](SearchSubscriptionsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swap_plan

> models::SwapPlanResponse swap_plan(subscription_id, swap_plan_request)
SwapPlan

Schedules a `SWAP_PLAN` action to swap a subscription plan variation in an existing subscription.  For more information, see [Swap Subscription Plan Variations](https://developer.squareup.com/docs/subscriptions-api/swap-plan-variations).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | The ID of the subscription to swap the subscription plan for. | [required] |
**swap_plan_request** | [**SwapPlanRequest**](SwapPlanRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SwapPlanResponse**](SwapPlanResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subscription

> models::UpdateSubscriptionResponse update_subscription(subscription_id, update_subscription_request)
UpdateSubscription

Updates a subscription by modifying or clearing `subscription` field values. To clear a field, set its value to `null`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** | The ID of the subscription to update. | [required] |
**update_subscription_request** | [**UpdateSubscriptionRequest**](UpdateSubscriptionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateSubscriptionResponse**](UpdateSubscriptionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

