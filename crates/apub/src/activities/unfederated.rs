use crate::SendActivity;
use lemmy_api_common::{
  comment::{
    CommentReportResponse, CommentResponse, DistinguishComment, GetComment, ListCommentReports,
    ListCommentReportsResponse, ResolveCommentReport, SaveComment,
  },
  community::{
    CommunityResponse, CreateCommunity, GetCommunityResponse, ListCommunities,
    ListCommunitiesResponse, TransferCommunity,
  },
  custom_emoji::{
    CreateCustomEmoji, CustomEmojiResponse, DeleteCustomEmoji, DeleteCustomEmojiResponse,
    EditCustomEmoji,
  },
  person::{
    AddAdmin, AddAdminResponse, BannedPersonsResponse, BlockPerson, BlockPersonResponse,
    ChangePassword, CommentReplyResponse, GetBannedPersons, GetCaptcha, GetCaptchaResponse,
    GetPersonMentions, GetPersonMentionsResponse, GetReplies, GetRepliesResponse, GetReportCount,
    GetReportCountResponse, GetUnreadCount, GetUnreadCountResponse, Login, LoginResponse,
    MarkAllAsRead, MarkCommentReplyAsRead, MarkPersonMentionAsRead, PasswordChangeAfterReset,
    PasswordReset, PasswordResetResponse, PersonMentionResponse, Register, SaveUserSettings,
    VerifyEmail, VerifyEmailResponse,
  },
  pipayment::{
    CreatePayment, CreatePaymentResponse, GetPayment, GetPaymentResponse, GetPayments,
    GetPaymentsResponse, GetPiBalances, GetPiBalancesResponse, PiAgreeRegister, PiAgreeResponse,
    PiApprove, PiApproveResponse, PiKey, PiKeyResponse, PiLogin, PiPaymentComplete,
    PiPaymentCompleteResponse, PiPaymentFound, PiPaymentFoundResponse, PiRegister,
    PiRegisterWithFee, PiWithdraw, PiWithdrawResponse, SendPayment, SendPaymentResponse,
  },
  post::{
    GetPost, GetPostResponse, GetSiteMetadata, GetSiteMetadataResponse, ListPostReports,
    ListPostReportsResponse, MarkPostAsRead, PostReportResponse, PostResponse, ResolvePostReport,
    SavePost,
  },
  private_message::{
    CreatePrivateMessageReport, GetPrivateMessages, ListPrivateMessageReports,
    ListPrivateMessageReportsResponse, MarkPrivateMessageAsRead, PrivateMessageReportResponse,
    PrivateMessageResponse, PrivateMessagesResponse, ResolvePrivateMessageReport,
  },
  site::{
    ApproveRegistrationApplication, CreateSite, EditSite, GetFederatedInstances,
    GetFederatedInstancesResponse, GetModlog, GetModlogResponse, GetSite, GetSiteResponse,
    GetUnreadRegistrationApplicationCount, GetUnreadRegistrationApplicationCountResponse,
    LeaveAdmin, ListRegistrationApplications, ListRegistrationApplicationsResponse, PurgeComment,
    PurgeCommunity, PurgeItemResponse, PurgePerson, PurgePost, RegistrationApplicationResponse,
    SiteResponse,
  },
  web3::{Web3Login, Web3Register},
};

impl SendActivity for Register {
  type Response = LoginResponse;
}

impl SendActivity for GetPrivateMessages {
  type Response = PrivateMessagesResponse;
}

impl SendActivity for CreateSite {
  type Response = SiteResponse;
}

impl SendActivity for EditSite {
  type Response = SiteResponse;
}

impl SendActivity for GetSite {
  type Response = GetSiteResponse;
}

impl SendActivity for ListCommunities {
  type Response = ListCommunitiesResponse;
}

impl SendActivity for CreateCommunity {
  type Response = CommunityResponse;
}

impl SendActivity for GetPost {
  type Response = GetPostResponse;
}

impl SendActivity for GetComment {
  type Response = CommentResponse;
}

impl SendActivity for Login {
  type Response = LoginResponse;
}

impl SendActivity for GetCaptcha {
  type Response = GetCaptchaResponse;
}

impl SendActivity for GetReplies {
  type Response = GetRepliesResponse;
}

impl SendActivity for AddAdmin {
  type Response = AddAdminResponse;
}

impl SendActivity for GetUnreadRegistrationApplicationCount {
  type Response = GetUnreadRegistrationApplicationCountResponse;
}

impl SendActivity for ListRegistrationApplications {
  type Response = ListRegistrationApplicationsResponse;
}

impl SendActivity for ApproveRegistrationApplication {
  type Response = RegistrationApplicationResponse;
}

impl SendActivity for GetBannedPersons {
  type Response = BannedPersonsResponse;
}

impl SendActivity for BlockPerson {
  type Response = BlockPersonResponse;
}

