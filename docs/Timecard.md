# Timecard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | **Read only** The Square-issued UUID for this object. | [optional]
**location_id** | **String** | The ID of the [location](entity:Location) for this timecard. The location should be based on where the team member clocked in. | 
**timezone** | Option<**String**> | **Read only** The time zone calculated from the location based on the `location_id`, provided as a convenience value. Format: the IANA time zone database identifier for the location time zone. | [optional]
**start_at** | **String** | The start time of the timecard, in RFC 3339 format and shifted to the location timezone + offset. Precision up to the minute is respected; seconds are truncated. | 
**end_at** | Option<**String**> | The end time of the timecard, in RFC 3339 format and shifted to the location timezone + offset. Precision up to the minute is respected; seconds are truncated. | [optional]
**wage** | Option<[**models::TimecardWage**](TimecardWage.md)> |  | [optional]
**breaks** | Option<[**Vec<models::Break>**](Break.md)> | A list of all the paid or unpaid breaks that were taken during this timecard. | [optional]
**status** | Option<[**models::TimecardStatus**](TimecardStatus.md)> |  | [optional]
**version** | Option<**i32**> | **Read only** The current version of the timecard, which is incremented with each update. This field is used for [optimistic concurrency](https://developer.squareup.com/docs/build-basics/common-api-patterns/optimistic-concurrency) control to ensure that requests don't overwrite data from another request. | [optional]
**created_at** | Option<**String**> | The timestamp of when the timecard was created, in RFC 3339 format presented as UTC. | [optional][readonly]
**updated_at** | Option<**String**> | The timestamp of when the timecard was last updated, in RFC 3339 format presented as UTC. | [optional][readonly]
**team_member_id** | **String** | The ID of the [team member](entity:TeamMember) this timecard belongs to. | 
**declared_cash_tip_money** | Option<[**models::Money**](Money.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


