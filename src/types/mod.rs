mod web_app_info;
pub use web_app_info::WebAppInfo;

mod chat_member;
pub use chat_member::ChatMember;

mod chat;
pub use chat::Chat;

mod message_origin_hidden_user;
pub use message_origin_hidden_user::MessageOriginHiddenUser;

mod chat_member_restricted;
pub use chat_member_restricted::ChatMemberRestricted;

mod inline_query_result_gif;
pub use inline_query_result_gif::InlineQueryResultGif;

mod inline_query_result_audio;
pub use inline_query_result_audio::InlineQueryResultAudio;

mod input_media_photo;
pub use input_media_photo::InputMediaPhoto;

mod message_origin_chat;
pub use message_origin_chat::MessageOriginChat;

mod inline_query_result_photo;
pub use inline_query_result_photo::InlineQueryResultPhoto;

mod star_transactions;
pub use star_transactions::StarTransactions;

mod passport_data;
pub use passport_data::PassportData;

mod giveaway;
pub use giveaway::Giveaway;

mod video;
pub use video::Video;

mod inline_query_result_cached_audio;
pub use inline_query_result_cached_audio::InlineQueryResultCachedAudio;

mod general_forum_topic_unhidden;
pub use general_forum_topic_unhidden::GeneralForumTopicUnhidden;

mod passport_element_error;
pub use passport_element_error::PassportElementError;

mod passport_element_error_files;
pub use passport_element_error_files::PassportElementErrorFiles;

mod background_type;
pub use background_type::BackgroundType;

mod inline_query;
pub use inline_query::InlineQuery;

mod giveaway_created;
pub use giveaway_created::GiveawayCreated;

mod chat_member_banned;
pub use chat_member_banned::ChatMemberBanned;

mod sent_web_app_message;
pub use sent_web_app_message::SentWebAppMessage;

mod keyboard_button_request_users;
pub use keyboard_button_request_users::KeyboardButtonRequestUsers;

mod switch_inline_query_chosen_chat;
pub use switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat;

mod general_forum_topic_hidden;
pub use general_forum_topic_hidden::GeneralForumTopicHidden;

mod background_type_pattern;
pub use background_type_pattern::BackgroundTypePattern;

mod menu_button;
pub use menu_button::MenuButton;

mod inline_query_result_cached_voice;
pub use inline_query_result_cached_voice::InlineQueryResultCachedVoice;

mod labeled_price;
pub use labeled_price::LabeledPrice;

mod inline_query_result_game;
pub use inline_query_result_game::InlineQueryResultGame;

mod chat_boost_source;
pub use chat_boost_source::ChatBoostSource;

mod input_text_message_content;
pub use input_text_message_content::InputTextMessageContent;

mod message_origin;
pub use message_origin::MessageOrigin;

mod business_opening_hours_interval;
pub use business_opening_hours_interval::BusinessOpeningHoursInterval;

mod animation;
pub use animation::Animation;

mod poll_option;
pub use poll_option::PollOption;

mod message_origin_user;
pub use message_origin_user::MessageOriginUser;

mod video_chat_started;
pub use video_chat_started::VideoChatStarted;

mod chat_join_request;
pub use chat_join_request::ChatJoinRequest;

mod chat_boost_source_giveaway;
pub use chat_boost_source_giveaway::ChatBoostSourceGiveaway;

mod shipping_option;
pub use shipping_option::ShippingOption;

mod giveaway_winners;
pub use giveaway_winners::GiveawayWinners;

mod message_id;
pub use message_id::MessageId;

mod story;
pub use story::Story;

mod reaction_count;
pub use reaction_count::ReactionCount;

mod transaction_partner_user;
pub use transaction_partner_user::TransactionPartnerUser;

mod passport_file;
pub use passport_file::PassportFile;

mod business_connection;
pub use business_connection::BusinessConnection;

mod input_media_video;
pub use input_media_video::InputMediaVideo;

mod inline_query_result_contact;
pub use inline_query_result_contact::InlineQueryResultContact;

mod shipping_address;
pub use shipping_address::ShippingAddress;

mod input_poll_option;
pub use input_poll_option::InputPollOption;

mod background_type_fill;
pub use background_type_fill::BackgroundTypeFill;

mod encrypted_passport_element;
pub use encrypted_passport_element::EncryptedPassportElement;

