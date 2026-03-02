# Job

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | **Read only** The unique Square-assigned ID of the job. If you need a job ID for an API request, call [ListJobs](api-endpoint:Team-ListJobs) or use the ID returned when you created the job. You can also get job IDs from a team member's wage setting. | [optional]
**title** | Option<**String**> | The title of the job. | [optional]
**is_tip_eligible** | Option<**bool**> | Indicates whether team members can earn tips for the job. | [optional]
**created_at** | Option<**String**> | The timestamp when the job was created, in RFC 3339 format. | [optional][readonly]
**updated_at** | Option<**String**> | The timestamp when the job was last updated, in RFC 3339 format. | [optional][readonly]
**version** | Option<**i32**> | **Read only** The current version of the job. Include this field in `UpdateJob` requests to enable [optimistic concurrency](https://developer.squareup.com/docs/working-with-apis/optimistic-concurrency) control and avoid overwrites from concurrent requests. Requests fail if the provided version doesn't match the server version at the time of the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


