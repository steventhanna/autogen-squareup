# BulkDeleteBookingCustomAttributesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**values** | Option<[**std::collections::HashMap<String, models::BookingCustomAttributeDeleteResponse>**](BookingCustomAttributeDeleteResponse.md)> | A map of responses that correspond to individual delete requests. Each response has the same ID as the corresponding request and contains `booking_id` and  `errors` field. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


