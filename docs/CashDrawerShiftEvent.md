# CashDrawerShiftEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique ID of the event. | [optional]
**event_type** | Option<[**models::CashDrawerEventType**](CashDrawerEventType.md)> |  | [optional]
**event_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**created_at** | Option<**String**> | The event time in RFC 3339 format. | [optional][readonly]
**description** | Option<**String**> | An optional description of the event, entered by the employee that created the event. | [optional]
**team_member_id** | Option<**String**> | The ID of the team member that created the event. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


