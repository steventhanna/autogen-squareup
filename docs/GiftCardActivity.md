# GiftCardActivity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The Square-assigned ID of the gift card activity. | [optional][readonly]
**r#type** | [**models::GiftCardActivityType**](GiftCardActivityType.md) |  | 
**location_id** | **String** | The ID of the [business location](entity:Location) where the activity occurred. | 
**created_at** | Option<**String**> | The timestamp when the gift card activity was created, in RFC 3339 format. | [optional][readonly]
**gift_card_id** | Option<**String**> | The gift card ID. When creating a gift card activity, `gift_card_id` is not required if  `gift_card_gan` is specified. | [optional]
**gift_card_gan** | Option<**String**> | The gift card account number (GAN). When creating a gift card activity, `gift_card_gan`  is not required if `gift_card_id` is specified. | [optional]
**gift_card_balance_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**load_activity_details** | Option<[**models::GiftCardActivityLoad**](GiftCardActivityLoad.md)> |  | [optional]
**activate_activity_details** | Option<[**models::GiftCardActivityActivate**](GiftCardActivityActivate.md)> |  | [optional]
**redeem_activity_details** | Option<[**models::GiftCardActivityRedeem**](GiftCardActivityRedeem.md)> |  | [optional]
**clear_balance_activity_details** | Option<[**models::GiftCardActivityClearBalance**](GiftCardActivityClearBalance.md)> |  | [optional]
**deactivate_activity_details** | Option<[**models::GiftCardActivityDeactivate**](GiftCardActivityDeactivate.md)> |  | [optional]
**adjust_increment_activity_details** | Option<[**models::GiftCardActivityAdjustIncrement**](GiftCardActivityAdjustIncrement.md)> |  | [optional]
**adjust_decrement_activity_details** | Option<[**models::GiftCardActivityAdjustDecrement**](GiftCardActivityAdjustDecrement.md)> |  | [optional]
**refund_activity_details** | Option<[**models::GiftCardActivityRefund**](GiftCardActivityRefund.md)> |  | [optional]
**unlinked_activity_refund_activity_details** | Option<[**models::GiftCardActivityUnlinkedActivityRefund**](GiftCardActivityUnlinkedActivityRefund.md)> |  | [optional]
**import_activity_details** | Option<[**models::GiftCardActivityImport**](GiftCardActivityImport.md)> |  | [optional]
**block_activity_details** | Option<[**models::GiftCardActivityBlock**](GiftCardActivityBlock.md)> |  | [optional]
**unblock_activity_details** | Option<[**models::GiftCardActivityUnblock**](GiftCardActivityUnblock.md)> |  | [optional]
**import_reversal_activity_details** | Option<[**models::GiftCardActivityImportReversal**](GiftCardActivityImportReversal.md)> |  | [optional]
**transfer_balance_to_activity_details** | Option<[**models::GiftCardActivityTransferBalanceTo**](GiftCardActivityTransferBalanceTo.md)> |  | [optional]
**transfer_balance_from_activity_details** | Option<[**models::GiftCardActivityTransferBalanceFrom**](GiftCardActivityTransferBalanceFrom.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


