# JobAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_title** | Option<**String**> | The title of the job. | [optional]
**pay_type** | [**models::JobAssignmentPayType**](JobAssignmentPayType.md) |  | 
**hourly_rate** | Option<[**models::Money**](Money.md)> |  | [optional]
**annual_rate** | Option<[**models::Money**](Money.md)> |  | [optional]
**weekly_hours** | Option<**i32**> | The planned hours per week for the job. Set if the job `PayType` is `SALARY`. | [optional]
**job_id** | Option<**String**> | The ID of the [job](entity:Job). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


