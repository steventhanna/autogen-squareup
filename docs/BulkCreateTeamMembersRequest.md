# BulkCreateTeamMembersRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**team_members** | [**std::collections::HashMap<String, models::CreateTeamMemberRequest>**](CreateTeamMemberRequest.md) | The data used to create the `TeamMember` objects. Each key is the `idempotency_key` that maps to the `CreateTeamMemberRequest`. The maximum number of create objects is 25.  If you include a team member's `wage_setting`, you must provide `job_id` for each job assignment. To get job IDs, call [ListJobs](api-endpoint:Team-ListJobs). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


