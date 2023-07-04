// @generated automatically by Diesel CLI.

pub mod sql_types {
  #[derive(diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "listing_type_enum"))]
  pub struct ListingTypeEnum;

  #[derive(diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "registration_mode_enum"))]
  pub struct RegistrationModeEnum;

  #[derive(diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "sort_type_enum"))]
  pub struct SortTypeEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    activity (id) {
        id -> Uuid,
        data -> Jsonb,
        local -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        ap_id -> Text,
        sensitive -> Bool,
    }
}

diesel::table! {
  use diesel::sql_types::*;
  admin_purge_comment (id) {
    id -> Uuid,
    admin_person_id -> Uuid,
    post_id -> Uuid,
    reason -> Nullable<Text>,
    when_ -> Timestamp,
  }
}

diesel::table! {
  use diesel::sql_types::*;
  admin_purge_community (id) {
    id -> Uuid,
    admin_person_id -> Uuid,
    reason -> Nullable<Text>,
    when_ -> Timestamp,
  }
}

diesel::table! {
  use diesel::sql_types::*;
  admin_purge_person (id) {
    id -> Uuid,
    admin_person_id -> Uuid,
    reason -> Nullable<Text>,
    when_ -> Timestamp,
  }
}

diesel::table! {
  use diesel::sql_types::*;
  admin_purge_post (id) {
    id -> Uuid,
    admin_person_id -> Uuid,
    community_id -> Uuid,
    reason -> Nullable<Text>,
    when_ -> Timestamp,
  }
}

