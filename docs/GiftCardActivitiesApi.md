# \GiftCardActivitiesApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_gift_card_activity**](GiftCardActivitiesApi.md#create_gift_card_activity) | **POST** /v2/gift-cards/activities | CreateGiftCardActivity
[**list_gift_card_activities**](GiftCardActivitiesApi.md#list_gift_card_activities) | **GET** /v2/gift-cards/activities | ListGiftCardActivities



## create_gift_card_activity

> models::CreateGiftCardActivityResponse create_gift_card_activity(create_gift_card_activity_request)
CreateGiftCardActivity

Creates a gift card activity to manage the balance or state of a [gift card](entity:GiftCard). For example, create an `ACTIVATE` activity to activate a gift card with an initial balance before first use.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_gift_card_activity_request** | [**CreateGiftCardActivityRequest**](CreateGiftCardActivityRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateGiftCardActivityResponse**](CreateGiftCardActivityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gift_card_activities

> models::ListGiftCardActivitiesResponse list_gift_card_activities(gift_card_id, r#type, location_id, begin_time, end_time, limit, cursor, sort_order)
ListGiftCardActivities

Lists gift card activities. By default, you get gift card activities for all gift cards in the seller's account. You can optionally specify query parameters to filter the list. For example, you can get a list of gift card activities for a gift card, for all gift cards in a specific region, or for activities within a time window.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gift_card_id** | Option<**String**> | If a gift card ID is provided, the endpoint returns activities related  to the specified gift card. Otherwise, the endpoint returns all gift card activities for  the seller. |  |
**r#type** | Option<**String**> | If a [type](entity:GiftCardActivityType) is provided, the endpoint returns gift card activities of the specified type.  Otherwise, the endpoint returns all types of gift card activities. |  |
**location_id** | Option<**String**> | If a location ID is provided, the endpoint returns gift card activities for the specified location.  Otherwise, the endpoint returns gift card activities for all locations. |  |
**begin_time** | Option<**String**> | The timestamp for the beginning of the reporting period, in RFC 3339 format. This start time is inclusive. The default value is the current time minus one year. |  |
**end_time** | Option<**String**> | The timestamp for the end of the reporting period, in RFC 3339 format. This end time is inclusive. The default value is the current time. |  |
**limit** | Option<**i32**> | If a limit is provided, the endpoint returns the specified number  of results (or fewer) per page. The maximum value is 100. The default value is 50. For more information, see [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination). |  |
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this cursor to retrieve the next set of results for the original query. If a cursor is not provided, the endpoint returns the first page of the results. For more information, see [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination). |  |
**sort_order** | Option<**String**> | The order in which the endpoint returns the activities, based on `created_at`. - `ASC` - Oldest to newest. - `DESC` - Newest to oldest (default). |  |

### Return type

[**models::ListGiftCardActivitiesResponse**](ListGiftCardActivitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

