# BulkUpdateCustomerData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**given_name** | Option<**String**> | The given name (that is, the first name) associated with the customer profile. | [optional]
**family_name** | Option<**String**> | The family name (that is, the last name) associated with the customer profile. | [optional]
**company_name** | Option<**String**> | A business name associated with the customer profile. | [optional]
**nickname** | Option<**String**> | A nickname for the customer profile. | [optional]
**email_address** | Option<**String**> | The email address associated with the customer profile. | [optional]
**address** | Option<[**models::Address**](Address.md)> |  | [optional]
**phone_number** | Option<**String**> | The phone number associated with the customer profile. The phone number must be valid and can contain 9–16 digits, with an optional `+` prefix and country code. For more information, see [Customer phone numbers](https://developer.squareup.com/docs/customers-api/use-the-api/keep-records#phone-number). | [optional]
**reference_id** | Option<**String**> | An optional second ID used to associate the customer profile with an entity in another system. | [optional]
**note** | Option<**String**> | An custom note associates with the customer profile. | [optional]
**birthday** | Option<**String**> | The birthday associated with the customer profile, in `YYYY-MM-DD` or `MM-DD` format. For example, specify `1998-09-21` for September 21, 1998, or `09-21` for September 21. Birthdays are returned in `YYYY-MM-DD` format, where `YYYY` is the specified birth year or `0000` if a birth year is not specified. | [optional]
**tax_ids** | Option<[**models::CustomerTaxIds**](CustomerTaxIds.md)> |  | [optional]
**version** | Option<**i64**> | The current version of the customer profile.  As a best practice, you should include this field to enable [optimistic concurrency](https://developer.squareup.com/docs/build-basics/common-api-patterns/optimistic-concurrency) control. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


