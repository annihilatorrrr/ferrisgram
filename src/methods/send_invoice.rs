// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::Message;
use crate::types::{InlineKeyboardMarkup, LabeledPrice, ReplyParameters};
use crate::Bot;

impl Bot {
    /// Use this method to send invoices. On success, the sent Message is returned.
    /// <https://core.telegram.org/bots/api#sendinvoice>
    pub fn send_invoice(
        &self,
        chat_id: i64,
        title: String,
        description: String,
        payload: String,
        currency: String,
        prices: Vec<LabeledPrice>,
    ) -> SendInvoiceBuilder {
        SendInvoiceBuilder::new(self, chat_id, title, description, payload, currency, prices)
    }
}

#[derive(Serialize)]
pub struct SendInvoiceBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    pub payload: String,
    /// Payment provider token, obtained via @BotFather. Pass an empty string for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    /// Three-letter ISO 4217 currency code, see more on currencies. Pass "XTR" for payments in Telegram Stars.
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in Telegram Stars.
    pub prices: Vec<LabeledPrice>,
    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed max_tip_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// Unique deep-linking parameter. If left empty, forwarded copies of the sent message will have a Pay button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a URL button with a deep link to the bot (instead of a Pay button), with the value used as the start parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Photo size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    /// Pass True if you require the user's full name to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// Pass True if you require the user's phone number to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// Pass True if you require the user's email address to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// Pass True if you require the user's shipping address to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// Pass True if the user's phone number should be sent to the provider. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass True if the user's email address should be sent to the provider. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// Pass True if the final price depends on the shipping method. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Pay total price' button will be shown. If not empty, the first button must be a Pay button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a> SendInvoiceBuilder<'a> {
    pub fn new(
        bot: &'a Bot,
        chat_id: i64,
        title: String,
        description: String,
        payload: String,
        currency: String,
        prices: Vec<LabeledPrice>,
    ) -> Self {
        Self {
            bot,
            chat_id,
            message_thread_id: None,
            title,
            description,
            payload,
            provider_token: None,
            currency,
            prices,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            start_parameter: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
            disable_notification: None,
            protect_content: None,
            message_effect_id: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn payload(mut self, payload: String) -> Self {
        self.payload = payload;
        self
    }

    pub fn provider_token(mut self, provider_token: String) -> Self {
        self.provider_token = Some(provider_token);
        self
    }

    pub fn currency(mut self, currency: String) -> Self {
        self.currency = currency;
        self
    }

    pub fn prices(mut self, prices: Vec<LabeledPrice>) -> Self {
        self.prices = prices;
        self
    }

    pub fn max_tip_amount(mut self, max_tip_amount: i64) -> Self {
        self.max_tip_amount = Some(max_tip_amount);
        self
    }

    pub fn suggested_tip_amounts(mut self, suggested_tip_amounts: Vec<i64>) -> Self {
        self.suggested_tip_amounts = Some(suggested_tip_amounts);
        self
    }

    pub fn start_parameter(mut self, start_parameter: String) -> Self {
        self.start_parameter = Some(start_parameter);
        self
    }

    pub fn provider_data(mut self, provider_data: String) -> Self {
        self.provider_data = Some(provider_data);
        self
    }

    pub fn photo_url(mut self, photo_url: String) -> Self {
        self.photo_url = Some(photo_url);
        self
    }

    pub fn photo_size(mut self, photo_size: i64) -> Self {
        self.photo_size = Some(photo_size);
        self
    }

    pub fn photo_width(mut self, photo_width: i64) -> Self {
        self.photo_width = Some(photo_width);
        self
    }

    pub fn photo_height(mut self, photo_height: i64) -> Self {
        self.photo_height = Some(photo_height);
        self
    }

    pub fn need_name(mut self, need_name: bool) -> Self {
        self.need_name = Some(need_name);
        self
    }

    pub fn need_phone_number(mut self, need_phone_number: bool) -> Self {
        self.need_phone_number = Some(need_phone_number);
        self
    }

    pub fn need_email(mut self, need_email: bool) -> Self {
        self.need_email = Some(need_email);
        self
    }

    pub fn need_shipping_address(mut self, need_shipping_address: bool) -> Self {
        self.need_shipping_address = Some(need_shipping_address);
        self
    }

    pub fn send_phone_number_to_provider(mut self, send_phone_number_to_provider: bool) -> Self {
        self.send_phone_number_to_provider = Some(send_phone_number_to_provider);
        self
    }

    pub fn send_email_to_provider(mut self, send_email_to_provider: bool) -> Self {
        self.send_email_to_provider = Some(send_email_to_provider);
        self
    }

    pub fn is_flexible(mut self, is_flexible: bool) -> Self {
        self.is_flexible = Some(is_flexible);
        self
    }

    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    pub fn message_effect_id(mut self, message_effect_id: String) -> Self {
        self.message_effect_id = Some(message_effect_id);
        self
    }

    pub fn reply_parameters(mut self, reply_parameters: ReplyParameters) -> Self {
        self.reply_parameters = Some(reply_parameters);
        self
    }

    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub async fn send(self) -> Result<Message> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("sendInvoice", Some(&form)).await
    }
}