impl SendActivity for GetPersonMentions {
  type Response = GetPersonMentionsResponse;
}

impl SendActivity for MarkPersonMentionAsRead {
  type Response = PersonMentionResponse;
}

impl SendActivity for MarkCommentReplyAsRead {
  type Response = CommentReplyResponse;
}

impl SendActivity for MarkAllAsRead {
  type Response = GetRepliesResponse;
}

impl SendActivity for PasswordReset {
  type Response = PasswordResetResponse;
}

impl SendActivity for PasswordChangeAfterReset {
  type Response = LoginResponse;
}

impl SendActivity for SaveUserSettings {
  type Response = LoginResponse;
}

impl SendActivity for ChangePassword {
  type Response = LoginResponse;
}

impl SendActivity for GetReportCount {
  type Response = GetReportCountResponse;
}

impl SendActivity for GetUnreadCount {
  type Response = GetUnreadCountResponse;
}

impl SendActivity for VerifyEmail {
  type Response = VerifyEmailResponse;
}

impl SendActivity for MarkPrivateMessageAsRead {
  type Response = PrivateMessageResponse;
}

impl SendActivity for CreatePrivateMessageReport {
  type Response = PrivateMessageReportResponse;
}

impl SendActivity for ResolvePrivateMessageReport {
  type Response = PrivateMessageReportResponse;
}

impl SendActivity for ListPrivateMessageReports {
  type Response = ListPrivateMessageReportsResponse;
}

impl SendActivity for GetModlog {
  type Response = GetModlogResponse;
}

impl SendActivity for PurgePerson {
  type Response = PurgeItemResponse;
}

impl SendActivity for PurgeCommunity {
  type Response = PurgeItemResponse;
}

impl SendActivity for PurgePost {
  type Response = PurgeItemResponse;
}

impl SendActivity for PurgeComment {
  type Response = PurgeItemResponse;
}

impl SendActivity for TransferCommunity {
  type Response = GetCommunityResponse;
}

impl SendActivity for LeaveAdmin {
  type Response = GetSiteResponse;
}

impl SendActivity for MarkPostAsRead {
  type Response = PostResponse;
}

impl SendActivity for SavePost {
  type Response = PostResponse;
}

impl SendActivity for ListPostReports {
  type Response = ListPostReportsResponse;
}

impl SendActivity for ResolvePostReport {
  type Response = PostReportResponse;
}

impl SendActivity for GetSiteMetadata {
  type Response = GetSiteMetadataResponse;
}

impl SendActivity for SaveComment {
  type Response = CommentResponse;
}

impl SendActivity for DistinguishComment {
  type Response = CommentResponse;
}

impl SendActivity for ListCommentReports {
  type Response = ListCommentReportsResponse;
}

impl SendActivity for ResolveCommentReport {
  type Response = CommentReportResponse;
}

impl SendActivity for CreateCustomEmoji {
  type Response = CustomEmojiResponse;
}

impl SendActivity for EditCustomEmoji {
  type Response = CustomEmojiResponse;
}

impl SendActivity for DeleteCustomEmoji {
  type Response = DeleteCustomEmojiResponse;
}

impl SendActivity for GetFederatedInstances {
  type Response = GetFederatedInstancesResponse;
}

impl SendActivity for Web3Register {
  type Response = LoginResponse;
}
impl SendActivity for Web3Login {
  type Response = LoginResponse;
}

impl SendActivity for PiRegister {
  type Response = LoginResponse;
}

impl SendActivity for PiLogin {
  type Response = LoginResponse;
}

impl SendActivity for PiAgreeRegister {
  type Response = PiAgreeResponse;
}

impl SendActivity for PiRegisterWithFee {
  type Response = LoginResponse;
}
impl SendActivity for PiApprove {
  type Response = PiApproveResponse;
}
impl SendActivity for PiPaymentComplete {
  type Response = PiPaymentCompleteResponse;
}
impl SendActivity for PiPaymentFound {
  type Response = PiPaymentFoundResponse;
}
impl SendActivity for PiKey {
  type Response = PiKeyResponse;
}
impl SendActivity for CreatePayment {
  type Response = CreatePaymentResponse;
}
impl SendActivity for GetPayment {
  type Response = GetPaymentResponse;
}
impl SendActivity for GetPayments {
  type Response = GetPaymentsResponse;
}
impl SendActivity for GetPiBalances {
  type Response = GetPiBalancesResponse;
}
impl SendActivity for PiWithdraw {
  type Response = PiWithdrawResponse;
}
impl SendActivity for SendPayment {
  type Response = SendPaymentResponse;
}

// impl SendActivity for GetToken {
//   type Response = GetTokenResponse;
// }

// impl SendActivity for GetMyUserInfo {
//   type Response = GetMyUserInfoResponse;
// }
