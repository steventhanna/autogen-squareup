# CashDrawerShiftSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The shift unique ID. | [optional]
**state** | Option<[**models::CashDrawerShiftState**](CashDrawerShiftState.md)> |  | [optional]
**opened_at** | Option<**String**> | The shift start time in ISO 8601 format. | [optional]
**ended_at** | Option<**String**> | The shift end time in ISO 8601 format. | [optional]
**closed_at** | Option<**String**> | The shift close time in ISO 8601 format. | [optional]
**description** | Option<**String**> | An employee free-text description of a cash drawer shift. | [optional]
**opened_cash_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**expected_cash_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**closed_cash_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**created_at** | Option<**String**> | The shift start time in RFC 3339 format. | [optional][readonly]
**updated_at** | Option<**String**> | The shift updated at time in RFC 3339 format. | [optional][readonly]
**location_id** | Option<**String**> | The ID of the location the cash drawer shift belongs to. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


