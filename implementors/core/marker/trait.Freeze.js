(function() {var implementors = {};
implementors["rs_cord"] = [{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/struct.ClientState.html\" title=\"struct rs_cord::ClientState\">ClientState</a>","synthetic":true,"types":["rs_cord::client::state::ClientState"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/struct.Client.html\" title=\"struct rs_cord::Client\">Client</a>","synthetic":true,"types":["rs_cord::client::Client"]},{"text":"impl Freeze for <a class=\"enum\" href=\"rs_cord/error/enum.Error.html\" title=\"enum rs_cord::error::Error\">Error</a>","synthetic":true,"types":["rs_cord::error::Error"]},{"text":"impl Freeze for <a class=\"enum\" href=\"rs_cord/http/enum.RequestMethod.html\" title=\"enum rs_cord::http::RequestMethod\">RequestMethod</a>","synthetic":true,"types":["rs_cord::http::RequestMethod"]},{"text":"impl&lt;'a&gt; Freeze for <a class=\"struct\" href=\"rs_cord/http/struct.Route.html\" title=\"struct rs_cord::http::Route\">Route</a>&lt;'a&gt;","synthetic":true,"types":["rs_cord::http::Route"]},{"text":"impl&lt;'c, 'r&gt; Freeze for <a class=\"struct\" href=\"rs_cord/http/struct.HttpClientRequestBuilder.html\" title=\"struct rs_cord::http::HttpClientRequestBuilder\">HttpClientRequestBuilder</a>&lt;'c, 'r&gt;","synthetic":true,"types":["rs_cord::http::HttpClientRequestBuilder"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/http/struct.HttpClient.html\" title=\"struct rs_cord::http::HttpClient\">HttpClient</a>","synthetic":true,"types":["rs_cord::http::HttpClient"]},{"text":"impl Freeze for <a class=\"enum\" href=\"rs_cord/manager/enum.NotFoundError.html\" title=\"enum rs_cord::manager::NotFoundError\">NotFoundError</a>","synthetic":true,"types":["rs_cord::manager::error::NotFoundError"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/manager/struct.UserManager.html\" title=\"struct rs_cord::manager::UserManager\">UserManager</a>","synthetic":true,"types":["rs_cord::manager::users::UserManager"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/models/asset/struct.Asset.html\" title=\"struct rs_cord::models::asset::Asset\">Asset</a>","synthetic":true,"types":["rs_cord::models::asset::Asset"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/models/color/struct.Color.html\" title=\"struct rs_cord::models::color::Color\">Color</a>","synthetic":true,"types":["rs_cord::models::color::Color"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/models/gateway/struct.Intents.html\" title=\"struct rs_cord::models::gateway::Intents\">Intents</a>","synthetic":true,"types":["rs_cord::models::gateway::Intents"]},{"text":"impl Freeze for <a class=\"enum\" href=\"rs_cord/models/gateway/enum.OpCode.html\" title=\"enum rs_cord::models::gateway::OpCode\">OpCode</a>","synthetic":true,"types":["rs_cord::models::gateway::OpCode"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/models/message/struct.Message.html\" title=\"struct rs_cord::models::message::Message\">Message</a>","synthetic":true,"types":["rs_cord::models::message::Message"]},{"text":"impl Freeze for <a class=\"enum\" href=\"rs_cord/models/timestamp/enum.RelativeTime.html\" title=\"enum rs_cord::models::timestamp::RelativeTime\">RelativeTime</a>","synthetic":true,"types":["rs_cord::models::timestamp::RelativeTime"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/models/timestamp/struct.Timestamp.html\" title=\"struct rs_cord::models::timestamp::Timestamp\">Timestamp</a>","synthetic":true,"types":["rs_cord::models::timestamp::Timestamp"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/models/user/struct.User.html\" title=\"struct rs_cord::models::user::User\">User</a>","synthetic":true,"types":["rs_cord::models::user::User"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/models/user/struct.UserFlags.html\" title=\"struct rs_cord::models::user::UserFlags\">UserFlags</a>","synthetic":true,"types":["rs_cord::models::user::UserFlags"]},{"text":"impl Freeze for <a class=\"enum\" href=\"rs_cord/models/user/enum.HypesquadHouse.html\" title=\"enum rs_cord::models::user::HypesquadHouse\">HypesquadHouse</a>","synthetic":true,"types":["rs_cord::models::user::HypesquadHouse"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/application/struct.ApplicationData.html\" title=\"struct rs_cord::types::application::ApplicationData\">ApplicationData</a>","synthetic":true,"types":["rs_cord::types::application::ApplicationData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/application/struct.PartialApplicationData.html\" title=\"struct rs_cord::types::application::PartialApplicationData\">PartialApplicationData</a>","synthetic":true,"types":["rs_cord::types::application::PartialApplicationData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/application/struct.TeamData.html\" title=\"struct rs_cord::types::application::TeamData\">TeamData</a>","synthetic":true,"types":["rs_cord::types::application::TeamData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/application/struct.TeamMemberData.html\" title=\"struct rs_cord::types::application::TeamMemberData\">TeamMemberData</a>","synthetic":true,"types":["rs_cord::types::application::TeamMemberData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/channel/struct.ChannelData.html\" title=\"struct rs_cord::types::channel::ChannelData\">ChannelData</a>","synthetic":true,"types":["rs_cord::types::channel::ChannelData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/channel/struct.OverwriteData.html\" title=\"struct rs_cord::types::channel::OverwriteData\">OverwriteData</a>","synthetic":true,"types":["rs_cord::types::channel::OverwriteData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/channel/struct.ThreadMetadata.html\" title=\"struct rs_cord::types::channel::ThreadMetadata\">ThreadMetadata</a>","synthetic":true,"types":["rs_cord::types::channel::ThreadMetadata"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/channel/struct.ThreadMemberData.html\" title=\"struct rs_cord::types::channel::ThreadMemberData\">ThreadMemberData</a>","synthetic":true,"types":["rs_cord::types::channel::ThreadMemberData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/emoji/struct.EmojiData.html\" title=\"struct rs_cord::types::emoji::EmojiData\">EmojiData</a>","synthetic":true,"types":["rs_cord::types::emoji::EmojiData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/emoji/struct.PartialEmojiData.html\" title=\"struct rs_cord::types::emoji::PartialEmojiData\">PartialEmojiData</a>","synthetic":true,"types":["rs_cord::types::emoji::PartialEmojiData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/emoji/struct.ActivityEmojiData.html\" title=\"struct rs_cord::types::emoji::ActivityEmojiData\">ActivityEmojiData</a>","synthetic":true,"types":["rs_cord::types::emoji::ActivityEmojiData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GetGatewayData.html\" title=\"struct rs_cord::types::gateway::GetGatewayData\">GetGatewayData</a>","synthetic":true,"types":["rs_cord::types::gateway::GetGatewayData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.SessionStartLimitData.html\" title=\"struct rs_cord::types::gateway::SessionStartLimitData\">SessionStartLimitData</a>","synthetic":true,"types":["rs_cord::types::gateway::SessionStartLimitData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GetGatewayBotData.html\" title=\"struct rs_cord::types::gateway::GetGatewayBotData\">GetGatewayBotData</a>","synthetic":true,"types":["rs_cord::types::gateway::GetGatewayBotData"]},{"text":"impl Freeze for <a class=\"enum\" href=\"rs_cord/types/gateway/enum.WsInboundEvent.html\" title=\"enum rs_cord::types::gateway::WsInboundEvent\">WsInboundEvent</a>","synthetic":true,"types":["rs_cord::types::gateway::WsInboundEvent"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.ReadyData.html\" title=\"struct rs_cord::types::gateway::ReadyData\">ReadyData</a>","synthetic":true,"types":["rs_cord::types::gateway::ReadyData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.ResumedData.html\" title=\"struct rs_cord::types::gateway::ResumedData\">ResumedData</a>","synthetic":true,"types":["rs_cord::types::gateway::ResumedData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.ChannelCreateData.html\" title=\"struct rs_cord::types::gateway::ChannelCreateData\">ChannelCreateData</a>","synthetic":true,"types":["rs_cord::types::gateway::ChannelCreateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.ChannelUpdateData.html\" title=\"struct rs_cord::types::gateway::ChannelUpdateData\">ChannelUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::ChannelUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.ChannelDeleteData.html\" title=\"struct rs_cord::types::gateway::ChannelDeleteData\">ChannelDeleteData</a>","synthetic":true,"types":["rs_cord::types::gateway::ChannelDeleteData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.ThreadCreateData.html\" title=\"struct rs_cord::types::gateway::ThreadCreateData\">ThreadCreateData</a>","synthetic":true,"types":["rs_cord::types::gateway::ThreadCreateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.ThreadUpdateData.html\" title=\"struct rs_cord::types::gateway::ThreadUpdateData\">ThreadUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::ThreadUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.ThreadDeleteData.html\" title=\"struct rs_cord::types::gateway::ThreadDeleteData\">ThreadDeleteData</a>","synthetic":true,"types":["rs_cord::types::gateway::ThreadDeleteData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.ThreadListSyncData.html\" title=\"struct rs_cord::types::gateway::ThreadListSyncData\">ThreadListSyncData</a>","synthetic":true,"types":["rs_cord::types::gateway::ThreadListSyncData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.ThreadMemberUpdateData.html\" title=\"struct rs_cord::types::gateway::ThreadMemberUpdateData\">ThreadMemberUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::ThreadMemberUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.ThreadMembersUpdateData.html\" title=\"struct rs_cord::types::gateway::ThreadMembersUpdateData\">ThreadMembersUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::ThreadMembersUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.ChannelPinsUpdateData.html\" title=\"struct rs_cord::types::gateway::ChannelPinsUpdateData\">ChannelPinsUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::ChannelPinsUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildCreateData.html\" title=\"struct rs_cord::types::gateway::GuildCreateData\">GuildCreateData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildCreateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildUpdateData.html\" title=\"struct rs_cord::types::gateway::GuildUpdateData\">GuildUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildDeleteData.html\" title=\"struct rs_cord::types::gateway::GuildDeleteData\">GuildDeleteData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildDeleteData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildBanAddData.html\" title=\"struct rs_cord::types::gateway::GuildBanAddData\">GuildBanAddData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildBanAddData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildBanRemoveData.html\" title=\"struct rs_cord::types::gateway::GuildBanRemoveData\">GuildBanRemoveData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildBanRemoveData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildEmojisUpdateData.html\" title=\"struct rs_cord::types::gateway::GuildEmojisUpdateData\">GuildEmojisUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildEmojisUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildStickersUpdateData.html\" title=\"struct rs_cord::types::gateway::GuildStickersUpdateData\">GuildStickersUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildStickersUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildIntegrationsUpdateData.html\" title=\"struct rs_cord::types::gateway::GuildIntegrationsUpdateData\">GuildIntegrationsUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildIntegrationsUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildMemberAddData.html\" title=\"struct rs_cord::types::gateway::GuildMemberAddData\">GuildMemberAddData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildMemberAddData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildMemberRemoveData.html\" title=\"struct rs_cord::types::gateway::GuildMemberRemoveData\">GuildMemberRemoveData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildMemberRemoveData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildMemberUpdateData.html\" title=\"struct rs_cord::types::gateway::GuildMemberUpdateData\">GuildMemberUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildMemberUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildMembersChunkData.html\" title=\"struct rs_cord::types::gateway::GuildMembersChunkData\">GuildMembersChunkData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildMembersChunkData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildRoleCreateData.html\" title=\"struct rs_cord::types::gateway::GuildRoleCreateData\">GuildRoleCreateData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildRoleCreateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildRoleUpdateData.html\" title=\"struct rs_cord::types::gateway::GuildRoleUpdateData\">GuildRoleUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildRoleUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildRoleDeleteData.html\" title=\"struct rs_cord::types::gateway::GuildRoleDeleteData\">GuildRoleDeleteData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildRoleDeleteData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildScheduledEventCreateData.html\" title=\"struct rs_cord::types::gateway::GuildScheduledEventCreateData\">GuildScheduledEventCreateData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildScheduledEventCreateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildScheduledEventUpdateData.html\" title=\"struct rs_cord::types::gateway::GuildScheduledEventUpdateData\">GuildScheduledEventUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildScheduledEventUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildScheduledEventDeleteData.html\" title=\"struct rs_cord::types::gateway::GuildScheduledEventDeleteData\">GuildScheduledEventDeleteData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildScheduledEventDeleteData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildScheduledEventUserAddData.html\" title=\"struct rs_cord::types::gateway::GuildScheduledEventUserAddData\">GuildScheduledEventUserAddData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildScheduledEventUserAddData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.GuildScheduledEventUserRemoveData.html\" title=\"struct rs_cord::types::gateway::GuildScheduledEventUserRemoveData\">GuildScheduledEventUserRemoveData</a>","synthetic":true,"types":["rs_cord::types::gateway::GuildScheduledEventUserRemoveData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.IntegrationCreateData.html\" title=\"struct rs_cord::types::gateway::IntegrationCreateData\">IntegrationCreateData</a>","synthetic":true,"types":["rs_cord::types::gateway::IntegrationCreateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.IntegrationUpdateData.html\" title=\"struct rs_cord::types::gateway::IntegrationUpdateData\">IntegrationUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::IntegrationUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.IntegrationDeleteData.html\" title=\"struct rs_cord::types::gateway::IntegrationDeleteData\">IntegrationDeleteData</a>","synthetic":true,"types":["rs_cord::types::gateway::IntegrationDeleteData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.InviteCreateData.html\" title=\"struct rs_cord::types::gateway::InviteCreateData\">InviteCreateData</a>","synthetic":true,"types":["rs_cord::types::gateway::InviteCreateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.InviteDeleteData.html\" title=\"struct rs_cord::types::gateway::InviteDeleteData\">InviteDeleteData</a>","synthetic":true,"types":["rs_cord::types::gateway::InviteDeleteData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.MessageCreateData.html\" title=\"struct rs_cord::types::gateway::MessageCreateData\">MessageCreateData</a>","synthetic":true,"types":["rs_cord::types::gateway::MessageCreateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.MessageUpdateData.html\" title=\"struct rs_cord::types::gateway::MessageUpdateData\">MessageUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::MessageUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.MessageDeleteData.html\" title=\"struct rs_cord::types::gateway::MessageDeleteData\">MessageDeleteData</a>","synthetic":true,"types":["rs_cord::types::gateway::MessageDeleteData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.MessageDeleteBulkData.html\" title=\"struct rs_cord::types::gateway::MessageDeleteBulkData\">MessageDeleteBulkData</a>","synthetic":true,"types":["rs_cord::types::gateway::MessageDeleteBulkData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.MessageReactionAddData.html\" title=\"struct rs_cord::types::gateway::MessageReactionAddData\">MessageReactionAddData</a>","synthetic":true,"types":["rs_cord::types::gateway::MessageReactionAddData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.MessageReactionRemoveData.html\" title=\"struct rs_cord::types::gateway::MessageReactionRemoveData\">MessageReactionRemoveData</a>","synthetic":true,"types":["rs_cord::types::gateway::MessageReactionRemoveData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.MessageReactionRemoveAllData.html\" title=\"struct rs_cord::types::gateway::MessageReactionRemoveAllData\">MessageReactionRemoveAllData</a>","synthetic":true,"types":["rs_cord::types::gateway::MessageReactionRemoveAllData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.MessageReactionRemoveEmojiData.html\" title=\"struct rs_cord::types::gateway::MessageReactionRemoveEmojiData\">MessageReactionRemoveEmojiData</a>","synthetic":true,"types":["rs_cord::types::gateway::MessageReactionRemoveEmojiData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.TypingStartData.html\" title=\"struct rs_cord::types::gateway::TypingStartData\">TypingStartData</a>","synthetic":true,"types":["rs_cord::types::gateway::TypingStartData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.UserUpdateData.html\" title=\"struct rs_cord::types::gateway::UserUpdateData\">UserUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::UserUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.VoiceStateUpdateData.html\" title=\"struct rs_cord::types::gateway::VoiceStateUpdateData\">VoiceStateUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::VoiceStateUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.VoiceServerUpdateData.html\" title=\"struct rs_cord::types::gateway::VoiceServerUpdateData\">VoiceServerUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::VoiceServerUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/gateway/struct.WebhooksUpdateData.html\" title=\"struct rs_cord::types::gateway::WebhooksUpdateData\">WebhooksUpdateData</a>","synthetic":true,"types":["rs_cord::types::gateway::WebhooksUpdateData"]},{"text":"impl Freeze for <a class=\"enum\" href=\"rs_cord/types/gateway/enum.WsDispatchEvent.html\" title=\"enum rs_cord::types::gateway::WsDispatchEvent\">WsDispatchEvent</a>","synthetic":true,"types":["rs_cord::types::gateway::WsDispatchEvent"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/guild/struct.GuildData.html\" title=\"struct rs_cord::types::guild::GuildData\">GuildData</a>","synthetic":true,"types":["rs_cord::types::guild::GuildData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/guild/struct.UnavailableGuildData.html\" title=\"struct rs_cord::types::guild::UnavailableGuildData\">UnavailableGuildData</a>","synthetic":true,"types":["rs_cord::types::guild::UnavailableGuildData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/guild/struct.WelcomeScreenData.html\" title=\"struct rs_cord::types::guild::WelcomeScreenData\">WelcomeScreenData</a>","synthetic":true,"types":["rs_cord::types::guild::WelcomeScreenData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/guild/struct.WelcomeScreenChannelData.html\" title=\"struct rs_cord::types::guild::WelcomeScreenChannelData\">WelcomeScreenChannelData</a>","synthetic":true,"types":["rs_cord::types::guild::WelcomeScreenChannelData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/guild/struct.ScheduledEventData.html\" title=\"struct rs_cord::types::guild::ScheduledEventData\">ScheduledEventData</a>","synthetic":true,"types":["rs_cord::types::guild::ScheduledEventData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/guild/struct.ScheduledEventMetadata.html\" title=\"struct rs_cord::types::guild::ScheduledEventMetadata\">ScheduledEventMetadata</a>","synthetic":true,"types":["rs_cord::types::guild::ScheduledEventMetadata"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/guild/struct.IntegrationData.html\" title=\"struct rs_cord::types::guild::IntegrationData\">IntegrationData</a>","synthetic":true,"types":["rs_cord::types::guild::IntegrationData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/guild/struct.IntegrationAccountData.html\" title=\"struct rs_cord::types::guild::IntegrationAccountData\">IntegrationAccountData</a>","synthetic":true,"types":["rs_cord::types::guild::IntegrationAccountData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/invite/struct.InviteData.html\" title=\"struct rs_cord::types::invite::InviteData\">InviteData</a>","synthetic":true,"types":["rs_cord::types::invite::InviteData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/invite/struct.InviteStageInstanceData.html\" title=\"struct rs_cord::types::invite::InviteStageInstanceData\">InviteStageInstanceData</a>","synthetic":true,"types":["rs_cord::types::invite::InviteStageInstanceData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/member/struct.MemberData.html\" title=\"struct rs_cord::types::member::MemberData\">MemberData</a>","synthetic":true,"types":["rs_cord::types::member::MemberData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.MessageData.html\" title=\"struct rs_cord::types::message::MessageData\">MessageData</a>","synthetic":true,"types":["rs_cord::types::message::MessageData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.MessageUpdateData.html\" title=\"struct rs_cord::types::message::MessageUpdateData\">MessageUpdateData</a>","synthetic":true,"types":["rs_cord::types::message::MessageUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.ChannelMentionData.html\" title=\"struct rs_cord::types::message::ChannelMentionData\">ChannelMentionData</a>","synthetic":true,"types":["rs_cord::types::message::ChannelMentionData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.AttachmentData.html\" title=\"struct rs_cord::types::message::AttachmentData\">AttachmentData</a>","synthetic":true,"types":["rs_cord::types::message::AttachmentData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.EmbedData.html\" title=\"struct rs_cord::types::message::EmbedData\">EmbedData</a>","synthetic":true,"types":["rs_cord::types::message::EmbedData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.EmbedFooterData.html\" title=\"struct rs_cord::types::message::EmbedFooterData\">EmbedFooterData</a>","synthetic":true,"types":["rs_cord::types::message::EmbedFooterData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.EmbedImageData.html\" title=\"struct rs_cord::types::message::EmbedImageData\">EmbedImageData</a>","synthetic":true,"types":["rs_cord::types::message::EmbedImageData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.EmbedThumbnailData.html\" title=\"struct rs_cord::types::message::EmbedThumbnailData\">EmbedThumbnailData</a>","synthetic":true,"types":["rs_cord::types::message::EmbedThumbnailData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.EmbedVideoData.html\" title=\"struct rs_cord::types::message::EmbedVideoData\">EmbedVideoData</a>","synthetic":true,"types":["rs_cord::types::message::EmbedVideoData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.EmbedProviderData.html\" title=\"struct rs_cord::types::message::EmbedProviderData\">EmbedProviderData</a>","synthetic":true,"types":["rs_cord::types::message::EmbedProviderData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.EmbedAuthorData.html\" title=\"struct rs_cord::types::message::EmbedAuthorData\">EmbedAuthorData</a>","synthetic":true,"types":["rs_cord::types::message::EmbedAuthorData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.EmbedFieldData.html\" title=\"struct rs_cord::types::message::EmbedFieldData\">EmbedFieldData</a>","synthetic":true,"types":["rs_cord::types::message::EmbedFieldData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.ReactionData.html\" title=\"struct rs_cord::types::message::ReactionData\">ReactionData</a>","synthetic":true,"types":["rs_cord::types::message::ReactionData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.MessageReferenceData.html\" title=\"struct rs_cord::types::message::MessageReferenceData\">MessageReferenceData</a>","synthetic":true,"types":["rs_cord::types::message::MessageReferenceData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.MessageInteractionData.html\" title=\"struct rs_cord::types::message::MessageInteractionData\">MessageInteractionData</a>","synthetic":true,"types":["rs_cord::types::message::MessageInteractionData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.ComponentData.html\" title=\"struct rs_cord::types::message::ComponentData\">ComponentData</a>","synthetic":true,"types":["rs_cord::types::message::ComponentData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/message/struct.SelectOptionData.html\" title=\"struct rs_cord::types::message::SelectOptionData\">SelectOptionData</a>","synthetic":true,"types":["rs_cord::types::message::SelectOptionData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/presence/struct.PresenceUpdateData.html\" title=\"struct rs_cord::types::presence::PresenceUpdateData\">PresenceUpdateData</a>","synthetic":true,"types":["rs_cord::types::presence::PresenceUpdateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/presence/struct.ActivityData.html\" title=\"struct rs_cord::types::presence::ActivityData\">ActivityData</a>","synthetic":true,"types":["rs_cord::types::presence::ActivityData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/presence/struct.ActivityTimestampsData.html\" title=\"struct rs_cord::types::presence::ActivityTimestampsData\">ActivityTimestampsData</a>","synthetic":true,"types":["rs_cord::types::presence::ActivityTimestampsData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/presence/struct.ActivityPartyData.html\" title=\"struct rs_cord::types::presence::ActivityPartyData\">ActivityPartyData</a>","synthetic":true,"types":["rs_cord::types::presence::ActivityPartyData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/presence/struct.ActivityAssetsData.html\" title=\"struct rs_cord::types::presence::ActivityAssetsData\">ActivityAssetsData</a>","synthetic":true,"types":["rs_cord::types::presence::ActivityAssetsData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/presence/struct.ActivitySecretsData.html\" title=\"struct rs_cord::types::presence::ActivitySecretsData\">ActivitySecretsData</a>","synthetic":true,"types":["rs_cord::types::presence::ActivitySecretsData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/presence/struct.ActivityButtonData.html\" title=\"struct rs_cord::types::presence::ActivityButtonData\">ActivityButtonData</a>","synthetic":true,"types":["rs_cord::types::presence::ActivityButtonData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/presence/struct.ClientStatusData.html\" title=\"struct rs_cord::types::presence::ClientStatusData\">ClientStatusData</a>","synthetic":true,"types":["rs_cord::types::presence::ClientStatusData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/role/struct.RoleData.html\" title=\"struct rs_cord::types::role::RoleData\">RoleData</a>","synthetic":true,"types":["rs_cord::types::role::RoleData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/role/struct.RoleTagsData.html\" title=\"struct rs_cord::types::role::RoleTagsData\">RoleTagsData</a>","synthetic":true,"types":["rs_cord::types::role::RoleTagsData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/sticker/struct.StickerData.html\" title=\"struct rs_cord::types::sticker::StickerData\">StickerData</a>","synthetic":true,"types":["rs_cord::types::sticker::StickerData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/sticker/struct.StickerItemData.html\" title=\"struct rs_cord::types::sticker::StickerItemData\">StickerItemData</a>","synthetic":true,"types":["rs_cord::types::sticker::StickerItemData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/user/struct.UserData.html\" title=\"struct rs_cord::types::user::UserData\">UserData</a>","synthetic":true,"types":["rs_cord::types::user::UserData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/voice/struct.VoiceStateData.html\" title=\"struct rs_cord::types::voice::VoiceStateData\">VoiceStateData</a>","synthetic":true,"types":["rs_cord::types::voice::VoiceStateData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"rs_cord/types/voice/struct.StageInstanceData.html\" title=\"struct rs_cord::types::voice::StageInstanceData\">StageInstanceData</a>","synthetic":true,"types":["rs_cord::types::voice::StageInstanceData"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()