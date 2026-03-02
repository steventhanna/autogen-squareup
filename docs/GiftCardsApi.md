# \GiftCardsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_gift_card**](GiftCardsApi.md#create_gift_card) | **POST** /v2/gift-cards | CreateGiftCard
[**link_customer_to_gift_card**](GiftCardsApi.md#link_customer_to_gift_card) | **POST** /v2/gift-cards/{gift_card_id}/link-customer | LinkCustomerToGiftCard
[**list_gift_cards**](GiftCardsApi.md#list_gift_cards) | **GET** /v2/gift-cards | ListGiftCards
[**retrieve_gift_card**](GiftCardsApi.md#retrieve_gift_card) | **GET** /v2/gift-cards/{id} | RetrieveGiftCard
[**retrieve_gift_card_from_gan**](GiftCardsApi.md#retrieve_gift_card_from_gan) | **POST** /v2/gift-cards/from-gan | RetrieveGiftCardFromGAN
[**retrieve_gift_card_from_nonce**](GiftCardsApi.md#retrieve_gift_card_from_nonce) | **POST** /v2/gift-cards/from-nonce | RetrieveGiftCardFromNonce
[**unlink_customer_from_gift_card**](GiftCardsApi.md#unlink_customer_from_gift_card) | **POST** /v2/gift-cards/{gift_card_id}/unlink-customer | UnlinkCustomerFromGiftCard



## create_gift_card

> models::CreateGiftCardResponse create_gift_card(create_gift_card_request)
CreateGiftCard

Creates a digital gift card or registers a physical (plastic) gift card. The resulting gift card has a `PENDING` state. To activate a gift card so that it can be redeemed for purchases, call [CreateGiftCardActivity](api-endpoint:GiftCardActivities-CreateGiftCardActivity) and create an `ACTIVATE` activity with the initial balance. Alternatively, you can use [RefundPayment](api-endpoint:Refunds-RefundPayment) to refund a payment to the new gift card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_gift_card_request** | [**CreateGiftCardRequest**](CreateGiftCardRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateGiftCardResponse**](CreateGiftCardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_customer_to_gift_card

> models::LinkCustomerToGiftCardResponse link_customer_to_gift_card(gift_card_id, link_customer_to_gift_card_request)
LinkCustomerToGiftCard

Links a customer to a gift card, which is also referred to as adding a card on file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gift_card_id** | **String** | The ID of the gift card to be linked. | [required] |
**link_customer_to_gift_card_request** | [**LinkCustomerToGiftCardRequest**](LinkCustomerToGiftCardRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::LinkCustomerToGiftCardResponse**](LinkCustomerToGiftCardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gift_cards

> models::ListGiftCardsResponse list_gift_cards(r#type, state, limit, cursor, customer_id)
ListGiftCards

Lists all gift cards. You can specify optional filters to retrieve  a subset of the gift cards. Results are sorted by `created_at` in ascending order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | If a [type](entity:GiftCardType) is provided, the endpoint returns gift cards of the specified type. Otherwise, the endpoint returns gift cards of all types. |  |
**state** | Option<**String**> | If a [state](entity:GiftCardStatus) is provided, the endpoint returns the gift cards in the specified state. Otherwise, the endpoint returns the gift cards of all states. |  |
**limit** | Option<**i32**> | If a limit is provided, the endpoint returns only the specified number of results per page. The maximum value is 200. The default value is 30. For more information, see [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination). |  |
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this cursor to retrieve the next set of results for the original query. If a cursor is not provided, the endpoint returns the first page of the results.  For more information, see [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination). |  |
**customer_id** | Option<**String**> | If a customer ID is provided, the endpoint returns only the gift cards linked to the specified customer. |  |

### Return type

[**models::ListGiftCardsResponse**](ListGiftCardsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_gift_card

> models::RetrieveGiftCardResponse retrieve_gift_card(id)
RetrieveGiftCard

Retrieves a gift card using the gift card ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the gift card to retrieve. | [required] |

### Return type

[**models::RetrieveGiftCardResponse**](RetrieveGiftCardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_gift_card_from_gan

> models::RetrieveGiftCardFromGanResponse retrieve_gift_card_from_gan(retrieve_gift_card_from_gan_request)
RetrieveGiftCardFromGAN

Retrieves a gift card using the gift card account number (GAN).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrieve_gift_card_from_gan_request** | [**RetrieveGiftCardFromGanRequest**](RetrieveGiftCardFromGanRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::RetrieveGiftCardFromGanResponse**](RetrieveGiftCardFromGANResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_gift_card_from_nonce

> models::RetrieveGiftCardFromNonceResponse retrieve_gift_card_from_nonce(retrieve_gift_card_from_nonce_request)
RetrieveGiftCardFromNonce

Retrieves a gift card using a secure payment token that represents the gift card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrieve_gift_card_from_nonce_request** | [**RetrieveGiftCardFromNonceRequest**](RetrieveGiftCardFromNonceRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::RetrieveGiftCardFromNonceResponse**](RetrieveGiftCardFromNonceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_customer_from_gift_card

> models::UnlinkCustomerFromGiftCardResponse unlink_customer_from_gift_card(gift_card_id, unlink_customer_from_gift_card_request)
UnlinkCustomerFromGiftCard

Unlinks a customer from a gift card, which is also referred to as removing a card on file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gift_card_id** | **String** | The ID of the gift card to be unlinked. | [required] |
**unlink_customer_from_gift_card_request** | [**UnlinkCustomerFromGiftCardRequest**](UnlinkCustomerFromGiftCardRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UnlinkCustomerFromGiftCardResponse**](UnlinkCustomerFromGiftCardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

