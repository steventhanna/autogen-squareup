# SearchTeamMembersFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**location_ids** | Option<**Vec<String>**> | When present, filters by team members assigned to the specified locations. When empty, includes team members assigned to any location. | [optional]
**status** | Option<[**models::TeamMemberStatus**](TeamMemberStatus.md)> |  | [optional]
**is_owner** | Option<**bool**> | When present and set to true, returns the team member who is the owner of the Square account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


