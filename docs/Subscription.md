# Subscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The Square-assigned ID of the subscription. | [optional][readonly]
**location_id** | Option<**String**> | The ID of the location associated with the subscription. | [optional][readonly]
**plan_variation_id** | Option<**String**> | The ID of the subscribed-to [subscription plan variation](entity:CatalogSubscriptionPlanVariation). | [optional][readonly]
**customer_id** | Option<**String**> | The ID of the subscribing [customer](entity:Customer) profile. | [optional][readonly]
**start_date** | Option<**String**> | The `YYYY-MM-DD`-formatted date (for example, 2013-01-15) to start the subscription. | [optional][readonly]
**canceled_date** | Option<**String**> | The `YYYY-MM-DD`-formatted date (for example, 2013-01-15) to cancel the subscription,  when the subscription status changes to `CANCELED` and the subscription billing stops.  If this field is not set, the subscription ends according its subscription plan.  This field cannot be updated, other than being cleared. | [optional]
**charged_through_date** | Option<**String**> | The `YYYY-MM-DD`-formatted date up to when the subscriber is invoiced for the subscription.  After the invoice is sent for a given billing period, this date will be the last day of the billing period. For example, suppose for the month of May a subscriber gets an invoice (or charged the card) on May 1. For the monthly billing scenario, this date is then set to May 31. | [optional][readonly]
**status** | Option<[**models::SubscriptionStatus**](SubscriptionStatus.md)> |  | [optional]
**tax_percentage** | Option<**String**> | The tax amount applied when billing the subscription. The percentage is expressed in decimal form, using a `'.'` as the decimal separator and without a `'%'` sign. For example, a value of `7.5` corresponds to 7.5%. | [optional]
**invoice_ids** | Option<**Vec<String>**> | The IDs of the [invoices](entity:Invoice) created for the subscription, listed in order when the invoices were created (newest invoices appear first). | [optional][readonly]
**price_override_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**version** | Option<**i64**> | The version of the object. When updating an object, the version supplied must match the version in the database, otherwise the write will be rejected as conflicting. | [optional]
**created_at** | Option<**String**> | The timestamp when the subscription was created, in RFC 3339 format. | [optional][readonly]
**card_id** | Option<**String**> | The ID of the [subscriber's](entity:Customer) [card](entity:Card) used to charge for the subscription. | [optional]
**timezone** | Option<**String**> | Timezone that will be used in date calculations for the subscription. Defaults to the timezone of the location based on `location_id`. Format: the IANA Timezone Database identifier for the location timezone (for example, `America/Los_Angeles`). | [optional][readonly]
**source** | Option<[**models::SubscriptionSource**](SubscriptionSource.md)> |  | [optional]
**actions** | Option<[**Vec<models::SubscriptionAction>**](SubscriptionAction.md)> | The list of scheduled actions on this subscription. It is set only in the response from   [RetrieveSubscription](api-endpoint:Subscriptions-RetrieveSubscription) with the query parameter of `include=actions` or from  [SearchSubscriptions](api-endpoint:Subscriptions-SearchSubscriptions) with the input parameter  of `include:[\"actions\"]`. | [optional]
**monthly_billing_anchor_date** | Option<**i32**> | The day of the month on which the subscription will issue invoices and publish orders. | [optional][readonly]
**phases** | Option<[**Vec<models::Phase>**](Phase.md)> | array of phases for this subscription | [optional][readonly]
**completed_date** | Option<**String**> | The `YYYY-MM-DD`-formatted date when the subscription enters a terminal state. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


