# SearchOrdersFulfillmentFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fulfillment_types** | Option<[**Vec<models::FulfillmentType>**](FulfillmentType.md)> | A list of [fulfillment types](entity:FulfillmentType) to filter for. The list returns orders if any of its fulfillments match any of the fulfillment types listed in this field. See [FulfillmentType](#type-fulfillmenttype) for possible values | [optional]
**fulfillment_states** | Option<[**Vec<models::FulfillmentState>**](FulfillmentState.md)> | A list of [fulfillment states](entity:FulfillmentState) to filter for. The list returns orders if any of its fulfillments match any of the fulfillment states listed in this field. See [FulfillmentState](#type-fulfillmentstate) for possible values | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


