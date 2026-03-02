# BulkUpdateTeamMembersResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**team_members** | Option<[**std::collections::HashMap<String, models::UpdateTeamMemberResponse>**](UpdateTeamMemberResponse.md)> | The successfully updated `TeamMember` objects. Each key is the `team_member_id` that maps to the `UpdateTeamMemberRequest`. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | The errors that occurred during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


