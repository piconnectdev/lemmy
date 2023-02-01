use crate::SendActivity;
use lemmy_api_common::{
  comment::{
    CommentReportResponse,
    CommentResponse,
    GetComment,
    GetComments,
    GetCommentsResponse,
    ListCommentReports,
    ListCommentReportsResponse,
    ResolveCommentReport,
    SaveComment,
  },
  community::{
    CommunityResponse,
    CreateCommunity,
    GetCommunity,
    GetCommunityResponse,
    ListCommunities,
    ListCommunitiesResponse,
    TransferCommunity,
  },
  person::{
    AddAdmin,
    AddAdminResponse,
    BannedPersonsResponse,
    BlockPerson,
    BlockPersonResponse,
    ChangePassword,
    CommentReplyResponse,
    GetBannedPersons,
    GetCaptcha,
    GetCaptchaResponse,
    GetPersonDetails,
    GetPersonDetailsResponse,
    GetPersonMentions,
    GetPersonMentionsResponse,
    GetReplies,
    GetRepliesResponse,
    GetReportCount,
    GetReportCountResponse,
    GetUnreadCount,
    GetUnreadCountResponse,
    Login,
    LoginResponse,
    MarkAllAsRead,
    MarkCommentReplyAsRead,
    MarkPersonMentionAsRead,
    PasswordChangeAfterReset,
    PasswordReset,
    PasswordResetResponse,
    PersonMentionResponse,
    Register,
    SaveUserSettings,
    VerifyEmail,
    VerifyEmailResponse, GetToken, GetTokenResponse,
  },
  post::{
    GetPost,
    GetPostResponse,
    GetPosts,
    GetPostsResponse,
    GetSiteMetadata,
    GetSiteMetadataResponse,
    ListPostReports,
    ListPostReportsResponse,
    MarkPostAsRead,
    PostReportResponse,
    PostResponse,
    ResolvePostReport,
    SavePost,
  },
  private_message::{
    CreatePrivateMessageReport,
    GetPrivateMessages,
    ListPrivateMessageReports,
    ListPrivateMessageReportsResponse,
    MarkPrivateMessageAsRead,
    PrivateMessageReportResponse,
    PrivateMessageResponse,
    PrivateMessagesResponse,
    ResolvePrivateMessageReport,
  },
  site::{
    ApproveRegistrationApplication,
    CreateSite,
    EditSite,
    GetModlog,
    GetModlogResponse,
    GetSite,
    GetSiteResponse,
    GetUnreadRegistrationApplicationCount,
    GetUnreadRegistrationApplicationCountResponse,
    LeaveAdmin,
    ListRegistrationApplications,
    ListRegistrationApplicationsResponse,
    PurgeComment,
    PurgeCommunity,
    PurgeItemResponse,
    PurgePerson,
    PurgePost,
    RegistrationApplicationResponse,
    ResolveObject,
    ResolveObjectResponse,
    Search,
    SearchResponse,
    SiteResponse,
  },
  websocket::structs::{
    CommunityJoin,
    CommunityJoinResponse,
    ModJoin,
    ModJoinResponse,
    PostJoin,
    PostJoinResponse,
    UserJoin,
    UserJoinResponse,
  }, 
  web3::{
    Web3Register,
    Web3Login,
  }, 
  pipayment::{
    PiRegister,
    PiLogin,
    PiAgreeRegister,
    PiRegisterWithFee,
    PiApprove,
    PiTip,
    PiPaymentFound, 
    PiAgreeResponse, 
    PiApproveResponse,
    PiTipResponse, 
    PiPaymentFoundResponse, 
    PiKey, 
    PiKeyResponse, PiWithdraw, PiWithdrawResponse, GetPiBalances, GetPiPayments, GetPiPaymentsResponse, GetPiBalancesResponse,
  },
};

impl SendActivity for Register {
  type Response = LoginResponse;
}

impl SendActivity for GetPersonDetails {
  type Response = GetPersonDetailsResponse;
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

impl SendActivity for GetCommunity {
  type Response = GetCommunityResponse;
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

impl SendActivity for GetPosts {
  type Response = GetPostsResponse;
}

impl SendActivity for GetComment {
  type Response = CommentResponse;
}

impl SendActivity for GetComments {
  type Response = GetCommentsResponse;
}

impl SendActivity for Login {
  type Response = LoginResponse;
}

impl SendActivity for GetCaptcha {
  type Response = GetCaptchaResponse;
}

impl SendActivity for GetToken {
  type Response = GetTokenResponse;
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

impl SendActivity for UserJoin {
  type Response = UserJoinResponse;
}

impl SendActivity for PostJoin {
  type Response = PostJoinResponse;
}

impl SendActivity for CommunityJoin {
  type Response = CommunityJoinResponse;
}

impl SendActivity for ModJoin {
  type Response = ModJoinResponse;
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

impl SendActivity for Search {
  type Response = SearchResponse;
}

impl SendActivity for ResolveObject {
  type Response = ResolveObjectResponse;
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

impl SendActivity for ListCommentReports {
  type Response = ListCommentReportsResponse;
}

impl SendActivity for ResolveCommentReport {
  type Response = CommentReportResponse;
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
impl SendActivity for PiTip {
  type Response = PiTipResponse;
}
impl SendActivity for PiPaymentFound {
  type Response = PiPaymentFoundResponse;
}
impl SendActivity for PiKey {
  type Response = PiKeyResponse;
}
impl SendActivity for GetPiPayments {
  type Response = GetPiPaymentsResponse;
}
impl SendActivity for GetPiBalances {
  type Response = GetPiBalancesResponse;
}
impl SendActivity for PiWithdraw {
  type Response = PiWithdrawResponse;
}

