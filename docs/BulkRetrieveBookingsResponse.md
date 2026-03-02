# BulkRetrieveBookingsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bookings** | Option<[**std::collections::HashMap<String, models::RetrieveBookingResponse>**](RetrieveBookingResponse.md)> | Requested bookings returned as a map containing `booking_id` as the key and `RetrieveBookingResponse` as the value. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Errors that occurred during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


