# CatalogItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The item's name. This is a searchable attribute for use in applicable query filters, its value must not be empty, and the length is of Unicode code points. | [optional]
**description** | Option<**String**> | The item's description. This is a searchable attribute for use in applicable query filters, and its value length is of Unicode code points.  Deprecated at 2022-07-20, this field is planned to retire in 6 months. You should migrate to use `description_html` to set the description of the [CatalogItem](entity:CatalogItem) instance.  The `description` and `description_html` field values are kept in sync. If you try to set the both fields, the `description_html` text value overwrites the `description` value. Updates in one field are also reflected in the other, except for when you use an early version before Square API 2022-07-20 and `description_html` is set to blank, setting the `description` value to null does not nullify `description_html`. | [optional]
**abbreviation** | Option<**String**> | The text of the item's display label in the Square Point of Sale app. Only up to the first five characters of the string are used. This attribute is searchable, and its value length is of Unicode code points. | [optional]
**label_color** | Option<**String**> | The color of the item's display label in the Square Point of Sale app. This must be a valid hex color code. | [optional]
**is_taxable** | Option<**bool**> | Indicates whether the item is taxable (`true`) or non-taxable (`false`). Default is `true`. | [optional]
**category_id** | Option<**String**> | The ID of the item's category, if any. Deprecated since 2023-12-13. Use `CatalogItem.categories`, instead. | [optional]
**tax_ids** | Option<**Vec<String>**> | A set of IDs indicating the taxes enabled for this item. When updating an item, any taxes listed here will be added to the item. Taxes may also be added to or deleted from an item using `UpdateItemTaxes`. | [optional]
**modifier_list_info** | Option<[**Vec<models::CatalogItemModifierListInfo>**](CatalogItemModifierListInfo.md)> | A set of `CatalogItemModifierListInfo` objects representing the modifier lists that apply to this item, along with the overrides and min and max limits that are specific to this item. Modifier lists may also be added to or deleted from an item using `UpdateItemModifierLists`. | [optional]
**variations** | Option<[**Vec<models::CatalogObject>**](CatalogObject.md)> | A list of [CatalogItemVariation](entity:CatalogItemVariation) objects for this item. An item must have at least one variation. | [optional]
**product_type** | Option<[**models::CatalogItemProductType**](CatalogItemProductType.md)> |  | [optional]
**skip_modifier_screen** | Option<**bool**> | If `false`, the Square Point of Sale app will present the `CatalogItem`'s details screen immediately, allowing the merchant to choose `CatalogModifier`s before adding the item to the cart.  This is the default behavior.  If `true`, the Square Point of Sale app will immediately add the item to the cart with the pre-selected modifiers, and merchants can edit modifiers by drilling down onto the item's details.  Third-party clients are encouraged to implement similar behaviors. | [optional]
**item_options** | Option<[**Vec<models::CatalogItemOptionForItem>**](CatalogItemOptionForItem.md)> | List of item options IDs for this item. Used to manage and group item variations in a specified order.  Maximum: 6 item options. | [optional]
**ecom_uri** | Option<**String**> | Deprecated. A URI pointing to a published e-commerce product page for the Item. | [optional]
**ecom_image_uris** | Option<**Vec<String>**> | Deprecated. A comma-separated list of encoded URIs pointing to a set of published e-commerce images for the Item. | [optional]
**image_ids** | Option<**Vec<String>**> | The IDs of images associated with this `CatalogItem` instance. These images will be shown to customers in Square Online Store. The first image will show up as the icon for this item in POS. | [optional]
**sort_name** | Option<**String**> | A name to sort the item by. If this name is unspecified, namely, the `sort_name` field is absent, the regular `name` field is used for sorting. Its value must not be empty.  It is currently supported for sellers of the Japanese locale only. | [optional]
**categories** | Option<[**Vec<models::CatalogObjectCategory>**](CatalogObjectCategory.md)> | The list of categories. | [optional]
**description_html** | Option<**String**> | The item's description as expressed in valid HTML elements. The length of this field value, including those of HTML tags, is of Unicode points. With application query filters, the text values of the HTML elements and attributes are searchable. Invalid or unsupported HTML elements or attributes are ignored.  Supported HTML elements include: - `a`: Link. Supports linking to website URLs, email address, and telephone numbers. - `b`, `strong`:  Bold text - `br`: Line break - `code`: Computer code - `div`: Section - `h1-h6`: Headings - `i`, `em`: Italics - `li`: List element - `ol`: Numbered list - `p`: Paragraph - `ul`: Bullet list - `u`: Underline   Supported HTML attributes include: - `align`: Alignment of the text content - `href`: Link destination - `rel`: Relationship between link's target and source - `target`: Place to open the linked document | [optional]
**description_plaintext** | Option<**String**> | A server-generated plaintext version of the `description_html` field, without formatting tags. | [optional][readonly]
**channels** | Option<**Vec<String>**> | A list of IDs representing channels, such as a Square Online site, where the item can be made visible or available. This field is read only and cannot be edited. | [optional]
**is_archived** | Option<**bool**> | Indicates whether this item is archived (`true`) or not (`false`). | [optional]
**ecom_seo_data** | Option<[**models::CatalogEcomSeoData**](CatalogEcomSeoData.md)> |  | [optional]
**food_and_beverage_details** | Option<[**models::CatalogItemFoodAndBeverageDetails**](CatalogItemFoodAndBeverageDetails.md)> |  | [optional]
**reporting_category** | Option<[**models::CatalogObjectCategory**](CatalogObjectCategory.md)> |  | [optional]
**is_alcoholic** | Option<**bool**> | Indicates whether this item is alcoholic (`true`) or not (`false`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


