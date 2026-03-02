# BulkCreateTeamMembersResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**team_members** | Option<[**std::collections::HashMap<String, models::CreateTeamMemberResponse>**](CreateTeamMemberResponse.md)> | The successfully created `TeamMember` objects. Each key is the `idempotency_key` that maps to the `CreateTeamMemberRequest`. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | The errors that occurred during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


