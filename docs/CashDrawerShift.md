# CashDrawerShift

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The shift unique ID. | [optional]
**state** | Option<[**models::CashDrawerShiftState**](CashDrawerShiftState.md)> |  | [optional]
**opened_at** | Option<**String**> | The time when the shift began, in ISO 8601 format. | [optional]
**ended_at** | Option<**String**> | The time when the shift ended, in ISO 8601 format. | [optional]
**closed_at** | Option<**String**> | The time when the shift was closed, in ISO 8601 format. | [optional]
**description** | Option<**String**> | The free-form text description of a cash drawer by an employee. | [optional]
**opened_cash_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**cash_payment_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**cash_refunds_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**cash_paid_in_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**cash_paid_out_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**expected_cash_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**closed_cash_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**device** | Option<[**models::CashDrawerDevice**](CashDrawerDevice.md)> |  | [optional]
**created_at** | Option<**String**> | The shift start time in RFC 3339 format. | [optional][readonly]
**updated_at** | Option<**String**> | The shift updated at time in RFC 3339 format. | [optional][readonly]
**location_id** | Option<**String**> | The ID of the location the cash drawer shift belongs to. | [optional][readonly]
**team_member_ids** | Option<**Vec<String>**> | The IDs of all team members that were logged into Square Point of Sale at any point while the cash drawer shift was open. | [optional][readonly]
**opening_team_member_id** | Option<**String**> | The ID of the team member that started the cash drawer shift. | [optional][readonly]
**ending_team_member_id** | Option<**String**> | The ID of the team member that ended the cash drawer shift. | [optional][readonly]
**closing_team_member_id** | Option<**String**> | The ID of the team member that closed the cash drawer shift by auditing the cash drawer contents. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


