# Customer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A unique Square-assigned ID for the customer profile.  If you need this ID for an API request, use the ID returned when you created the customer profile or call the [SearchCustomers](api-endpoint:Customers-SearchCustomers)  or [ListCustomers](api-endpoint:Customers-ListCustomers) endpoint. | [optional]
**created_at** | Option<**String**> | The timestamp when the customer profile was created, in RFC 3339 format. | [optional][readonly]
**updated_at** | Option<**String**> | The timestamp when the customer profile was last updated, in RFC 3339 format. | [optional][readonly]
**given_name** | Option<**String**> | The given name (that is, the first name) associated with the customer profile. | [optional]
**family_name** | Option<**String**> | The family name (that is, the last name) associated with the customer profile. | [optional]
**nickname** | Option<**String**> | A nickname for the customer profile. | [optional]
**company_name** | Option<**String**> | A business name associated with the customer profile. | [optional]
**email_address** | Option<**String**> | The email address associated with the customer profile. | [optional]
**address** | Option<[**models::Address**](Address.md)> |  | [optional]
**phone_number** | Option<**String**> | The phone number associated with the customer profile. | [optional]
**birthday** | Option<**String**> | The birthday associated with the customer profile, in `YYYY-MM-DD` format. For example, `1998-09-21` represents September 21, 1998, and `0000-09-21` represents September 21 (without a birth year). | [optional]
**reference_id** | Option<**String**> | An optional second ID used to associate the customer profile with an entity in another system. | [optional]
**note** | Option<**String**> | A custom note associated with the customer profile. | [optional]
**preferences** | Option<[**models::CustomerPreferences**](CustomerPreferences.md)> |  | [optional]
**creation_source** | Option<[**models::CustomerCreationSource**](CustomerCreationSource.md)> |  | [optional]
**group_ids** | Option<**Vec<String>**> | The IDs of [customer groups](entity:CustomerGroup) the customer belongs to. | [optional]
**segment_ids** | Option<**Vec<String>**> | The IDs of [customer segments](entity:CustomerSegment) the customer belongs to. | [optional]
**version** | Option<**i64**> | The Square-assigned version number of the customer profile. The version number is incremented each time an update is committed to the customer profile, except for changes to customer segment membership. | [optional]
**tax_ids** | Option<[**models::CustomerTaxIds**](CustomerTaxIds.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


