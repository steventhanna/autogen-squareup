# ListCashDrawerShiftEventsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Opaque cursor for fetching the next page. Cursor is not present in the last page of results. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**cash_drawer_shift_events** | Option<[**Vec<models::CashDrawerShiftEvent>**](CashDrawerShiftEvent.md)> | All of the events (payments, refunds, etc.) for a cash drawer during the shift. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


