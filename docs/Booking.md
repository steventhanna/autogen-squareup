# Booking

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A unique ID of this object representing a booking. | [optional][readonly]
**version** | Option<**i32**> | The revision number for the booking used for optimistic concurrency. | [optional]
**status** | Option<[**models::BookingStatus**](BookingStatus.md)> |  | [optional]
**created_at** | Option<**String**> | The RFC 3339 timestamp specifying the creation time of this booking. | [optional][readonly]
**updated_at** | Option<**String**> | The RFC 3339 timestamp specifying the most recent update time of this booking. | [optional][readonly]
**start_at** | Option<**String**> | The RFC 3339 timestamp specifying the starting time of this booking. | [optional]
**location_id** | Option<**String**> | The ID of the [Location](entity:Location) object representing the location where the booked service is provided. Once set when the booking is created, its value cannot be changed. | [optional]
**customer_id** | Option<**String**> | The ID of the [Customer](entity:Customer) object representing the customer receiving the booked service. | [optional]
**customer_note** | Option<**String**> | The free-text field for the customer to supply notes about the booking. For example, the note can be preferences that cannot be expressed by supported attributes of a relevant [CatalogObject](entity:CatalogObject) instance. | [optional]
**seller_note** | Option<**String**> | The free-text field for the seller to supply notes about the booking. For example, the note can be preferences that cannot be expressed by supported attributes of a specific [CatalogObject](entity:CatalogObject) instance. This field should not be visible to customers. | [optional]
**appointment_segments** | Option<[**Vec<models::AppointmentSegment>**](AppointmentSegment.md)> | A list of appointment segments for this booking. | [optional]
**transition_time_minutes** | Option<**i32**> | Additional time at the end of a booking. Applications should not make this field visible to customers of a seller. | [optional][readonly]
**all_day** | Option<**bool**> | Whether the booking is of a full business day. | [optional][readonly]
**location_type** | Option<[**models::BusinessAppointmentSettingsBookingLocationType**](BusinessAppointmentSettingsBookingLocationType.md)> |  | [optional]
**creator_details** | Option<[**models::BookingCreatorDetails**](BookingCreatorDetails.md)> |  | [optional]
**source** | Option<[**models::BookingBookingSource**](BookingBookingSource.md)> |  | [optional]
**address** | Option<[**models::Address**](Address.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


