# OrderQuantityUnit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**measurement_unit** | Option<[**models::MeasurementUnit**](MeasurementUnit.md)> |  | [optional]
**precision** | Option<**i32**> | For non-integer quantities, represents the number of digits after the decimal point that are recorded for this quantity.  For example, a precision of 1 allows quantities such as `\"1.0\"` and `\"1.1\"`, but not `\"1.01\"`.  Min: 0. Max: 5. | [optional]
**catalog_object_id** | Option<**String**> | The catalog object ID referencing the [CatalogMeasurementUnit](entity:CatalogMeasurementUnit).  This field is set when this is a catalog-backed measurement unit. | [optional]
**catalog_version** | Option<**i64**> | The version of the catalog object that this measurement unit references.  This field is set when this is a catalog-backed measurement unit. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


