# CatalogItemVariationVendorInformation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vendor_id** | Option<**String**> | ID of the [Vendor](entity:Vendor) linked to a default cost of this product. When the product is added to a purchase order, the default cost is pre-filled. This field is not validated. Clients should gracefully handle cases where the vendor_id does not match any existing vendor. | [optional]
**vendor_code** | Option<**String**> | Unique identifier of this product in the specified vendor's' inventory system. When the product is added to a purchase order, the vendor code is pre-filled based on the selected vendor. | [optional]
**unit_cost_money** | Option<[**models::Money**](Money.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


