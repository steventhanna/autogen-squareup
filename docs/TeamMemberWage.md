# TeamMemberWage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The UUID for this object. | [optional]
**team_member_id** | Option<**String**> | The `TeamMember` that this wage is assigned to. | [optional]
**title** | Option<**String**> | The job title that this wage relates to. | [optional]
**hourly_rate** | Option<[**models::Money**](Money.md)> |  | [optional]
**job_id** | Option<**String**> | An identifier for the [job](entity:Job) that this wage relates to. | [optional]
**tip_eligible** | Option<**bool**> | Whether team members are eligible for tips when working this job. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