mod revenue_withdrawal_state_failed;
pub use revenue_withdrawal_state_failed::RevenueWithdrawalStateFailed;

mod keyboard_button_request_chat;
pub use keyboard_button_request_chat::KeyboardButtonRequestChat;

mod sticker;
pub use sticker::Sticker;

mod transaction_partner_other;
pub use transaction_partner_other::TransactionPartnerOther;

mod webhook_info;
pub use webhook_info::WebhookInfo;

mod chat_boost_source_gift_code;
pub use chat_boost_source_gift_code::ChatBoostSourceGiftCode;

mod inline_query_result_cached_mpeg_4_gif;
pub use inline_query_result_cached_mpeg_4_gif::InlineQueryResultCachedMpeg4Gif;

mod reply_parameters;
pub use reply_parameters::ReplyParameters;

mod keyboard_button;
pub use keyboard_button::KeyboardButton;

mod login_url;
pub use login_url::LoginUrl;

mod chat_full_info;
pub use chat_full_info::ChatFullInfo;

mod message_reaction_updated;
pub use message_reaction_updated::MessageReactionUpdated;

mod document;
pub use document::Document;

mod contact;
pub use contact::Contact;

mod background_type_wallpaper;
pub use background_type_wallpaper::BackgroundTypeWallpaper;

mod input_media_animation;
pub use input_media_animation::InputMediaAnimation;

mod sticker_set;
pub use sticker_set::StickerSet;

mod input_invoice_message_content;
pub use input_invoice_message_content::InputInvoiceMessageContent;

mod message_reaction_count_updated;
pub use message_reaction_count_updated::MessageReactionCountUpdated;

mod dice;
pub use dice::Dice;

mod transaction_partner_fragment;
pub use transaction_partner_fragment::TransactionPartnerFragment;

mod passport_element_error_selfie;
pub use passport_element_error_selfie::PassportElementErrorSelfie;

mod chat_background;
pub use chat_background::ChatBackground;

mod inline_query_result_article;
pub use inline_query_result_article::InlineQueryResultArticle;

mod input_file;
pub use input_file::InputFile;

mod background_type_chat_theme;
pub use background_type_chat_theme::BackgroundTypeChatTheme;

mod revenue_withdrawal_state;
pub use revenue_withdrawal_state::RevenueWithdrawalState;

mod write_access_allowed;
pub use write_access_allowed::WriteAccessAllowed;

mod encrypted_credentials;
pub use encrypted_credentials::EncryptedCredentials;

mod chat_administrator_rights;
pub use chat_administrator_rights::ChatAdministratorRights;

mod external_reply_info;
pub use external_reply_info::ExternalReplyInfo;

mod forum_topic_closed;
pub use forum_topic_closed::ForumTopicClosed;

mod input_media;
pub use input_media::InputMedia;

mod keyboard_button_poll_type;
pub use keyboard_button_poll_type::KeyboardButtonPollType;

mod chat_invite_link;
pub use chat_invite_link::ChatInviteLink;

mod pre_checkout_query;
pub use pre_checkout_query::PreCheckoutQuery;

mod voice;
pub use voice::Voice;

mod inline_query_result_document;
pub use inline_query_result_document::InlineQueryResultDocument;

mod chat_member_owner;
pub use chat_member_owner::ChatMemberOwner;

mod update;
pub use update::Update;

mod chat_permissions;
pub use chat_permissions::ChatPermissions;

mod chat_boost_removed;
pub use chat_boost_removed::ChatBoostRemoved;

mod passport_element_error_reverse_side;
pub use passport_element_error_reverse_side::PassportElementErrorReverseSide;

mod link_preview_options;
pub use link_preview_options::LinkPreviewOptions;

mod proximity_alert_triggered;
pub use proximity_alert_triggered::ProximityAlertTriggered;

mod chat_member_left;
pub use chat_member_left::ChatMemberLeft;

mod birthdate;
pub use birthdate::Birthdate;

mod user_chat_boosts;
pub use user_chat_boosts::UserChatBoosts;

mod file;
pub use file::File;

mod bot_command_scope;
pub use bot_command_scope::BotCommandScope;

mod chat_boost;
pub use chat_boost::ChatBoost;

mod bot_command_scope_all_group_chats;
pub use bot_command_scope_all_group_chats::BotCommandScopeAllGroupChats;

mod passport_element_error_data_field;
pub use passport_element_error_data_field::PassportElementErrorDataField;

