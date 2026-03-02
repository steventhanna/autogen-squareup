# Address

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address_line_1** | Option<**String**> | The first line of the address.  Fields that start with `address_line` provide the address's most specific details, like street number, street name, and building name. They do *not* provide less specific details like city, state/province, or country (these details are provided in other fields). | [optional]
**address_line_2** | Option<**String**> | The second line of the address, if any. | [optional]
**address_line_3** | Option<**String**> | The third line of the address, if any. | [optional]
**locality** | Option<**String**> | The city or town of the address. For a full list of field meanings by country, see [Working with Addresses](https://developer.squareup.com/docs/build-basics/working-with-addresses). | [optional]
**sublocality** | Option<**String**> | A civil region within the address's `locality`, if any. | [optional]
**sublocality_2** | Option<**String**> | A civil region within the address's `sublocality`, if any. | [optional]
**sublocality_3** | Option<**String**> | A civil region within the address's `sublocality_2`, if any. | [optional]
**administrative_district_level_1** | Option<**String**> | A civil entity within the address's country. In the US, this is the state. For a full list of field meanings by country, see [Working with Addresses](https://developer.squareup.com/docs/build-basics/working-with-addresses). | [optional]
**administrative_district_level_2** | Option<**String**> | A civil entity within the address's `administrative_district_level_1`. In the US, this is the county. | [optional]
**administrative_district_level_3** | Option<**String**> | A civil entity within the address's `administrative_district_level_2`, if any. | [optional]
**postal_code** | Option<**String**> | The address's postal code. For a full list of field meanings by country, see [Working with Addresses](https://developer.squareup.com/docs/build-basics/working-with-addresses). | [optional]
**country** | Option<[**models::Country**](Country.md)> |  | [optional]
**first_name** | Option<**String**> | Optional first name when it's representing recipient. | [optional]
**last_name** | Option<**String**> | Optional last name when it's representing recipient. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


