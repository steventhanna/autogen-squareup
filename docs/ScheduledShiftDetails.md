# ScheduledShiftDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**team_member_id** | Option<**String**> | The ID of the [team member](entity:TeamMember) scheduled for the shift. | [optional]
**location_id** | Option<**String**> | The ID of the [location](entity:Location) the shift is scheduled for. | [optional]
**job_id** | Option<**String**> | The ID of the [job](entity:Job) the shift is scheduled for. | [optional]
**start_at** | Option<**String**> | The start time of the shift, in RFC 3339 format in the time zone &plus; offset of the shift location specified in `location_id`. Precision up to the minute is respected; seconds are truncated. | [optional]
**end_at** | Option<**String**> | The end time for the shift, in RFC 3339 format in the time zone &plus; offset of the shift location specified in `location_id`. Precision up to the minute is respected; seconds are truncated. | [optional]
**notes** | Option<**String**> | Optional notes for the shift. | [optional]
**is_deleted** | Option<**bool**> | Indicates whether the draft shift version is deleted. If set to `true` when the shift is published, the entire scheduled shift (including the published shift) is deleted and cannot be accessed using any endpoint. | [optional]
**timezone** | Option<**String**> | The time zone of the shift location, calculated based on the `location_id`. This field is provided for convenience. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


