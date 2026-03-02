# ListBookingsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bookings** | Option<[**Vec<models::Booking>**](Booking.md)> | The list of targeted bookings. | [optional]
**cursor** | Option<**String**> | The pagination cursor to be used in the subsequent request to get the next page of the results. Stop retrieving the next page of the results when the cursor is not set. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Errors that occurred during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


