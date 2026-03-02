# BulkUpdateTeamMembersRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**team_members** | [**std::collections::HashMap<String, models::UpdateTeamMemberRequest>**](UpdateTeamMemberRequest.md) | The data used to update the `TeamMember` objects. Each key is the `team_member_id` that maps to the `UpdateTeamMemberRequest`. The maximum number of update objects is 25.  For each team member, include the fields to add, change, or clear. Fields can be cleared using a null value. To update `wage_setting.job_assignments`, you must provide the complete list of job assignments. If needed, call [ListJobs](api-endpoint:Team-ListJobs) to get the required `job_id` values. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


