# CreateSubscriptionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique string that identifies this `CreateSubscription` request. If you do not provide a unique string (or provide an empty string as the value), the endpoint treats each request as independent.  For more information, see [Idempotency keys](https://developer.squareup.com/docs/build-basics/common-api-patterns/idempotency). | [optional]
**location_id** | **String** | The ID of the location the subscription is associated with. | 
**plan_variation_id** | Option<**String**> | The ID of the [subscription plan variation](https://developer.squareup.com/docs/subscriptions-api/plans-and-variations#plan-variations) created using the Catalog API. | [optional]
**customer_id** | **String** | The ID of the [customer](entity:Customer) subscribing to the subscription plan variation. | 
**start_date** | Option<**String**> | The `YYYY-MM-DD`-formatted date to start the subscription.  If it is unspecified, the subscription starts immediately. | [optional]
**canceled_date** | Option<**String**> | The `YYYY-MM-DD`-formatted date when the newly created subscription is scheduled for cancellation.   This date overrides the cancellation date set in the plan variation configuration. If the cancellation date is earlier than the end date of a subscription cycle, the subscription stops at the canceled date and the subscriber is sent a prorated invoice at the beginning of the canceled cycle.   When the subscription plan of the newly created subscription has a fixed number of cycles and the `canceled_date` occurs before the subscription plan completes, the specified `canceled_date` sets the date when the subscription stops through the end of the last cycle. | [optional]
**tax_percentage** | Option<**String**> | The tax to add when billing the subscription. The percentage is expressed in decimal form, using a `'.'` as the decimal separator and without a `'%'` sign. For example, a value of 7.5 corresponds to 7.5%. | [optional]
**price_override_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**card_id** | Option<**String**> | The ID of the [subscriber's](entity:Customer) [card](entity:Card) to charge. If it is not specified, the subscriber receives an invoice via email with a link to pay for their subscription. | [optional]
**timezone** | Option<**String**> | The timezone that is used in date calculations for the subscription. If unset, defaults to the location timezone. If a timezone is not configured for the location, defaults to \"America/New_York\". Format: the IANA Timezone Database identifier for the location timezone. For a list of time zones, see [List of tz database time zones](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). | [optional]
**source** | Option<[**models::SubscriptionSource**](SubscriptionSource.md)> |  | [optional]
**monthly_billing_anchor_date** | Option<**i32**> | The day-of-the-month to change the billing date to. | [optional]
**phases** | Option<[**Vec<models::Phase>**](Phase.md)> | array of phases for this subscription | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