mod forum_topic_created;
pub use forum_topic_created::ForumTopicCreated;

mod chat_shared;
pub use chat_shared::ChatShared;

mod menu_button_commands;
pub use menu_button_commands::MenuButtonCommands;

mod bot_name;
pub use bot_name::BotName;

mod inline_keyboard_button;
pub use inline_keyboard_button::InlineKeyboardButton;

mod chat_boost_source_premium;
pub use chat_boost_source_premium::ChatBoostSourcePremium;

mod forum_topic_reopened;
pub use forum_topic_reopened::ForumTopicReopened;

mod inline_keyboard_markup;
pub use inline_keyboard_markup::InlineKeyboardMarkup;

mod chat_boost_updated;
pub use chat_boost_updated::ChatBoostUpdated;

mod menu_button_default;
pub use menu_button_default::MenuButtonDefault;

mod reply_keyboard_remove;
pub use reply_keyboard_remove::ReplyKeyboardRemove;

mod inline_query_results_button;
pub use inline_query_results_button::InlineQueryResultsButton;

mod order_info;
pub use order_info::OrderInfo;

mod inline_query_result_mpeg_4_gif;
pub use inline_query_result_mpeg_4_gif::InlineQueryResultMpeg4Gif;

mod business_location;
pub use business_location::BusinessLocation;

mod shared_user;
pub use shared_user::SharedUser;

mod business_opening_hours;
pub use business_opening_hours::BusinessOpeningHours;

mod passport_element_error_translation_file;
pub use passport_element_error_translation_file::PassportElementErrorTranslationFile;

mod inline_query_result_cached_document;
pub use inline_query_result_cached_document::InlineQueryResultCachedDocument;

mod poll;
pub use poll::Poll;

mod web_app_data;
pub use web_app_data::WebAppData;

mod inline_query_result_cached_video;
pub use inline_query_result_cached_video::InlineQueryResultCachedVideo;

mod location;
pub use location::Location;

mod text_quote;
pub use text_quote::TextQuote;

mod user;
pub use user::User;

mod user_profile_photos;
pub use user_profile_photos::UserProfilePhotos;

mod chat_photo;
pub use chat_photo::ChatPhoto;

mod chat_member_administrator;
pub use chat_member_administrator::ChatMemberAdministrator;

mod reaction_type;
pub use reaction_type::ReactionType;

mod mask_position;
pub use mask_position::MaskPosition;

mod inline_query_result_video;
pub use inline_query_result_video::InlineQueryResultVideo;

mod inline_query_result_location;
pub use inline_query_result_location::InlineQueryResultLocation;

mod inline_query_result_cached_gif;
pub use inline_query_result_cached_gif::InlineQueryResultCachedGif;

mod response_parameters;
pub use response_parameters::ResponseParameters;

mod invoice;
pub use invoice::Invoice;

mod shipping_query;
pub use shipping_query::ShippingQuery;

mod revenue_withdrawal_state_succeeded;
pub use revenue_withdrawal_state_succeeded::RevenueWithdrawalStateSucceeded;

mod passport_element_error_front_side;
pub use passport_element_error_front_side::PassportElementErrorFrontSide;

mod passport_element_error_file;
pub use passport_element_error_file::PassportElementErrorFile;

mod callback_query;
pub use callback_query::CallbackQuery;

mod game_high_score;
pub use game_high_score::GameHighScore;

mod inline_query_result_cached_photo;
pub use inline_query_result_cached_photo::InlineQueryResultCachedPhoto;

mod force_reply;
pub use force_reply::ForceReply;

mod video_note;
pub use video_note::VideoNote;

mod chat_boost_added;
pub use chat_boost_added::ChatBoostAdded;

mod forum_topic;
pub use forum_topic::ForumTopic;

mod input_location_message_content;
pub use input_location_message_content::InputLocationMessageContent;

mod poll_answer;
pub use poll_answer::PollAnswer;

mod business_messages_deleted;
pub use business_messages_deleted::BusinessMessagesDeleted;

mod maybe_inaccessible_message;
pub use maybe_inaccessible_message::MaybeInaccessibleMessage;

mod users_shared;
pub use users_shared::UsersShared;

mod video_chat_scheduled;
pub use video_chat_scheduled::VideoChatScheduled;

mod input_media_audio;
pub use input_media_audio::InputMediaAudio;

