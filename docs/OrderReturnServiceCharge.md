# OrderReturnServiceCharge

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | A unique ID that identifies the return service charge only within this order. | [optional]
**source_service_charge_uid** | Option<**String**> | The service charge `uid` from the order containing the original service charge. `source_service_charge_uid` is `null` for unlinked returns. | [optional]
**name** | Option<**String**> | The name of the service charge. | [optional]
**catalog_object_id** | Option<**String**> | The catalog object ID of the associated [OrderServiceCharge](entity:OrderServiceCharge). | [optional]
**catalog_version** | Option<**i64**> | The version of the catalog object that this service charge references. | [optional]
**percentage** | Option<**String**> | The percentage of the service charge, as a string representation of a decimal number. For example, a value of `\"7.25\"` corresponds to a percentage of 7.25%.  Either `percentage` or `amount_money` should be set, but not both. | [optional]
**amount_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**applied_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**total_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**total_tax_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**calculation_phase** | Option<[**models::OrderServiceChargeCalculationPhase**](OrderServiceChargeCalculationPhase.md)> |  | [optional]
**taxable** | Option<**bool**> | Indicates whether the surcharge can be taxed. Service charges calculated in the `TOTAL_PHASE` cannot be marked as taxable. | [optional]
**applied_taxes** | Option<[**Vec<models::OrderLineItemAppliedTax>**](OrderLineItemAppliedTax.md)> | The list of references to `OrderReturnTax` entities applied to the `OrderReturnServiceCharge`. Each `OrderLineItemAppliedTax` has a `tax_uid` that references the `uid` of a top-level `OrderReturnTax` that is being applied to the `OrderReturnServiceCharge`. On reads, the applied amount is populated. | [optional]
**treatment_type** | Option<[**models::OrderServiceChargeTreatmentType**](OrderServiceChargeTreatmentType.md)> |  | [optional]
**scope** | Option<[**models::OrderServiceChargeScope**](OrderServiceChargeScope.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


