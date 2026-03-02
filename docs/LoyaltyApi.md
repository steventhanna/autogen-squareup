# \LoyaltyApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accumulate_loyalty_points**](LoyaltyApi.md#accumulate_loyalty_points) | **POST** /v2/loyalty/accounts/{account_id}/accumulate | AccumulateLoyaltyPoints
[**adjust_loyalty_points**](LoyaltyApi.md#adjust_loyalty_points) | **POST** /v2/loyalty/accounts/{account_id}/adjust | AdjustLoyaltyPoints
[**calculate_loyalty_points**](LoyaltyApi.md#calculate_loyalty_points) | **POST** /v2/loyalty/programs/{program_id}/calculate | CalculateLoyaltyPoints
[**cancel_loyalty_promotion**](LoyaltyApi.md#cancel_loyalty_promotion) | **POST** /v2/loyalty/programs/{program_id}/promotions/{promotion_id}/cancel | CancelLoyaltyPromotion
[**create_loyalty_account**](LoyaltyApi.md#create_loyalty_account) | **POST** /v2/loyalty/accounts | CreateLoyaltyAccount
[**create_loyalty_promotion**](LoyaltyApi.md#create_loyalty_promotion) | **POST** /v2/loyalty/programs/{program_id}/promotions | CreateLoyaltyPromotion
[**create_loyalty_reward**](LoyaltyApi.md#create_loyalty_reward) | **POST** /v2/loyalty/rewards | CreateLoyaltyReward
[**delete_loyalty_reward**](LoyaltyApi.md#delete_loyalty_reward) | **DELETE** /v2/loyalty/rewards/{reward_id} | DeleteLoyaltyReward
[**list_loyalty_programs**](LoyaltyApi.md#list_loyalty_programs) | **GET** /v2/loyalty/programs | ListLoyaltyPrograms
[**list_loyalty_promotions**](LoyaltyApi.md#list_loyalty_promotions) | **GET** /v2/loyalty/programs/{program_id}/promotions | ListLoyaltyPromotions
[**redeem_loyalty_reward**](LoyaltyApi.md#redeem_loyalty_reward) | **POST** /v2/loyalty/rewards/{reward_id}/redeem | RedeemLoyaltyReward
[**retrieve_loyalty_account**](LoyaltyApi.md#retrieve_loyalty_account) | **GET** /v2/loyalty/accounts/{account_id} | RetrieveLoyaltyAccount
[**retrieve_loyalty_program**](LoyaltyApi.md#retrieve_loyalty_program) | **GET** /v2/loyalty/programs/{program_id} | RetrieveLoyaltyProgram
[**retrieve_loyalty_promotion**](LoyaltyApi.md#retrieve_loyalty_promotion) | **GET** /v2/loyalty/programs/{program_id}/promotions/{promotion_id} | RetrieveLoyaltyPromotion
[**retrieve_loyalty_reward**](LoyaltyApi.md#retrieve_loyalty_reward) | **GET** /v2/loyalty/rewards/{reward_id} | RetrieveLoyaltyReward
[**search_loyalty_accounts**](LoyaltyApi.md#search_loyalty_accounts) | **POST** /v2/loyalty/accounts/search | SearchLoyaltyAccounts
[**search_loyalty_events**](LoyaltyApi.md#search_loyalty_events) | **POST** /v2/loyalty/events/search | SearchLoyaltyEvents
[**search_loyalty_rewards**](LoyaltyApi.md#search_loyalty_rewards) | **POST** /v2/loyalty/rewards/search | SearchLoyaltyRewards



## accumulate_loyalty_points

> models::AccumulateLoyaltyPointsResponse accumulate_loyalty_points(account_id, accumulate_loyalty_points_request)
AccumulateLoyaltyPoints

Adds points earned from a purchase to a [loyalty account](entity:LoyaltyAccount).  - If you are using the Orders API to manage orders, provide the `order_id`. Square reads the order to compute the points earned from both the base loyalty program and an associated [loyalty promotion](entity:LoyaltyPromotion). For purchases that qualify for multiple accrual rules, Square computes points based on the accrual rule that grants the most points. For purchases that qualify for multiple promotions, Square computes points based on the most recently created promotion. A purchase must first qualify for program points to be eligible for promotion points.  - If you are not using the Orders API to manage orders, provide `points` with the number of points to add. You must first perform a client-side computation of the points earned from the loyalty program and loyalty promotion. For spend-based and visit-based programs, you can call [CalculateLoyaltyPoints](api-endpoint:Loyalty-CalculateLoyaltyPoints) to compute the points earned from the base loyalty program. For information about computing points earned from a loyalty promotion, see [Calculating promotion points](https://developer.squareup.com/docs/loyalty-api/loyalty-promotions#calculate-promotion-points).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the target [loyalty account](entity:LoyaltyAccount). | [required] |
**accumulate_loyalty_points_request** | [**AccumulateLoyaltyPointsRequest**](AccumulateLoyaltyPointsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::AccumulateLoyaltyPointsResponse**](AccumulateLoyaltyPointsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## adjust_loyalty_points

> models::AdjustLoyaltyPointsResponse adjust_loyalty_points(account_id, adjust_loyalty_points_request)
AdjustLoyaltyPoints

Adds points to or subtracts points from a buyer's account.  Use this endpoint only when you need to manually adjust points. Otherwise, in your application flow, you call [AccumulateLoyaltyPoints](api-endpoint:Loyalty-AccumulateLoyaltyPoints) to add points when a buyer pays for the purchase.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the target [loyalty account](entity:LoyaltyAccount). | [required] |
**adjust_loyalty_points_request** | [**AdjustLoyaltyPointsRequest**](AdjustLoyaltyPointsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::AdjustLoyaltyPointsResponse**](AdjustLoyaltyPointsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calculate_loyalty_points

> models::CalculateLoyaltyPointsResponse calculate_loyalty_points(program_id, calculate_loyalty_points_request)
CalculateLoyaltyPoints

Calculates the number of points a buyer can earn from a purchase. Applications might call this endpoint to display the points to the buyer.  - If you are using the Orders API to manage orders, provide the `order_id` and (optional) `loyalty_account_id`. Square reads the order to compute the points earned from the base loyalty program and an associated [loyalty promotion](entity:LoyaltyPromotion).  - If you are not using the Orders API to manage orders, provide `transaction_amount_money` with the purchase amount. Square uses this amount to calculate the points earned from the base loyalty program, but not points earned from a loyalty promotion. For spend-based and visit-based programs, the `tax_mode` setting of the accrual rule indicates how taxes should be treated for loyalty points accrual. If the purchase qualifies for program points, call [ListLoyaltyPromotions](api-endpoint:Loyalty-ListLoyaltyPromotions) and perform a client-side computation to calculate whether the purchase also qualifies for promotion points. For more information, see [Calculating promotion points](https://developer.squareup.com/docs/loyalty-api/loyalty-promotions#calculate-promotion-points).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**program_id** | **String** | The ID of the [loyalty program](entity:LoyaltyProgram), which defines the rules for accruing points. | [required] |
**calculate_loyalty_points_request** | [**CalculateLoyaltyPointsRequest**](CalculateLoyaltyPointsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CalculateLoyaltyPointsResponse**](CalculateLoyaltyPointsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_loyalty_promotion

> models::CancelLoyaltyPromotionResponse cancel_loyalty_promotion(promotion_id, program_id)
CancelLoyaltyPromotion

Cancels a loyalty promotion. Use this endpoint to cancel an `ACTIVE` promotion earlier than the end date, cancel an `ACTIVE` promotion when an end date is not specified, or cancel a `SCHEDULED` promotion. Because updating a promotion is not supported, you can also use this endpoint to cancel a promotion before you create a new one.  This endpoint sets the loyalty promotion to the `CANCELED` state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**promotion_id** | **String** | The ID of the [loyalty promotion](entity:LoyaltyPromotion) to cancel. You can cancel a promotion that has an `ACTIVE` or `SCHEDULED` status. | [required] |
**program_id** | **String** | The ID of the base [loyalty program](entity:LoyaltyProgram). | [required] |

### Return type

[**models::CancelLoyaltyPromotionResponse**](CancelLoyaltyPromotionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_loyalty_account

> models::CreateLoyaltyAccountResponse create_loyalty_account(create_loyalty_account_request)
CreateLoyaltyAccount

Creates a loyalty account. To create a loyalty account, you must provide the `program_id` and a `mapping` with the `phone_number` of the buyer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_loyalty_account_request** | [**CreateLoyaltyAccountRequest**](CreateLoyaltyAccountRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateLoyaltyAccountResponse**](CreateLoyaltyAccountResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_loyalty_promotion

> models::CreateLoyaltyPromotionResponse create_loyalty_promotion(program_id, create_loyalty_promotion_request)
CreateLoyaltyPromotion

Creates a loyalty promotion for a [loyalty program](entity:LoyaltyProgram). A loyalty promotion enables buyers to earn points in addition to those earned from the base loyalty program.  This endpoint sets the loyalty promotion to the `ACTIVE` or `SCHEDULED` status, depending on the `available_time` setting. A loyalty program can have a maximum of 10 loyalty promotions with an `ACTIVE` or `SCHEDULED` status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**program_id** | **String** | The ID of the [loyalty program](entity:LoyaltyProgram) to associate with the promotion. To get the program ID, call [RetrieveLoyaltyProgram](api-endpoint:Loyalty-RetrieveLoyaltyProgram) using the `main` keyword. | [required] |
**create_loyalty_promotion_request** | [**CreateLoyaltyPromotionRequest**](CreateLoyaltyPromotionRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateLoyaltyPromotionResponse**](CreateLoyaltyPromotionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_loyalty_reward

> models::CreateLoyaltyRewardResponse create_loyalty_reward(create_loyalty_reward_request)
CreateLoyaltyReward

Creates a loyalty reward. In the process, the endpoint does following:  - Uses the `reward_tier_id` in the request to determine the number of points to lock for this reward. - If the request includes `order_id`, it adds the reward and related discount to the order.  After a reward is created, the points are locked and not available for the buyer to redeem another reward.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_loyalty_reward_request** | [**CreateLoyaltyRewardRequest**](CreateLoyaltyRewardRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateLoyaltyRewardResponse**](CreateLoyaltyRewardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_loyalty_reward

> models::DeleteLoyaltyRewardResponse delete_loyalty_reward(reward_id)
DeleteLoyaltyReward

Deletes a loyalty reward by doing the following:  - Returns the loyalty points back to the loyalty account. - If an order ID was specified when the reward was created (see [CreateLoyaltyReward](api-endpoint:Loyalty-CreateLoyaltyReward)), it updates the order by removing the reward and related discounts.  You cannot delete a reward that has reached the terminal state (REDEEMED).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reward_id** | **String** | The ID of the [loyalty reward](entity:LoyaltyReward) to delete. | [required] |

### Return type

[**models::DeleteLoyaltyRewardResponse**](DeleteLoyaltyRewardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_loyalty_programs

> models::ListLoyaltyProgramsResponse list_loyalty_programs()
ListLoyaltyPrograms

Returns a list of loyalty programs in the seller's account. Loyalty programs define how buyers can earn points and redeem points for rewards. Square sellers can have only one loyalty program, which is created and managed from the Seller Dashboard. For more information, see [Loyalty Program Overview](https://developer.squareup.com/docs/loyalty/overview).   Replaced with [RetrieveLoyaltyProgram](api-endpoint:Loyalty-RetrieveLoyaltyProgram) when used with the keyword `main`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListLoyaltyProgramsResponse**](ListLoyaltyProgramsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_loyalty_promotions

> models::ListLoyaltyPromotionsResponse list_loyalty_promotions(program_id, status, cursor, limit)
ListLoyaltyPromotions

Lists the loyalty promotions associated with a [loyalty program](entity:LoyaltyProgram). Results are sorted by the `created_at` date in descending order (newest to oldest).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**program_id** | **String** | The ID of the base [loyalty program](entity:LoyaltyProgram). To get the program ID, call [RetrieveLoyaltyProgram](api-endpoint:Loyalty-RetrieveLoyaltyProgram) using the `main` keyword. | [required] |
**status** | Option<[**LoyaltyPromotionStatus**](.md)> | The status to filter the results by. If a status is provided, only loyalty promotions with the specified status are returned. Otherwise, all loyalty promotions associated with the loyalty program are returned. |  |
**cursor** | Option<**String**> | The cursor returned in the paged response from the previous call to this endpoint. Provide this cursor to retrieve the next page of results for your original request. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**limit** | Option<**i32**> | The maximum number of results to return in a single paged response. The minimum value is 1 and the maximum value is 30. The default value is 30. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |

### Return type

[**models::ListLoyaltyPromotionsResponse**](ListLoyaltyPromotionsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## redeem_loyalty_reward

> models::RedeemLoyaltyRewardResponse redeem_loyalty_reward(reward_id, redeem_loyalty_reward_request)
RedeemLoyaltyReward

Redeems a loyalty reward.  The endpoint sets the reward to the `REDEEMED` terminal state.  If you are using your own order processing system (not using the Orders API), you call this endpoint after the buyer paid for the purchase.  After the reward reaches the terminal state, it cannot be deleted. In other words, points used for the reward cannot be returned to the account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reward_id** | **String** | The ID of the [loyalty reward](entity:LoyaltyReward) to redeem. | [required] |
**redeem_loyalty_reward_request** | [**RedeemLoyaltyRewardRequest**](RedeemLoyaltyRewardRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::RedeemLoyaltyRewardResponse**](RedeemLoyaltyRewardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_loyalty_account

> models::RetrieveLoyaltyAccountResponse retrieve_loyalty_account(account_id)
RetrieveLoyaltyAccount

Retrieves a loyalty account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the [loyalty account](entity:LoyaltyAccount) to retrieve. | [required] |

### Return type

[**models::RetrieveLoyaltyAccountResponse**](RetrieveLoyaltyAccountResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_loyalty_program

> models::RetrieveLoyaltyProgramResponse retrieve_loyalty_program(program_id)
RetrieveLoyaltyProgram

Retrieves the loyalty program in a seller's account, specified by the program ID or the keyword `main`.  Loyalty programs define how buyers can earn points and redeem points for rewards. Square sellers can have only one loyalty program, which is created and managed from the Seller Dashboard. For more information, see [Loyalty Program Overview](https://developer.squareup.com/docs/loyalty/overview).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**program_id** | **String** | The ID of the loyalty program or the keyword `main`. Either value can be used to retrieve the single loyalty program that belongs to the seller. | [required] |

### Return type

[**models::RetrieveLoyaltyProgramResponse**](RetrieveLoyaltyProgramResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_loyalty_promotion

> models::RetrieveLoyaltyPromotionResponse retrieve_loyalty_promotion(promotion_id, program_id)
RetrieveLoyaltyPromotion

Retrieves a loyalty promotion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**promotion_id** | **String** | The ID of the [loyalty promotion](entity:LoyaltyPromotion) to retrieve. | [required] |
**program_id** | **String** | The ID of the base [loyalty program](entity:LoyaltyProgram). To get the program ID, call [RetrieveLoyaltyProgram](api-endpoint:Loyalty-RetrieveLoyaltyProgram) using the `main` keyword. | [required] |

### Return type

[**models::RetrieveLoyaltyPromotionResponse**](RetrieveLoyaltyPromotionResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_loyalty_reward

> models::RetrieveLoyaltyRewardResponse retrieve_loyalty_reward(reward_id)
RetrieveLoyaltyReward

Retrieves a loyalty reward.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reward_id** | **String** | The ID of the [loyalty reward](entity:LoyaltyReward) to retrieve. | [required] |

### Return type

[**models::RetrieveLoyaltyRewardResponse**](RetrieveLoyaltyRewardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_loyalty_accounts

> models::SearchLoyaltyAccountsResponse search_loyalty_accounts(search_loyalty_accounts_request)
SearchLoyaltyAccounts

Searches for loyalty accounts in a loyalty program.  You can search for a loyalty account using the phone number or customer ID associated with the account. To return all loyalty accounts, specify an empty `query` object or omit it entirely.  Search results are sorted by `created_at` in ascending order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_loyalty_accounts_request** | [**SearchLoyaltyAccountsRequest**](SearchLoyaltyAccountsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchLoyaltyAccountsResponse**](SearchLoyaltyAccountsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_loyalty_events

> models::SearchLoyaltyEventsResponse search_loyalty_events(search_loyalty_events_request)
SearchLoyaltyEvents

Searches for loyalty events.  A Square loyalty program maintains a ledger of events that occur during the lifetime of a buyer's loyalty account. Each change in the point balance (for example, points earned, points redeemed, and points expired) is recorded in the ledger. Using this endpoint, you can search the ledger for events.  Search results are sorted by `created_at` in descending order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_loyalty_events_request** | [**SearchLoyaltyEventsRequest**](SearchLoyaltyEventsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchLoyaltyEventsResponse**](SearchLoyaltyEventsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_loyalty_rewards

> models::SearchLoyaltyRewardsResponse search_loyalty_rewards(search_loyalty_rewards_request)
SearchLoyaltyRewards

Searches for loyalty rewards. This endpoint accepts a request with no query filters and returns results for all loyalty accounts. If you include a `query` object, `loyalty_account_id` is required and `status` is  optional.  If you know a reward ID, use the [RetrieveLoyaltyReward](api-endpoint:Loyalty-RetrieveLoyaltyReward) endpoint.  Search results are sorted by `updated_at` in descending order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_loyalty_rewards_request** | [**SearchLoyaltyRewardsRequest**](SearchLoyaltyRewardsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchLoyaltyRewardsResponse**](SearchLoyaltyRewardsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

