# BusinessBookingProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_id** | Option<**String**> | The ID of the seller, obtainable using the Merchants API. | [optional]
**created_at** | Option<**String**> | The RFC 3339 timestamp specifying the booking's creation time. | [optional][readonly]
**booking_enabled** | Option<**bool**> | Indicates whether the seller is open for booking. | [optional]
**customer_timezone_choice** | Option<[**models::BusinessBookingProfileCustomerTimezoneChoice**](BusinessBookingProfileCustomerTimezoneChoice.md)> |  | [optional]
**booking_policy** | Option<[**models::BusinessBookingProfileBookingPolicy**](BusinessBookingProfileBookingPolicy.md)> |  | [optional]
**allow_user_cancel** | Option<**bool**> | Indicates whether customers can cancel or reschedule their own bookings (`true`) or not (`false`). | [optional]
**business_appointment_settings** | Option<[**models::BusinessAppointmentSettings**](BusinessAppointmentSettings.md)> |  | [optional]
**support_seller_level_writes** | Option<**bool**> | Indicates whether the seller's subscription to Square Appointments supports creating, updating or canceling an appointment through the API (`true`) or not (`false`) using seller permission. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


