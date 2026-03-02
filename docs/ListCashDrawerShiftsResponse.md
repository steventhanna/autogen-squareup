# ListCashDrawerShiftsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Opaque cursor for fetching the next page of results. Cursor is not present in the last page of results. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**cash_drawer_shifts** | Option<[**Vec<models::CashDrawerShiftSummary>**](CashDrawerShiftSummary.md)> | A collection of CashDrawerShiftSummary objects for shifts that match the query. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


