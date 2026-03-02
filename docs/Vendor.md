# Vendor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A unique Square-generated ID for the [Vendor](entity:Vendor). This field is required when attempting to update a [Vendor](entity:Vendor). | [optional]
**created_at** | Option<**String**> | An RFC 3339-formatted timestamp that indicates when the [Vendor](entity:Vendor) was created. | [optional][readonly]
**updated_at** | Option<**String**> | An RFC 3339-formatted timestamp that indicates when the [Vendor](entity:Vendor) was last updated. | [optional][readonly]
**name** | Option<**String**> | The name of the [Vendor](entity:Vendor). This field is required when attempting to create or update a [Vendor](entity:Vendor). | [optional]
**address** | Option<[**models::Address**](Address.md)> |  | [optional]
**contacts** | Option<[**Vec<models::VendorContact>**](VendorContact.md)> | The contacts of the [Vendor](entity:Vendor). | [optional]
**account_number** | Option<**String**> | The account number of the [Vendor](entity:Vendor). | [optional]
**note** | Option<**String**> | A note detailing information about the [Vendor](entity:Vendor). | [optional]
**version** | Option<**i32**> | The version of the [Vendor](entity:Vendor). | [optional]
**status** | Option<[**models::VendorStatus**](VendorStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


