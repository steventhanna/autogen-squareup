# CardPaymentDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | The card payment's current state. The state can be AUTHORIZED, CAPTURED, VOIDED, or FAILED. | [optional][readonly]
**card** | Option<[**models::Card**](Card.md)> |  | [optional]
**entry_method** | Option<**String**> | The method used to enter the card's details for the payment. The method can be `KEYED`, `SWIPED`, `EMV`, `ON_FILE`, or `CONTACTLESS`. | [optional][readonly]
**cvv_status** | Option<**String**> | The status code returned from the Card Verification Value (CVV) check. The code can be `CVV_ACCEPTED`, `CVV_REJECTED`, or `CVV_NOT_CHECKED`. | [optional][readonly]
**avs_status** | Option<**String**> | The status code returned from the Address Verification System (AVS) check. The code can be `AVS_ACCEPTED`, `AVS_REJECTED`, or `AVS_NOT_CHECKED`. | [optional][readonly]
**auth_result_code** | Option<**String**> | The status code returned by the card issuer that describes the payment's authorization status. | [optional][readonly]
**application_identifier** | Option<**String**> | For EMV payments, the application ID identifies the EMV application used for the payment. | [optional][readonly]
**application_name** | Option<**String**> | For EMV payments, the human-readable name of the EMV application used for the payment. | [optional][readonly]
**application_cryptogram** | Option<**String**> | For EMV payments, the cryptogram generated for the payment. | [optional][readonly]
**verification_method** | Option<**String**> | For EMV payments, the method used to verify the cardholder's identity. The method can be `PIN`, `SIGNATURE`, `PIN_AND_SIGNATURE`, `ON_DEVICE`, or `NONE`. | [optional][readonly]
**verification_results** | Option<**String**> | For EMV payments, the results of the cardholder verification. The result can be `SUCCESS`, `FAILURE`, or `UNKNOWN`. | [optional][readonly]
**statement_description** | Option<**String**> | The statement description sent to the card networks.  Note: The actual statement description varies and is likely to be truncated and appended with additional information on a per issuer basis. | [optional][readonly]
**device_details** | Option<[**models::DeviceDetails**](DeviceDetails.md)> |  | [optional]
**card_payment_timeline** | Option<[**models::CardPaymentTimeline**](CardPaymentTimeline.md)> |  | [optional]
**refund_requires_card_presence** | Option<**bool**> | Whether the card must be physically present for the payment to be refunded.  If set to `true`, the card must be present. | [optional][readonly]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Information about errors encountered during the request. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


