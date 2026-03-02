# BreakType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The UUID for this object. | [optional]
**location_id** | **String** | The ID of the business location this type of break applies to. | 
**break_name** | **String** | A human-readable name for this type of break. The name is displayed to team members in Square products. | 
**expected_duration** | **String** | Format: RFC-3339 P[n]Y[n]M[n]DT[n]H[n]M[n]S. The expected length of this break. Precision less than minutes is truncated.  Example for break expected duration of 15 minutes: PT15M | 
**is_paid** | **bool** | Whether this break counts towards time worked for compensation purposes. | 
**version** | Option<**i32**> | Used for resolving concurrency issues. The request fails if the version provided does not match the server version at the time of the request. If a value is not provided, Square's servers execute a \"blind\" write; potentially overwriting another writer's data. | [optional]
**created_at** | Option<**String**> | A read-only timestamp in RFC 3339 format. | [optional][readonly]
**updated_at** | Option<**String**> | A read-only timestamp in RFC 3339 format. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


