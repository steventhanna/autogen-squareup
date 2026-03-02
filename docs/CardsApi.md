# \CardsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_card**](CardsApi.md#create_card) | **POST** /v2/cards | CreateCard
[**disable_card**](CardsApi.md#disable_card) | **POST** /v2/cards/{card_id}/disable | DisableCard
[**list_cards**](CardsApi.md#list_cards) | **GET** /v2/cards | ListCards
[**retrieve_card**](CardsApi.md#retrieve_card) | **GET** /v2/cards/{card_id} | RetrieveCard



## create_card

> models::CreateCardResponse create_card(create_card_request)
CreateCard

Adds a card on file to an existing merchant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_card_request** | [**CreateCardRequest**](CreateCardRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateCardResponse**](CreateCardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_card

> models::DisableCardResponse disable_card(card_id)
DisableCard

Disables the card, preventing any further updates or charges. Disabling an already disabled card is allowed but has no effect.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **String** | Unique ID for the desired Card. | [required] |

### Return type

[**models::DisableCardResponse**](DisableCardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cards

> models::ListCardsResponse list_cards(cursor, customer_id, include_disabled, reference_id, sort_order)
ListCards

Retrieves a list of cards owned by the account making the request. A max of 25 cards will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this to retrieve the next set of results for your original query.  See [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination) for more information. |  |
**customer_id** | Option<**String**> | Limit results to cards associated with the customer supplied. By default, all cards owned by the merchant are returned. |  |
**include_disabled** | Option<**bool**> | Includes disabled cards. By default, all enabled cards owned by the merchant are returned. |  |[default to false]
**reference_id** | Option<**String**> | Limit results to cards associated with the reference_id supplied. |  |
**sort_order** | Option<[**SortOrder**](.md)> | Sorts the returned list by when the card was created with the specified order. This field defaults to ASC. |  |

### Return type

[**models::ListCardsResponse**](ListCardsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_card

> models::RetrieveCardResponse retrieve_card(card_id)
RetrieveCard

Retrieves details for a specific Card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **String** | Unique ID for the desired Card. | [required] |

### Return type

[**models::RetrieveCardResponse**](RetrieveCardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

