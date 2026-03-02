# TimecardWage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | The name of the job performed during this timecard. | [optional]
**hourly_rate** | Option<[**models::Money**](Money.md)> |  | [optional]
**job_id** | Option<**String**> | The ID of the [job](entity:Job) performed for this timecard. Square labor-reporting UIs might group timecards together by ID. | [optional][readonly]
**tip_eligible** | Option<**bool**> | Whether team members are eligible for tips when working this job. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


