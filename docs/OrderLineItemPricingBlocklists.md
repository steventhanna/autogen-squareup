# OrderLineItemPricingBlocklists

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blocked_discounts** | Option<[**Vec<models::OrderLineItemPricingBlocklistsBlockedDiscount>**](OrderLineItemPricingBlocklistsBlockedDiscount.md)> | A list of discounts blocked from applying to the line item. Discounts can be blocked by the `discount_uid` (for ad hoc discounts) or the `discount_catalog_object_id` (for catalog discounts). | [optional]
**blocked_taxes** | Option<[**Vec<models::OrderLineItemPricingBlocklistsBlockedTax>**](OrderLineItemPricingBlocklistsBlockedTax.md)> | A list of taxes blocked from applying to the line item. Taxes can be blocked by the `tax_uid` (for ad hoc taxes) or the `tax_catalog_object_id` (for catalog taxes). | [optional]
**blocked_service_charges** | Option<[**Vec<models::OrderLineItemPricingBlocklistsBlockedServiceCharge>**](OrderLineItemPricingBlocklistsBlockedServiceCharge.md)> | A list of service charges blocked from applying to the line item. Service charges can be blocked by the `service_charge_uid` (for ad hoc service charges) or the `service_charge_catalog_object_id` (for catalog service charges). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


