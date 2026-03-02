# BusinessAppointmentSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**location_types** | Option<[**Vec<models::BusinessAppointmentSettingsBookingLocationType>**](BusinessAppointmentSettingsBookingLocationType.md)> | Types of the location allowed for bookings. See [BusinessAppointmentSettingsBookingLocationType](#type-businessappointmentsettingsbookinglocationtype) for possible values | [optional]
**alignment_time** | Option<[**models::BusinessAppointmentSettingsAlignmentTime**](BusinessAppointmentSettingsAlignmentTime.md)> |  | [optional]
**min_booking_lead_time_seconds** | Option<**i32**> | The minimum lead time in seconds before a service can be booked. A booking must be created at least this amount of time before its starting time. | [optional]
**max_booking_lead_time_seconds** | Option<**i32**> | The maximum lead time in seconds before a service can be booked. A booking must be created at most this amount of time before its starting time. | [optional]
**any_team_member_booking_enabled** | Option<**bool**> | Indicates whether a customer can choose from all available time slots and have a staff member assigned automatically (`true`) or not (`false`). | [optional]
**multiple_service_booking_enabled** | Option<**bool**> | Indicates whether a customer can book multiple services in a single online booking. | [optional]
**max_appointments_per_day_limit_type** | Option<[**models::BusinessAppointmentSettingsMaxAppointmentsPerDayLimitType**](BusinessAppointmentSettingsMaxAppointmentsPerDayLimitType.md)> |  | [optional]
**max_appointments_per_day_limit** | Option<**i32**> | The maximum number of daily appointments per team member or per location. | [optional]
**cancellation_window_seconds** | Option<**i32**> | The cut-off time in seconds for allowing clients to cancel or reschedule an appointment. | [optional]
**cancellation_fee_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**cancellation_policy** | Option<[**models::BusinessAppointmentSettingsCancellationPolicy**](BusinessAppointmentSettingsCancellationPolicy.md)> |  | [optional]
**cancellation_policy_text** | Option<**String**> | The free-form text of the seller's cancellation policy. | [optional]
**skip_booking_flow_staff_selection** | Option<**bool**> | Indicates whether customers has an assigned staff member (`true`) or can select s staff member of their choice (`false`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


