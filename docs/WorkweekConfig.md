# WorkweekConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The UUID for this object. | [optional]
**start_of_week** | [**models::Weekday**](Weekday.md) |  | 
**start_of_day_local_time** | **String** | The local time at which a business week starts. Represented as a string in `HH:MM` format (`HH:MM:SS` is also accepted, but seconds are truncated). | 
**version** | Option<**i32**> | Used for resolving concurrency issues. The request fails if the version provided does not match the server version at the time of the request. If not provided, Square executes a blind write; potentially overwriting data from another write. | [optional]
**created_at** | Option<**String**> | A read-only timestamp in RFC 3339 format; presented in UTC. | [optional][readonly]
**updated_at** | Option<**String**> | A read-only timestamp in RFC 3339 format; presented in UTC. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


