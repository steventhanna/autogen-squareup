# PayoutEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique ID for the payout entry. | 
**payout_id** | **String** | The ID of the payout entries’ associated payout. | 
**effective_at** | Option<**String**> | The timestamp of when the payout entry affected the balance, in RFC 3339 format. | [optional]
**r#type** | Option<[**models::ActivityType**](ActivityType.md)> |  | [optional]
**gross_amount_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**fee_amount_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**net_amount_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**type_app_fee_revenue_details** | Option<[**models::PaymentBalanceActivityAppFeeRevenueDetail**](PaymentBalanceActivityAppFeeRevenueDetail.md)> |  | [optional]
**type_app_fee_refund_details** | Option<[**models::PaymentBalanceActivityAppFeeRefundDetail**](PaymentBalanceActivityAppFeeRefundDetail.md)> |  | [optional]
**type_automatic_savings_details** | Option<[**models::PaymentBalanceActivityAutomaticSavingsDetail**](PaymentBalanceActivityAutomaticSavingsDetail.md)> |  | [optional]
**type_automatic_savings_reversed_details** | Option<[**models::PaymentBalanceActivityAutomaticSavingsReversedDetail**](PaymentBalanceActivityAutomaticSavingsReversedDetail.md)> |  | [optional]
**type_charge_details** | Option<[**models::PaymentBalanceActivityChargeDetail**](PaymentBalanceActivityChargeDetail.md)> |  | [optional]
**type_deposit_fee_details** | Option<[**models::PaymentBalanceActivityDepositFeeDetail**](PaymentBalanceActivityDepositFeeDetail.md)> |  | [optional]
**type_deposit_fee_reversed_details** | Option<[**models::PaymentBalanceActivityDepositFeeReversedDetail**](PaymentBalanceActivityDepositFeeReversedDetail.md)> |  | [optional]
**type_dispute_details** | Option<[**models::PaymentBalanceActivityDisputeDetail**](PaymentBalanceActivityDisputeDetail.md)> |  | [optional]
**type_fee_details** | Option<[**models::PaymentBalanceActivityFeeDetail**](PaymentBalanceActivityFeeDetail.md)> |  | [optional]
**type_free_processing_details** | Option<[**models::PaymentBalanceActivityFreeProcessingDetail**](PaymentBalanceActivityFreeProcessingDetail.md)> |  | [optional]
**type_hold_adjustment_details** | Option<[**models::PaymentBalanceActivityHoldAdjustmentDetail**](PaymentBalanceActivityHoldAdjustmentDetail.md)> |  | [optional]
**type_open_dispute_details** | Option<[**models::PaymentBalanceActivityOpenDisputeDetail**](PaymentBalanceActivityOpenDisputeDetail.md)> |  | [optional]
**type_other_details** | Option<[**models::PaymentBalanceActivityOtherDetail**](PaymentBalanceActivityOtherDetail.md)> |  | [optional]
**type_other_adjustment_details** | Option<[**models::PaymentBalanceActivityOtherAdjustmentDetail**](PaymentBalanceActivityOtherAdjustmentDetail.md)> |  | [optional]
**type_refund_details** | Option<[**models::PaymentBalanceActivityRefundDetail**](PaymentBalanceActivityRefundDetail.md)> |  | [optional]
**type_release_adjustment_details** | Option<[**models::PaymentBalanceActivityReleaseAdjustmentDetail**](PaymentBalanceActivityReleaseAdjustmentDetail.md)> |  | [optional]
**type_reserve_hold_details** | Option<[**models::PaymentBalanceActivityReserveHoldDetail**](PaymentBalanceActivityReserveHoldDetail.md)> |  | [optional]
**type_reserve_release_details** | Option<[**models::PaymentBalanceActivityReserveReleaseDetail**](PaymentBalanceActivityReserveReleaseDetail.md)> |  | [optional]
**type_square_capital_payment_details** | Option<[**models::PaymentBalanceActivitySquareCapitalPaymentDetail**](PaymentBalanceActivitySquareCapitalPaymentDetail.md)> |  | [optional]
**type_square_capital_reversed_payment_details** | Option<[**models::PaymentBalanceActivitySquareCapitalReversedPaymentDetail**](PaymentBalanceActivitySquareCapitalReversedPaymentDetail.md)> |  | [optional]
**type_tax_on_fee_details** | Option<[**models::PaymentBalanceActivityTaxOnFeeDetail**](PaymentBalanceActivityTaxOnFeeDetail.md)> |  | [optional]
**type_third_party_fee_details** | Option<[**models::PaymentBalanceActivityThirdPartyFeeDetail**](PaymentBalanceActivityThirdPartyFeeDetail.md)> |  | [optional]
**type_third_party_fee_refund_details** | Option<[**models::PaymentBalanceActivityThirdPartyFeeRefundDetail**](PaymentBalanceActivityThirdPartyFeeRefundDetail.md)> |  | [optional]
**type_square_payroll_transfer_details** | Option<[**models::PaymentBalanceActivitySquarePayrollTransferDetail**](PaymentBalanceActivitySquarePayrollTransferDetail.md)> |  | [optional]
**type_square_payroll_transfer_reversed_details** | Option<[**models::PaymentBalanceActivitySquarePayrollTransferReversedDetail**](PaymentBalanceActivitySquarePayrollTransferReversedDetail.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


