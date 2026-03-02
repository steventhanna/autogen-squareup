# WageSetting

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**team_member_id** | Option<**String**> | The ID of the team member associated with the wage setting. | [optional]
**job_assignments** | Option<[**Vec<models::JobAssignment>**](JobAssignment.md)> | **Required** The ordered list of jobs that the team member is assigned to. The first job assignment is considered the team member's primary job. | [optional]
**is_overtime_exempt** | Option<**bool**> | Whether the team member is exempt from the overtime rules of the seller's country. | [optional]
**version** | Option<**i32**> | **Read only** Used for resolving concurrency issues. The request fails if the version provided does not match the server version at the time of the request. If not provided, Square executes a blind write, potentially overwriting data from another write. For more information, see [optimistic concurrency](https://developer.squareup.com/docs/working-with-apis/optimistic-concurrency). | [optional]
**created_at** | Option<**String**> | The timestamp when the wage setting was created, in RFC 3339 format. | [optional][readonly]
**updated_at** | Option<**String**> | The timestamp when the wage setting was last updated, in RFC 3339 format. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


