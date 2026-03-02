# Shift

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The UUID for this object. | [optional]
**employee_id** | Option<**String**> | The ID of the employee this shift belongs to. DEPRECATED at version 2020-08-26. Use `team_member_id` instead. | [optional]
**location_id** | **String** | The ID of the location this shift occurred at. The location should be based on where the employee clocked in. | 
**timezone** | Option<**String**> | The read-only convenience value that is calculated from the location based on the `location_id`. Format: the IANA timezone database identifier for the location timezone. | [optional]
**start_at** | **String** | RFC 3339; shifted to the location timezone + offset. Precision up to the minute is respected; seconds are truncated. | 
**end_at** | Option<**String**> | RFC 3339; shifted to the timezone + offset. Precision up to the minute is respected; seconds are truncated. | [optional]
**wage** | Option<[**models::ShiftWage**](ShiftWage.md)> |  | [optional]
**breaks** | Option<[**Vec<models::Break>**](Break.md)> | A list of all the paid or unpaid breaks that were taken during this shift. | [optional]
**status** | Option<[**models::ShiftStatus**](ShiftStatus.md)> |  | [optional]
**version** | Option<**i32**> | Used for resolving concurrency issues. The request fails if the version provided does not match the server version at the time of the request. If not provided, Square executes a blind write; potentially overwriting data from another write. | [optional]
**created_at** | Option<**String**> | A read-only timestamp in RFC 3339 format; presented in UTC. | [optional][readonly]
**updated_at** | Option<**String**> | A read-only timestamp in RFC 3339 format; presented in UTC. | [optional][readonly]
**team_member_id** | Option<**String**> | The ID of the team member this shift belongs to. Replaced `employee_id` at version \"2020-08-26\". | [optional]
**declared_cash_tip_money** | Option<[**models::Money**](Money.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


