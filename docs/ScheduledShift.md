# ScheduledShift

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | **Read only** The Square-issued ID of the scheduled shift. | [optional]
**draft_shift_details** | Option<[**models::ScheduledShiftDetails**](ScheduledShiftDetails.md)> |  | [optional]
**published_shift_details** | Option<[**models::ScheduledShiftDetails**](ScheduledShiftDetails.md)> |  | [optional]
**version** | Option<**i32**> | **Read only** The current version of the scheduled shift, which is incremented with each update. This field is used for [optimistic concurrency](https://developer.squareup.com/docs/build-basics/common-api-patterns/optimistic-concurrency) control to ensure that requests don't overwrite data from another request. | [optional]
**created_at** | Option<**String**> | The timestamp of when the scheduled shift was created, in RFC 3339 format presented as UTC. | [optional][readonly]
**updated_at** | Option<**String**> | The timestamp of when the scheduled shift was last updated, in RFC 3339 format presented as UTC. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


