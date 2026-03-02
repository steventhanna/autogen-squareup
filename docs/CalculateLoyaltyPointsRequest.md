# CalculateLoyaltyPointsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | Option<**String**> | The [order](entity:Order) ID for which to calculate the points. Specify this field if your application uses the Orders API to process orders. Otherwise, specify the `transaction_amount_money`. | [optional]
**transaction_amount_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**loyalty_account_id** | Option<**String**> | The ID of the target [loyalty account](entity:LoyaltyAccount). Optionally specify this field if your application uses the Orders API to process orders.  If specified, the `promotion_points` field in the response shows the number of points the buyer would earn from the purchase. In this case, Square uses the account ID to determine whether the promotion's `trigger_limit` (the maximum number of times that a buyer can trigger the promotion) has been reached. If not specified, the `promotion_points` field shows the number of points the purchase qualifies for regardless of the trigger limit. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


