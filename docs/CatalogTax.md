# CatalogTax

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The tax's name. This is a searchable attribute for use in applicable query filters, and its value length is of Unicode code points. | [optional]
**calculation_phase** | Option<[**models::TaxCalculationPhase**](TaxCalculationPhase.md)> |  | [optional]
**inclusion_type** | Option<[**models::TaxInclusionType**](TaxInclusionType.md)> |  | [optional]
**percentage** | Option<**String**> | The percentage of the tax in decimal form, using a `'.'` as the decimal separator and without a `'%'` sign. A value of `7.5` corresponds to 7.5%. For a location-specific tax rate, contact the tax authority of the location or a tax consultant. | [optional]
**applies_to_custom_amounts** | Option<**bool**> | If `true`, the fee applies to custom amounts entered into the Square Point of Sale app that are not associated with a particular `CatalogItem`. | [optional]
**enabled** | Option<**bool**> | A Boolean flag to indicate whether the tax is displayed as enabled (`true`) in the Square Point of Sale app or not (`false`). | [optional]
**applies_to_product_set_id** | Option<**String**> | The ID of a `CatalogProductSet` object. If set, the tax is applicable to all products in the product set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