diesel::table! {
  use diesel::sql_types::{Bool, Int4, Nullable, Text, Timestamp, Varchar};
  use diesel_ltree::sql_types::Ltree;
  use diesel::sql_types::*;
    captcha_answer (id) {
        id -> Int4,
        uuid -> Uuid,
        answer -> Text,
        published -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_ltree::sql_types::Ltree;

    comment (id) {
        id -> Uuid,
        creator_id -> Uuid,
        post_id -> Uuid,
        content -> Text,
        removed -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        deleted -> Bool,
        #[max_length = 255]
        ap_id -> Varchar,
        local -> Bool,
        path -> Ltree,
        distinguished -> Bool,
        language_id -> Int4,
        auth_sign -> Nullable<Text>,
        srv_sign -> Nullable<Text>,
        pipayid -> Nullable<Text>,
        tx -> Nullable<Text>,
        //extras -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    comment_aggregates (id) {
        id -> Uuid,
        comment_id -> Uuid,
        score -> Int8,
        upvotes -> Int8,
        downvotes -> Int8,
        published -> Timestamp,
        child_count -> Int4,
        hot_rank -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    comment_like (id) {
        id -> Uuid,
        person_id -> Uuid,
        comment_id -> Uuid,
        post_id -> Uuid,
        score -> Int2,
        published -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    comment_reply (id) {
        id -> Uuid,
        recipient_id -> Uuid,
        comment_id -> Uuid,
        read -> Bool,
        published -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    comment_report (id) {
        id -> Uuid,
        creator_id -> Uuid,
        comment_id -> Uuid,
        original_comment_text -> Text,
        reason -> Text,
        resolved -> Bool,
        resolver_id -> Nullable<Uuid>,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    comment_saved (id) {
        id -> Uuid,
        comment_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    community (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        removed -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        deleted -> Bool,
        nsfw -> Bool,
        #[max_length = 255]
        actor_id -> Varchar,
        local -> Bool,
        private_key -> Nullable<Text>,
        public_key -> Text,
        last_refreshed_at -> Timestamp,
        icon -> Nullable<Text>,
        banner -> Nullable<Text>,
        #[max_length = 255]
        followers_url -> Varchar,
        #[max_length = 255]
        inbox_url -> Varchar,
        #[max_length = 255]
        shared_inbox_url -> Nullable<Varchar>,
        hidden -> Bool,
        posting_restricted_to_mods -> Bool,
        instance_id -> Uuid,
        #[max_length = 255]
        moderators_url -> Nullable<Varchar>,
        #[max_length = 255]
        featured_url -> Nullable<Varchar>,
        is_home -> Bool,
        person_id -> Nullable<Uuid>,
        srv_sign -> Nullable<Text>,
        pipayid -> Nullable<Text>,
        tx -> Nullable<Text>,
        //extras -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    community_aggregates (id) {
        id -> Uuid,
        community_id -> Uuid,
        subscribers -> Int8,
        posts -> Int8,
        comments -> Int8,
        published -> Timestamp,
        users_active_day -> Int8,
        users_active_week -> Int8,
        users_active_month -> Int8,
        users_active_half_year -> Int8,
        hot_rank -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    community_block (id) {
        id -> Uuid,
        person_id -> Uuid,
        community_id -> Uuid,
        published -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    community_follower (id) {
        id -> Uuid,
        community_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
        pending -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    community_language(id) {
        id -> Uuid,
        community_id -> Uuid,
        language_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    community_moderator (id) {
        id -> Uuid,
        community_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    community_person_ban (id) {
        id -> Uuid,
        community_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
        expires -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    custom_emoji (id) {
        id -> Uuid,
        local_site_id -> Uuid,
        #[max_length = 128]
        shortcode -> Varchar,
        image_url -> Text,
        alt_text -> Text,
        category -> Text,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    custom_emoji_keyword (id) {
        id -> Uuid,
        custom_emoji_id -> Uuid,
        #[max_length = 128]
        keyword -> Varchar,
    }
}

diesel::table! {
  use diesel::sql_types::*;
  email_verification (id) {
    id -> Uuid,
    local_user_id -> Uuid,
    email -> Text,
    verification_token -> Text,
    published -> Timestamp,
  }
}

diesel::table! {
  use diesel::sql_types::*;
  federation_allowlist(id) {
    id -> Uuid,
    instance_id -> Uuid,
    published -> Timestamp,
    updated -> Nullable<Timestamp>,
  }
}

diesel::table! {
  use diesel::sql_types::*;
  federation_blocklist(id) {
    id -> Uuid,
    instance_id -> Uuid,
    published -> Timestamp,
    updated -> Nullable<Timestamp>,
  }
}

diesel::table! {
  use diesel::sql_types::*;
  instance(id) {
    id -> Uuid,
    #[max_length = 255]
    domain -> Varchar,
    published -> Timestamp,
    updated -> Nullable<Timestamp>,
    #[max_length = 255]
    software -> Nullable<Varchar>,
    #[max_length = 255]
    version -> Nullable<Varchar>,
  }
}

diesel::table! {
    use diesel::sql_types::*;
    language (id) {
        id -> Int4,
        #[max_length = 3]
        code -> Varchar,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ListingTypeEnum;
    use super::sql_types::RegistrationModeEnum;

    local_site (id) {
        id -> Uuid,
        site_id -> Uuid,
        site_setup -> Bool,
        enable_downvotes -> Bool,
        enable_nsfw -> Bool,
        community_creation_admin_only -> Bool,
        require_email_verification -> Bool,
        application_question -> Nullable<Text>,
        private_instance -> Bool,
        default_theme -> Text,
        default_post_listing_type -> ListingTypeEnum,
        legal_information -> Nullable<Text>,
        hide_modlog_mod_names -> Bool,
        application_email_admins -> Bool,
        slur_filter_regex -> Nullable<Text>,
        actor_name_max_length -> Int4,
        federation_enabled -> Bool,
        captcha_enabled -> Bool,
        #[max_length = 255]
        captcha_difficulty -> Varchar,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        registration_mode -> RegistrationModeEnum,
        reports_email_admins -> Bool,
    }
}

diesel::table! {
  use diesel::sql_types::*;
  local_site_rate_limit(id) {
    id -> Uuid,
    local_site_id -> Uuid,
    message -> Int4,
    message_per_second-> Int4,
    post -> Int4,
    post_per_second -> Int4,
    register -> Int4,
    register_per_second -> Int4,
    image -> Int4,
    image_per_second -> Int4,
    comment -> Int4,
    comment_per_second -> Int4,
    search -> Int4,
    search_per_second -> Int4,
    published -> Timestamp,
    updated -> Nullable<Timestamp>,
  }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SortTypeEnum;
    use super::sql_types::ListingTypeEnum;

    local_user (id) {
        id -> Uuid,
        person_id -> Uuid,
        password_encrypted -> Text,
        email -> Nullable<Text>,
        show_nsfw -> Bool,
        theme -> Text,
        default_sort_type -> SortTypeEnum,
        default_listing_type -> ListingTypeEnum,
        #[max_length = 20]
        interface_language -> Varchar,
        show_avatars -> Bool,
        send_notifications_to_email -> Bool,
        validator_time -> Timestamp,
        show_scores -> Bool,
        show_bot_accounts -> Bool,
        show_read_posts -> Bool,
        show_new_post_notifs -> Bool,
        email_verified -> Bool,
        accepted_application -> Bool,
        totp_2fa_secret -> Nullable<Text>,
        totp_2fa_url -> Nullable<Text>,
        open_links_in_new_tab -> Bool,
        //private_seeds -> Nullable<Text>,
        //signing_data -> Bool,
        //extras -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    local_user_language(id) {
        id -> Uuid,
        local_user_id -> Uuid,
        language_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    mod_add (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        removed -> Bool,
        when_ -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    mod_add_community (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        community_id -> Uuid,
        removed -> Bool,
        when_ -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    mod_ban (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        reason -> Nullable<Text>,
        banned -> Bool,
        expires -> Nullable<Timestamp>,
        when_ -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    mod_ban_from_community (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        community_id -> Uuid,
        reason -> Nullable<Text>,
        banned -> Bool,
        expires -> Nullable<Timestamp>,
        when_ -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    mod_feature_post (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        post_id -> Uuid,
        featured -> Bool,
        when_ -> Timestamp,
        is_featured_community -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    mod_hide_community (id) {
        id -> Uuid,
        community_id -> Uuid,
        mod_person_id -> Uuid,
        when_ -> Timestamp,
        reason -> Nullable<Text>,
        hidden -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    mod_lock_post (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        post_id -> Uuid,
        locked -> Bool,
        when_ -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    mod_remove_comment (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        comment_id -> Uuid,
        reason -> Nullable<Text>,
        removed -> Bool,
        when_ -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    mod_remove_community (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        community_id -> Uuid,
        reason -> Nullable<Text>,
        removed -> Bool,
        expires -> Nullable<Timestamp>,
        when_ -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    mod_remove_post (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        post_id -> Uuid,
        reason -> Nullable<Text>,
        removed -> Bool,
        when_ -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    mod_transfer_community (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        community_id -> Uuid,
        when_ -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    password_reset_request (id) {
        id -> Uuid,
        token_encrypted -> Text,
        published -> Timestamp,
        local_user_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    person (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        display_name -> Nullable<Varchar>,
        avatar -> Nullable<Text>,
        banned -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        #[max_length = 255]
        actor_id -> Varchar,
        bio -> Nullable<Text>,
        local -> Bool,
        private_key -> Nullable<Text>,
        public_key -> Text,
        last_refreshed_at -> Timestamp,
        banner -> Nullable<Varchar>,
        deleted -> Bool,
        #[max_length = 255]
        inbox_url -> Varchar,
        #[max_length = 255]
        shared_inbox_url -> Nullable<Varchar>,
        matrix_user_id -> Nullable<Text>,
        admin -> Bool,
        bot_account -> Bool,
        ban_expires -> Nullable<Timestamp>,
        instance_id -> Uuid,
        home -> Nullable<Uuid>,
        external_id -> Nullable<Text>,
        external_name -> Nullable<Text>,
        //private_seeds -> Nullable<Text>,
        verified -> Bool,
        pi_address -> Nullable<Text>,
        web3_address -> Nullable<Text>,
        pol_address -> Nullable<Text>,
        dap_address -> Nullable<Text>,
        cosmos_address -> Nullable<Text>,
        sui_address -> Nullable<Text>,
        auth_sign -> Nullable<Text>,
        srv_sign -> Nullable<Text>,
        pipayid -> Nullable<Text>,
        tx -> Nullable<Text>,
        //extras -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    person_aggregates (id) {
        id -> Uuid,
        person_id -> Uuid,
        post_count -> Int8,
        post_score -> Int8,
        comment_count -> Int8,
        comment_score -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    person_ban (id) {
        id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    person_block (id) {
        id -> Uuid,
        person_id -> Uuid,
        target_id -> Uuid,
        published -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    person_follower (id) {
        id -> Uuid,
        person_id -> Uuid,
        follower_id -> Uuid,
        published -> Timestamp,
        pending -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    person_mention (id) {
        id -> Uuid,
        recipient_id -> Uuid,
        comment_id -> Uuid,
        read -> Bool,
        published -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    person_post_aggregates (id) {
        id -> Uuid,
        person_id -> Uuid,
        post_id -> Uuid,
        read_comments -> Int8,
        published -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    post (id) {
        id -> Uuid,
        #[max_length = 200]
        name -> Varchar,
        #[max_length = 512]
        url -> Nullable<Varchar>,
        body -> Nullable<Text>,
        creator_id -> Uuid,
        community_id -> Uuid,
        removed -> Bool,
        locked -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        deleted -> Bool,
        nsfw -> Bool,
        embed_title -> Nullable<Text>,
        embed_description -> Nullable<Text>,
        thumbnail_url -> Nullable<Text>,
        #[max_length = 255]
        ap_id -> Varchar,
        local -> Bool,
        embed_video_url -> Nullable<Text>,
        language_id -> Int4,
        featured_community -> Bool,
        featured_local -> Bool,

        auth_sign -> Nullable<Text>,
        srv_sign -> Nullable<Text>,
        pipayid -> Nullable<Text>,
        tx -> Nullable<Text>,
        //extras -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    post_aggregates (id) {
        id -> Uuid,
        post_id -> Uuid,
        comments -> Int8,
        score -> Int8,
        upvotes -> Int8,
        downvotes -> Int8,
        published -> Timestamp,
        newest_comment_time_necro -> Timestamp,
        newest_comment_time -> Timestamp,
        featured_community -> Bool,
        featured_local -> Bool,
        hot_rank -> Int4,
        hot_rank_active -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    post_like (id) {
        id -> Uuid,
        post_id -> Uuid,
        person_id -> Uuid,
        score -> Int2,
        published -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    post_read (id) {
        id -> Uuid,
        post_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    post_report (id) {
        id -> Uuid,
        creator_id -> Uuid,
        post_id -> Uuid,
        #[max_length = 200]
        original_post_name -> Varchar,
        original_post_url -> Nullable<Text>,
        original_post_body -> Nullable<Text>,
        reason -> Text,
        resolved -> Bool,
        resolver_id -> Nullable<Uuid>,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    post_saved (id) {
        id -> Uuid,
        post_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    private_message (id) {
        id -> Uuid,
        creator_id -> Uuid,
        recipient_id -> Uuid,
        content -> Text,
        deleted -> Bool,
        read -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        #[max_length = 255]
        ap_id -> Varchar,
        local -> Bool,
        secured -> Nullable<Text>,
        auth_sign -> Nullable<Text>,
        srv_sign -> Nullable<Text>,
        pipayid -> Nullable<Text>,
        tx -> Nullable<Text>,
        //extras -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    private_message_report (id) {
        id -> Uuid,
        creator_id -> Uuid,
        private_message_id -> Uuid,
        original_pm_text -> Text,
        reason -> Text,
        resolved -> Bool,
        resolver_id -> Nullable<Uuid>,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    registration_application (id) {
        id -> Uuid,
        local_user_id -> Uuid,
        answer -> Text,
        admin_id -> Nullable<Uuid>,
        deny_reason -> Nullable<Text>,
        published -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    secret(id) {
        id -> Uuid,
        jwt_secret -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    site (id) {
        id -> Uuid,
        #[max_length = 20]
        name -> Varchar,
        sidebar -> Nullable<Text>,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        icon -> Nullable<Text>,
        banner -> Nullable<Text>,
        #[max_length = 150]
        description -> Nullable<Text>,
        #[max_length = 255]
        actor_id -> Text,
        last_refreshed_at -> Timestamp,
        #[max_length = 255]
        inbox_url -> Text,
        private_key -> Nullable<Text>,
        public_key -> Text,
        instance_id -> Uuid,
        srv_sign -> Nullable<Text>,
        pipayid -> Nullable<Text>,
        tx -> Nullable<Text>,
        //extras -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    site_aggregates (id) {
        id -> Uuid,
        site_id -> Uuid,
        users -> Int8,
        posts -> Int8,
        comments -> Int8,
        communities -> Int8,
        users_active_day -> Int8,
        users_active_week -> Int8,
        users_active_month -> Int8,
        users_active_half_year -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    site_language(id) {
        id -> Uuid,
        site_id -> Uuid,
        language_id -> Int4,
    }
}

diesel::table! {
  use diesel::sql_types::*;
  tagline(id) {
    id -> Uuid,
    local_site_id -> Uuid,
    content -> Text,
    published -> Timestamp,
    updated -> Nullable<Timestamp>,
  }
}

diesel::table! {
    use diesel::sql_types::*;
    pipayment (id) {
        id -> Uuid,
        domain -> Nullable<Text>,      //
        instance_id -> Nullable<Uuid>, // WePi instance
        person_id -> Nullable<Uuid>, // WePi user's id
        obj_cat -> Nullable<Text>,   // register - page - note - message - person - instance - group, withdraw, deposit ...
        obj_id -> Nullable<Uuid>,    // Post id - comment id, chat message id, site id, instance id, person id, community id
        a2u -> Int4,
        step -> Int4,
        asset -> Nullable<Text>,
        fee -> Double,
        testnet -> Bool,
        finished -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        ref_id -> Nullable<Uuid>,
        comment -> Nullable<Text>,
        stat -> Nullable<Text>,

        pi_uid -> Nullable<Uuid>,   // UserDTO - uid
        pi_username -> Text,        // UserDTO - username
        identifier -> Nullable<Text>,         // PaymentDto - identifier
        user_uid -> Nullable<Text>,           // PaymentDto - user_uid
        amount -> Double,
        memo -> Nullable<Text>,
        from_address -> Nullable<Text>,
        to_address -> Nullable<Text>,
        direction -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,

        approved -> Bool,
        verified -> Bool,
        completed -> Bool,
        cancelled -> Bool,
        user_cancelled -> Bool,
        tx_verified -> Bool,
        tx_link -> Nullable<Text>,
        tx_id -> Nullable<Text>,
        network -> Nullable<Text>,
        metadata -> Nullable<Jsonb>,
        extras -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    person_balance (id) {
        id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
        asset -> Nullable<Text>,
        deposited -> Double,
        received -> Double,
        withdrawed -> Double,
        spent -> Double,
        amount -> Double,
        pending -> Double,
        updated -> Nullable<Timestamp>,
        extras -> Nullable<Jsonb>,
    }
}

// diesel::table! {
//     person_web3address (id) {
//         id -> Uuid,
//         person_id -> Nullable<Uuid>,
//         name -> Text,
//         pi -> Nullable<Text>,
//         web3 -> Nullable<Text>,
//         dap -> Nullable<Text>,
//         cosmos -> Nullable<Text>,
//         xlm -> Nullable<Text>,
//         sui -> Nullable<Text>,
//         ton -> Nullable<Text>,
//         egld -> Nullable<Text>,
//         near -> Nullable<Text>,
//         apt -> Nullable<Text>,
//         dot -> Nullable<Text>,
//         mina -> Nullable<Text>,
//         zil -> Nullable<Text>,
//         avax -> Nullable<Text>,
//         icp -> Nullable<Text>,
//         trx -> Nullable<Text>,
//         xtz -> Nullable<Text>,
//         ada -> Nullable<Text>,
//         btc -> Nullable<Text>,
//         doge -> Nullable<Text>,
//         xmr -> Nullable<Text>,
//         extras -> Nullable<Jsonb>,
//     }
// }

//joinable!(pipayment -> person (person_id));
//joinable!(person_web3address -> person (person_id));

diesel::joinable!(admin_purge_comment -> person (admin_person_id));
diesel::joinable!(admin_purge_comment -> post (post_id));
diesel::joinable!(admin_purge_community -> person (admin_person_id));
diesel::joinable!(admin_purge_person -> person (admin_person_id));
diesel::joinable!(admin_purge_post -> community (community_id));
diesel::joinable!(admin_purge_post -> person (admin_person_id));
diesel::joinable!(comment -> language (language_id));
diesel::joinable!(comment -> person (creator_id));
diesel::joinable!(comment -> post (post_id));
diesel::joinable!(comment_aggregates -> comment (comment_id));
diesel::joinable!(comment_like -> comment (comment_id));
diesel::joinable!(comment_like -> person (person_id));
diesel::joinable!(comment_like -> post (post_id));
diesel::joinable!(comment_reply -> comment (comment_id));
diesel::joinable!(comment_reply -> person (recipient_id));
diesel::joinable!(comment_report -> comment (comment_id));
diesel::joinable!(comment_saved -> comment (comment_id));
diesel::joinable!(comment_saved -> person (person_id));
diesel::joinable!(community -> instance (instance_id));
diesel::joinable!(community_aggregates -> community (community_id));
diesel::joinable!(community_block -> community (community_id));
diesel::joinable!(community_block -> person (person_id));
diesel::joinable!(community_follower -> community (community_id));
diesel::joinable!(community_follower -> person (person_id));
diesel::joinable!(community_language -> community (community_id));
diesel::joinable!(community_language -> language (language_id));
diesel::joinable!(community_moderator -> community (community_id));
diesel::joinable!(community_moderator -> person (person_id));
diesel::joinable!(community_person_ban -> community (community_id));
diesel::joinable!(community_person_ban -> person (person_id));
diesel::joinable!(custom_emoji -> local_site (local_site_id));
diesel::joinable!(custom_emoji_keyword -> custom_emoji (custom_emoji_id));
diesel::joinable!(email_verification -> local_user (local_user_id));
diesel::joinable!(federation_allowlist -> instance (instance_id));
diesel::joinable!(federation_blocklist -> instance (instance_id));
diesel::joinable!(local_site -> site (site_id));
diesel::joinable!(local_site_rate_limit -> local_site (local_site_id));
diesel::joinable!(local_user -> person (person_id));
diesel::joinable!(local_user_language -> language (language_id));
diesel::joinable!(local_user_language -> local_user (local_user_id));
diesel::joinable!(mod_add_community -> community (community_id));
diesel::joinable!(mod_ban_from_community -> community (community_id));
diesel::joinable!(mod_feature_post -> person (mod_person_id));
diesel::joinable!(mod_feature_post -> post (post_id));
diesel::joinable!(mod_hide_community -> community (community_id));
diesel::joinable!(mod_hide_community -> person (mod_person_id));
diesel::joinable!(mod_lock_post -> person (mod_person_id));
diesel::joinable!(mod_lock_post -> post (post_id));
diesel::joinable!(mod_remove_comment -> comment (comment_id));
diesel::joinable!(mod_remove_comment -> person (mod_person_id));
diesel::joinable!(mod_remove_community -> community (community_id));
diesel::joinable!(mod_remove_community -> person (mod_person_id));
diesel::joinable!(mod_remove_post -> person (mod_person_id));
diesel::joinable!(mod_remove_post -> post (post_id));
diesel::joinable!(mod_transfer_community -> community (community_id));
diesel::joinable!(password_reset_request -> local_user (local_user_id));
diesel::joinable!(person -> instance (instance_id));
diesel::joinable!(person_aggregates -> person (person_id));
diesel::joinable!(person_ban -> person (person_id));
diesel::joinable!(person_mention -> comment (comment_id));
diesel::joinable!(person_mention -> person (recipient_id));
diesel::joinable!(person_post_aggregates -> person (person_id));
diesel::joinable!(person_post_aggregates -> post (post_id));
diesel::joinable!(post -> community (community_id));
diesel::joinable!(post -> language (language_id));
diesel::joinable!(post -> person (creator_id));
diesel::joinable!(post_aggregates -> post (post_id));
diesel::joinable!(post_like -> person (person_id));
diesel::joinable!(post_like -> post (post_id));
diesel::joinable!(post_read -> person (person_id));
diesel::joinable!(post_read -> post (post_id));
diesel::joinable!(post_report -> post (post_id));
diesel::joinable!(post_saved -> person (person_id));
diesel::joinable!(post_saved -> post (post_id));
diesel::joinable!(private_message_report -> private_message (private_message_id));
diesel::joinable!(registration_application -> local_user (local_user_id));
diesel::joinable!(registration_application -> person (admin_id));
diesel::joinable!(site -> instance (instance_id));
diesel::joinable!(site_aggregates -> site (site_id));
diesel::joinable!(site_language -> language (language_id));
diesel::joinable!(site_language -> site (site_id));
diesel::joinable!(tagline -> local_site (local_site_id));

joinable!(person_balance -> person (person_id));

diesel::allow_tables_to_appear_in_same_query!(
  activity,
  admin_purge_comment,
  admin_purge_community,
  admin_purge_person,
  admin_purge_post,
  captcha_answer,
  comment,
  comment_aggregates,
  comment_like,
  comment_reply,
  comment_report,
  comment_saved,
  community,
  community_aggregates,
  community_block,
  community_follower,
  community_language,
  community_moderator,
  community_person_ban,
  custom_emoji,
  custom_emoji_keyword,
  email_verification,
  federation_allowlist,
  federation_blocklist,
  instance,
  language,
  local_site,
  local_site_rate_limit,
  local_user,
  local_user_language,
  mod_add,
  mod_add_community,
  mod_ban,
  mod_ban_from_community,
  mod_feature_post,
  mod_hide_community,
  mod_lock_post,
  mod_remove_comment,
  mod_remove_community,
  mod_remove_post,
  mod_transfer_community,
  password_reset_request,
  person,
  person_aggregates,
  person_ban,
  person_block,
  person_follower,
  person_mention,
  person_post_aggregates,
  post,
  post_aggregates,
  post_like,
  post_read,
  post_report,
  post_saved,
  private_message,
  private_message_report,
  registration_application,
  secret,
  site,
  site_aggregates,
  site_language,
  tagline,
  pipayment,
  person_balance,
);
