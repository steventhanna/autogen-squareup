# CheckoutLocationSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**location_id** | Option<**String**> | The ID of the location that these settings apply to. | [optional]
**customer_notes_enabled** | Option<**bool**> | Indicates whether customers are allowed to leave notes at checkout. | [optional]
**policies** | Option<[**Vec<models::CheckoutLocationSettingsPolicy>**](CheckoutLocationSettingsPolicy.md)> | Policy information is displayed at the bottom of the checkout pages. You can set a maximum of two policies. | [optional]
**branding** | Option<[**models::CheckoutLocationSettingsBranding**](CheckoutLocationSettingsBranding.md)> |  | [optional]
**tipping** | Option<[**models::CheckoutLocationSettingsTipping**](CheckoutLocationSettingsTipping.md)> |  | [optional]
**coupons** | Option<[**models::CheckoutLocationSettingsCoupons**](CheckoutLocationSettingsCoupons.md)> |  | [optional]
**updated_at** | Option<**String**> | The timestamp when the settings were last updated, in RFC 3339 format. Examples for January 25th, 2020 6:25:34pm Pacific Standard Time: UTC: 2020-01-26T02:25:34Z Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00 | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


