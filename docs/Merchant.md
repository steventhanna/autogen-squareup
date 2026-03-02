# Merchant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The Square-issued ID of the merchant. | [optional]
**business_name** | Option<**String**> | The name of the merchant's overall business. | [optional]
**country** | [**models::Country**](Country.md) |  | 
**language_code** | Option<**String**> | The code indicating the [language preferences](https://developer.squareup.com/docs/build-basics/general-considerations/language-preferences) of the merchant, in [BCP 47 format](https://tools.ietf.org/html/bcp47#appendix-A). For example, `en-US` or `fr-CA`. | [optional]
**currency** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**status** | Option<[**models::MerchantStatus**](MerchantStatus.md)> |  | [optional]
**main_location_id** | Option<**String**> | The ID of the [main `Location`](https://developer.squareup.com/docs/locations-api#about-the-main-location) for this merchant. | [optional]
**created_at** | Option<**String**> | The time when the merchant was created, in RFC 3339 format.    For more information, see [Working with Dates](https://developer.squareup.com/docs/build-basics/working-with-dates). | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


