# InvoiceAttachment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The Square-assigned ID of the attachment. | [optional][readonly]
**filename** | Option<**String**> | The file name of the attachment, which is displayed on the invoice. | [optional][readonly]
**description** | Option<**String**> | The description of the attachment, which is displayed on the invoice. This field maps to the seller-defined **Message** field. | [optional][readonly]
**filesize** | Option<**i32**> | The file size of the attachment in bytes. | [optional][readonly]
**hash** | Option<**String**> | The MD5 hash that was generated from the file contents. | [optional][readonly]
**mime_type** | Option<**String**> | The mime type of the attachment. The following mime types are supported:  image/gif, image/jpeg, image/png, image/tiff, image/bmp, application/pdf. | [optional][readonly]
**uploaded_at** | Option<**String**> | The timestamp when the attachment was uploaded, in RFC 3339 format. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