mod inline_query_result;
pub use inline_query_result::InlineQueryResult;

mod input_venue_message_content;
pub use input_venue_message_content::InputVenueMessageContent;

mod input_message_content;
pub use input_message_content::InputMessageContent;

mod callback_game;
pub use callback_game::CallbackGame;

mod inaccessible_message;
pub use inaccessible_message::InaccessibleMessage;

mod bot_command_scope_all_private_chats;
pub use bot_command_scope_all_private_chats::BotCommandScopeAllPrivateChats;

mod audio;
pub use audio::Audio;

mod reaction_type_emoji;
pub use reaction_type_emoji::ReactionTypeEmoji;

mod bot_description;
pub use bot_description::BotDescription;

mod message;
pub use message::Message;

mod chosen_inline_result;
pub use chosen_inline_result::ChosenInlineResult;

mod passport_element_error_unspecified;
pub use passport_element_error_unspecified::PassportElementErrorUnspecified;

mod video_chat_ended;
pub use video_chat_ended::VideoChatEnded;

mod business_intro;
pub use business_intro::BusinessIntro;

mod background_fill;
pub use background_fill::BackgroundFill;

mod bot_command_scope_chat_member;
pub use bot_command_scope_chat_member::BotCommandScopeChatMember;

mod bot_command_scope_default;
pub use bot_command_scope_default::BotCommandScopeDefault;

mod passport_element_error_translation_files;
pub use passport_element_error_translation_files::PassportElementErrorTranslationFiles;

mod reply_keyboard_markup;
pub use reply_keyboard_markup::ReplyKeyboardMarkup;

mod input_sticker;
pub use input_sticker::InputSticker;

mod star_transaction;
pub use star_transaction::StarTransaction;

mod background_fill_freeform_gradient;
pub use background_fill_freeform_gradient::BackgroundFillFreeformGradient;

mod background_fill_gradient;
pub use background_fill_gradient::BackgroundFillGradient;

mod input_media_document;
pub use input_media_document::InputMediaDocument;

mod inline_query_result_voice;
pub use inline_query_result_voice::InlineQueryResultVoice;

mod message_origin_channel;
pub use message_origin_channel::MessageOriginChannel;

mod inline_query_result_cached_sticker;
pub use inline_query_result_cached_sticker::InlineQueryResultCachedSticker;

mod successful_payment;
pub use successful_payment::SuccessfulPayment;

mod chat_location;
pub use chat_location::ChatLocation;

mod forum_topic_edited;
pub use forum_topic_edited::ForumTopicEdited;

mod photo_size;
pub use photo_size::PhotoSize;

mod video_chat_participants_invited;
pub use video_chat_participants_invited::VideoChatParticipantsInvited;

mod bot_command_scope_all_chat_administrators;
pub use bot_command_scope_all_chat_administrators::BotCommandScopeAllChatAdministrators;

mod revenue_withdrawal_state_pending;
pub use revenue_withdrawal_state_pending::RevenueWithdrawalStatePending;

mod message_auto_delete_timer_changed;
pub use message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;

mod chat_member_member;
pub use chat_member_member::ChatMemberMember;

mod reaction_type_custom_emoji;
pub use reaction_type_custom_emoji::ReactionTypeCustomEmoji;

mod venue;
pub use venue::Venue;

mod bot_command_scope_chat;
pub use bot_command_scope_chat::BotCommandScopeChat;

mod menu_button_web_app;
pub use menu_button_web_app::MenuButtonWebApp;

mod message_entity;
pub use message_entity::MessageEntity;

mod input_contact_message_content;
pub use input_contact_message_content::InputContactMessageContent;

mod bot_short_description;
pub use bot_short_description::BotShortDescription;

mod transaction_partner;
pub use transaction_partner::TransactionPartner;

mod chat_member_updated;
pub use chat_member_updated::ChatMemberUpdated;

mod bot_command;
pub use bot_command::BotCommand;

mod inline_query_result_venue;
pub use inline_query_result_venue::InlineQueryResultVenue;

mod game;
pub use game::Game;

mod background_fill_solid;
pub use background_fill_solid::BackgroundFillSolid;

mod giveaway_completed;
pub use giveaway_completed::GiveawayCompleted;

mod bot_command_scope_chat_administrators;
pub use bot_command_scope_chat_administrators::BotCommandScopeChatAdministrators;
