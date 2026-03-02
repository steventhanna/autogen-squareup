# CatalogDiscount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The discount name. This is a searchable attribute for use in applicable query filters, and its value length is of Unicode code points. | [optional]
**discount_type** | Option<[**models::CatalogDiscountType**](CatalogDiscountType.md)> |  | [optional]
**percentage** | Option<**String**> | The percentage of the discount as a string representation of a decimal number, using a `.` as the decimal separator and without a `%` sign. A value of `7.5` corresponds to `7.5%`. Specify a percentage of `0` if `discount_type` is `VARIABLE_PERCENTAGE`.  Do not use this field for amount-based or variable discounts. | [optional]
**amount_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**pin_required** | Option<**bool**> | Indicates whether a mobile staff member needs to enter their PIN to apply the discount to a payment in the Square Point of Sale app. | [optional]
**label_color** | Option<**String**> | The color of the discount display label in the Square Point of Sale app. This must be a valid hex color code. | [optional]
**modify_tax_basis** | Option<[**models::CatalogDiscountModifyTaxBasis**](CatalogDiscountModifyTaxBasis.md)> |  | [optional]
**maximum_amount_money** | Option<[**models::Money**](Money.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


