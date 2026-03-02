# \BookingsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_retrieve_bookings**](BookingsApi.md#bulk_retrieve_bookings) | **POST** /v2/bookings/bulk-retrieve | BulkRetrieveBookings
[**bulk_retrieve_team_member_booking_profiles**](BookingsApi.md#bulk_retrieve_team_member_booking_profiles) | **POST** /v2/bookings/team-member-booking-profiles/bulk-retrieve | BulkRetrieveTeamMemberBookingProfiles
[**cancel_booking**](BookingsApi.md#cancel_booking) | **POST** /v2/bookings/{booking_id}/cancel | CancelBooking
[**create_booking**](BookingsApi.md#create_booking) | **POST** /v2/bookings | CreateBooking
[**list_bookings**](BookingsApi.md#list_bookings) | **GET** /v2/bookings | ListBookings
[**list_location_booking_profiles**](BookingsApi.md#list_location_booking_profiles) | **GET** /v2/bookings/location-booking-profiles | ListLocationBookingProfiles
[**list_team_member_booking_profiles**](BookingsApi.md#list_team_member_booking_profiles) | **GET** /v2/bookings/team-member-booking-profiles | ListTeamMemberBookingProfiles
[**retrieve_booking**](BookingsApi.md#retrieve_booking) | **GET** /v2/bookings/{booking_id} | RetrieveBooking
[**retrieve_business_booking_profile**](BookingsApi.md#retrieve_business_booking_profile) | **GET** /v2/bookings/business-booking-profile | RetrieveBusinessBookingProfile
[**retrieve_location_booking_profile**](BookingsApi.md#retrieve_location_booking_profile) | **GET** /v2/bookings/location-booking-profiles/{location_id} | RetrieveLocationBookingProfile
[**retrieve_team_member_booking_profile**](BookingsApi.md#retrieve_team_member_booking_profile) | **GET** /v2/bookings/team-member-booking-profiles/{team_member_id} | RetrieveTeamMemberBookingProfile
[**search_availability**](BookingsApi.md#search_availability) | **POST** /v2/bookings/availability/search | SearchAvailability
[**update_booking**](BookingsApi.md#update_booking) | **PUT** /v2/bookings/{booking_id} | UpdateBooking



## bulk_retrieve_bookings

> models::BulkRetrieveBookingsResponse bulk_retrieve_bookings(bulk_retrieve_bookings_request)
BulkRetrieveBookings

Bulk-Retrieves a list of bookings by booking IDs.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_READ` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_READ` and `APPOINTMENTS_READ` for the OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_retrieve_bookings_request** | [**BulkRetrieveBookingsRequest**](BulkRetrieveBookingsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkRetrieveBookingsResponse**](BulkRetrieveBookingsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_retrieve_team_member_booking_profiles

> models::BulkRetrieveTeamMemberBookingProfilesResponse bulk_retrieve_team_member_booking_profiles(bulk_retrieve_team_member_booking_profiles_request)
BulkRetrieveTeamMemberBookingProfiles

Retrieves one or more team members' booking profiles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_retrieve_team_member_booking_profiles_request** | [**BulkRetrieveTeamMemberBookingProfilesRequest**](BulkRetrieveTeamMemberBookingProfilesRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkRetrieveTeamMemberBookingProfilesResponse**](BulkRetrieveTeamMemberBookingProfilesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_booking

> models::CancelBookingResponse cancel_booking(booking_id, cancel_booking_request)
CancelBooking

Cancels an existing booking.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_WRITE` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_WRITE` and `APPOINTMENTS_WRITE` for the OAuth scope.  For calls to this endpoint with seller-level permissions to succeed, the seller must have subscribed to *Appointments Plus* or *Appointments Premium*.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**booking_id** | **String** | The ID of the [Booking](entity:Booking) object representing the to-be-cancelled booking. | [required] |
**cancel_booking_request** | [**CancelBookingRequest**](CancelBookingRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CancelBookingResponse**](CancelBookingResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_booking

> models::CreateBookingResponse create_booking(create_booking_request)
CreateBooking

Creates a booking.  The required input must include the following: - `Booking.location_id` - `Booking.start_at` - `Booking.AppointmentSegment.team_member_id` - `Booking.AppointmentSegment.service_variation_id` - `Booking.AppointmentSegment.service_variation_version`  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_WRITE` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_WRITE` and `APPOINTMENTS_WRITE` for the OAuth scope.  For calls to this endpoint with seller-level permissions to succeed, the seller must have subscribed to *Appointments Plus* or *Appointments Premium*.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_booking_request** | [**CreateBookingRequest**](CreateBookingRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateBookingResponse**](CreateBookingResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_bookings

> models::ListBookingsResponse list_bookings(limit, cursor, customer_id, team_member_id, location_id, start_at_min, start_at_max)
ListBookings

Retrieve a collection of bookings.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_READ` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_READ` and `APPOINTMENTS_READ` for the OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum number of results per page to return in a paged response. |  |
**cursor** | Option<**String**> | The pagination cursor from the preceding response to return the next page of the results. Do not set this when retrieving the first page of the results. |  |
**customer_id** | Option<**String**> | The [customer](entity:Customer) for whom to retrieve bookings. If this is not set, bookings for all customers are retrieved. |  |
**team_member_id** | Option<**String**> | The team member for whom to retrieve bookings. If this is not set, bookings of all members are retrieved. |  |
**location_id** | Option<**String**> | The location for which to retrieve bookings. If this is not set, all locations' bookings are retrieved. |  |
**start_at_min** | Option<**String**> | The RFC 3339 timestamp specifying the earliest of the start time. If this is not set, the current time is used. |  |
**start_at_max** | Option<**String**> | The RFC 3339 timestamp specifying the latest of the start time. If this is not set, the time of 31 days after `start_at_min` is used. |  |

### Return type

[**models::ListBookingsResponse**](ListBookingsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_location_booking_profiles

> models::ListLocationBookingProfilesResponse list_location_booking_profiles(limit, cursor)
ListLocationBookingProfiles

Lists location booking profiles of a seller.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum number of results to return in a paged response. |  |
**cursor** | Option<**String**> | The pagination cursor from the preceding response to return the next page of the results. Do not set this when retrieving the first page of the results. |  |

### Return type

[**models::ListLocationBookingProfilesResponse**](ListLocationBookingProfilesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_team_member_booking_profiles

> models::ListTeamMemberBookingProfilesResponse list_team_member_booking_profiles(bookable_only, limit, cursor, location_id)
ListTeamMemberBookingProfiles

Lists booking profiles for team members.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bookable_only** | Option<**bool**> | Indicates whether to include only bookable team members in the returned result (`true`) or not (`false`). |  |[default to false]
**limit** | Option<**i32**> | The maximum number of results to return in a paged response. |  |
**cursor** | Option<**String**> | The pagination cursor from the preceding response to return the next page of the results. Do not set this when retrieving the first page of the results. |  |
**location_id** | Option<**String**> | Indicates whether to include only team members enabled at the given location in the returned result. |  |

### Return type

[**models::ListTeamMemberBookingProfilesResponse**](ListTeamMemberBookingProfilesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_booking

> models::RetrieveBookingResponse retrieve_booking(booking_id)
RetrieveBooking

Retrieves a booking.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_READ` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_READ` and `APPOINTMENTS_READ` for the OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**booking_id** | **String** | The ID of the [Booking](entity:Booking) object representing the to-be-retrieved booking. | [required] |

### Return type

[**models::RetrieveBookingResponse**](RetrieveBookingResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_business_booking_profile

> models::RetrieveBusinessBookingProfileResponse retrieve_business_booking_profile()
RetrieveBusinessBookingProfile

Retrieves a seller's booking profile.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RetrieveBusinessBookingProfileResponse**](RetrieveBusinessBookingProfileResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_location_booking_profile

> models::RetrieveLocationBookingProfileResponse retrieve_location_booking_profile(location_id)
RetrieveLocationBookingProfile

Retrieves a seller's location booking profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the location to retrieve the booking profile. | [required] |

### Return type

[**models::RetrieveLocationBookingProfileResponse**](RetrieveLocationBookingProfileResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_team_member_booking_profile

> models::RetrieveTeamMemberBookingProfileResponse retrieve_team_member_booking_profile(team_member_id)
RetrieveTeamMemberBookingProfile

Retrieves a team member's booking profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_member_id** | **String** | The ID of the team member to retrieve. | [required] |

### Return type

[**models::RetrieveTeamMemberBookingProfileResponse**](RetrieveTeamMemberBookingProfileResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_availability

> models::SearchAvailabilityResponse search_availability(search_availability_request)
SearchAvailability

Searches for availabilities for booking.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_READ` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_READ` and `APPOINTMENTS_READ` for the OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_availability_request** | [**SearchAvailabilityRequest**](SearchAvailabilityRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchAvailabilityResponse**](SearchAvailabilityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_booking

> models::UpdateBookingResponse update_booking(booking_id, update_booking_request)
UpdateBooking

Updates a booking.  To call this endpoint with buyer-level permissions, set `APPOINTMENTS_WRITE` for the OAuth scope. To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_WRITE` and `APPOINTMENTS_WRITE` for the OAuth scope.  For calls to this endpoint with seller-level permissions to succeed, the seller must have subscribed to *Appointments Plus* or *Appointments Premium*.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**booking_id** | **String** | The ID of the [Booking](entity:Booking) object representing the to-be-updated booking. | [required] |
**update_booking_request** | [**UpdateBookingRequest**](UpdateBookingRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateBookingResponse**](UpdateBookingResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

