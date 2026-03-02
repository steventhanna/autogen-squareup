# Break

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The UUID for this object. | [optional]
**start_at** | **String** | RFC 3339; follows the same timezone information as the [timecard](entity:Timecard). Precision up to the minute is respected; seconds are truncated. | 
**end_at** | Option<**String**> | RFC 3339; follows the same timezone information as the [timecard](entity:Timecard). Precision up to the minute is respected; seconds are truncated. | [optional]
**break_type_id** | **String** | The [BreakType](entity:BreakType) that this break was templated on. | 
**name** | **String** | A human-readable name. | 
**expected_duration** | **String** | Format: RFC-3339 P[n]Y[n]M[n]DT[n]H[n]M[n]S. The expected length of the break.  Example for break expected duration of 15 minutes: PT15M | 
**is_paid** | **bool** | Whether this break counts towards time worked for compensation purposes. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


