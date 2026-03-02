# ScheduledShiftFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**location_ids** | Option<**Vec<String>**> | Return shifts for the specified locations. When omitted, shifts for all locations are returned. If needed, call [ListLocations](api-endpoint:Locations-ListLocations) to get location IDs. | [optional]
**start** | Option<[**models::TimeRange**](TimeRange.md)> |  | [optional]
**end** | Option<[**models::TimeRange**](TimeRange.md)> |  | [optional]
**workday** | Option<[**models::ScheduledShiftWorkday**](ScheduledShiftWorkday.md)> |  | [optional]
**team_member_ids** | Option<**Vec<String>**> | Return shifts assigned to specified team members. If needed, call [SearchTeamMembers](api-endpoint:Team-SearchTeamMembers) to get team member IDs.  To return only the shifts assigned to the specified team members, include the `assignment_status` filter in the query. Otherwise, all unassigned shifts are returned along with shifts assigned to the specified team members. | [optional]
**assignment_status** | Option<[**models::ScheduledShiftFilterAssignmentStatus**](ScheduledShiftFilterAssignmentStatus.md)> |  | [optional]
**scheduled_shift_statuses** | Option<[**Vec<models::ScheduledShiftFilterScheduledShiftStatus>**](ScheduledShiftFilterScheduledShiftStatus.md)> | Return shifts based on the draft or published status of the shift. A shift is published if the `published_shift_details` field is present.  Note that shifts with `draft_shift_details.is_deleted` set to `true` are ignored with the `DRAFT` filter. See [ScheduledShiftFilterScheduledShiftStatus](#type-scheduledshiftfilterscheduledshiftstatus) for possible values | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


