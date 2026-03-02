# CreateCatalogImageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | **String** | A unique string that identifies this CreateCatalogImage request. Keys can be any valid string but must be unique for every CreateCatalogImage request.  See [Idempotency keys](https://developer.squareup.com/docs/build-basics/common-api-patterns/idempotency) for more information. | 
**object_id** | Option<**String**> | Unique ID of the `CatalogObject` to attach this `CatalogImage` object to. Leave this field empty to create unattached images, for example if you are building an integration where an image can be attached to catalog items at a later time. | [optional]
**image** | [**models::CatalogObject**](CatalogObject.md) |  | 
**is_primary** | Option<**bool**> | If this is set to `true`, the image created will be the primary, or first image of the object referenced by `object_id`. If the `CatalogObject` already has a primary `CatalogImage`, setting this field to `true` will replace the primary image. If this is set to `false` and you use the Square API version 2021-12-15 or later, the image id will be appended to the list of `image_ids` on the object.  With Square API version 2021-12-15 or later, the default value is `false`. Otherwise, the effective default value is `true`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


