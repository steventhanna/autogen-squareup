# Payout

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique ID for the payout. | 
**status** | Option<[**models::PayoutStatus**](PayoutStatus.md)> |  | [optional]
**location_id** | **String** | The ID of the location associated with the payout. | 
**created_at** | Option<**String**> | The timestamp of when the payout was created and submitted for deposit to the seller's banking destination, in RFC 3339 format. | [optional]
**updated_at** | Option<**String**> | The timestamp of when the payout was last updated, in RFC 3339 format. | [optional]
**amount_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**destination** | Option<[**models::Destination**](Destination.md)> |  | [optional]
**version** | Option<**i32**> | The version number, which is incremented each time an update is made to this payout record. The version number helps developers receive event notifications or feeds out of order. | [optional]
**r#type** | Option<[**models::PayoutType**](PayoutType.md)> |  | [optional]
**payout_fee** | Option<[**Vec<models::PayoutFee>**](PayoutFee.md)> | A list of transfer fees and any taxes on the fees assessed by Square for this payout. | [optional]
**arrival_date** | Option<**String**> | The calendar date, in ISO 8601 format (YYYY-MM-DD), when the payout is due to arrive in the seller’s banking destination. | [optional]
**end_to_end_id** | Option<**String**> | A unique ID for each `Payout` object that might also appear on the seller’s bank statement. You can use this ID to automate the process of reconciling each payout with the corresponding line item on the bank statement. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


