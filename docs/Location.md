# Location

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A short generated string of letters and numbers that uniquely identifies this location instance. | [optional][readonly]
**name** | Option<**String**> | The name of the location. This information appears in the Seller Dashboard as the nickname. A location name must be unique within a seller account. | [optional]
**address** | Option<[**models::Address**](Address.md)> |  | [optional]
**timezone** | Option<**String**> | The [IANA time zone](https://www.iana.org/time-zones) identifier for the time zone of the location. For example, `America/Los_Angeles`. | [optional]
**capabilities** | Option<[**Vec<models::LocationCapability>**](LocationCapability.md)> | The Square features that are enabled for the location. See [LocationCapability](entity:LocationCapability) for possible values. See [LocationCapability](#type-locationcapability) for possible values | [optional][readonly]
**status** | Option<[**models::LocationStatus**](LocationStatus.md)> |  | [optional]
**created_at** | Option<**String**> | The time when the location was created, in RFC 3339 format. For more information, see [Working with Dates](https://developer.squareup.com/docs/build-basics/working-with-dates). | [optional][readonly]
**merchant_id** | Option<**String**> | The ID of the merchant that owns the location. | [optional][readonly]
**country** | Option<[**models::Country**](Country.md)> |  | [optional]
**language_code** | Option<**String**> | The language associated with the location, in [BCP 47 format](https://tools.ietf.org/html/bcp47#appendix-A). For more information, see [Language Preferences](https://developer.squareup.com/docs/build-basics/general-considerations/language-preferences). | [optional]
**currency** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**phone_number** | Option<**String**> | The phone number of the location. For example, `+1 855-700-6000`. | [optional]
**business_name** | Option<**String**> | The name of the location's overall business. This name is present on receipts and other customer-facing branding, and can be changed no more than three times in a twelve-month period. | [optional]
**r#type** | Option<[**models::LocationType**](LocationType.md)> |  | [optional]
**website_url** | Option<**String**> | The website URL of the location.  For example, `https://squareup.com`. | [optional]
**business_hours** | Option<[**models::BusinessHours**](BusinessHours.md)> |  | [optional]
**business_email** | Option<**String**> | The email address of the location. This can be unique to the location and is not always the email address for the business owner or administrator. | [optional]
**description** | Option<**String**> | The description of the location. For example, `Main Street location`. | [optional]
**twitter_username** | Option<**String**> | The Twitter username of the location without the '@' symbol. For example, `Square`. | [optional]
**instagram_username** | Option<**String**> | The Instagram username of the location without the '@' symbol. For example, `square`. | [optional]
**facebook_url** | Option<**String**> | The Facebook profile URL of the location. The URL should begin with 'facebook.com/'. For example, `https://www.facebook.com/square`. | [optional]
**coordinates** | Option<[**models::Coordinates**](Coordinates.md)> |  | [optional]
**logo_url** | Option<**String**> | The URL of the logo image for the location. When configured in the Seller Dashboard (Receipts section), the logo appears on transactions (such as receipts and invoices) that Square generates on behalf of the seller. This image should have a roughly square (1:1) aspect ratio and should be at least 200x200 pixels. | [optional][readonly]
**pos_background_url** | Option<**String**> | The URL of the Point of Sale background image for the location. | [optional][readonly]
**mcc** | Option<**String**> | A four-digit number that describes the kind of goods or services sold at the location. The [merchant category code (MCC)](https://developer.squareup.com/docs/locations-api#initialize-a-merchant-category-code) of the location as standardized by ISO 18245. For example, `5045`, for a location that sells computer goods and software. | [optional]
**full_format_logo_url** | Option<**String**> | The URL of a full-format logo image for the location. When configured in the Seller Dashboard (Receipts section), the logo appears on transactions (such as receipts and invoices) that Square generates on behalf of the seller. This image can be wider than it is tall and should be at least 1280x648 pixels. | [optional][readonly]
**tax_ids** | Option<[**models::TaxIds**](TaxIds.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


