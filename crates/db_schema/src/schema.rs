table! {
    activity (id) {
        id -> Uuid,
        data -> Jsonb,
        local -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        ap_id -> Text,
        sensitive -> Nullable<Bool>,
    }
}

table! {
  use diesel_ltree::sql_types::Ltree;
  use diesel::sql_types::*;

    comment (id) {
        id -> Uuid,
        creator_id -> Uuid,
        post_id -> Uuid,
        content -> Text,
        removed -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        deleted -> Bool,
        ap_id -> Varchar,
        local -> Bool,
        path -> Ltree,
        distinguished -> Bool,
        language_id -> Int4,
        auth_sign -> Nullable<Text>,
        srv_sign -> Nullable<Text>,
        tx -> Nullable<Text>,
        //extras -> Nullable<Jsonb>,
    }
}

table! {
    comment_aggregates (id) {
        id -> Uuid,
        comment_id -> Uuid,
        score -> Int8,
        upvotes -> Int8,
        downvotes -> Int8,
        published -> Timestamp,
        child_count ->  Int4,
    }
}

table! {
    comment_like (id) {
        id -> Uuid,
        person_id -> Uuid,
        comment_id -> Uuid,
        post_id -> Uuid,
        score -> Int2,
        published -> Timestamp,
    }
}

table! {
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

table! {
    comment_saved (id) {
        id -> Uuid,
        comment_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

table! {
    community (id) {
        id -> Uuid,
        name -> Varchar,
        title -> Varchar,
        description -> Nullable<Text>,
        removed -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        deleted -> Bool,
        nsfw -> Bool,
        actor_id -> Varchar,
        local -> Bool,
        private_key -> Nullable<Text>,
        public_key -> Text,
        last_refreshed_at -> Timestamp,
        icon -> Nullable<Varchar>,
        banner -> Nullable<Varchar>,
        followers_url -> Varchar,
        inbox_url -> Varchar,
        shared_inbox_url -> Nullable<Varchar>,
        hidden -> Bool,
        posting_restricted_to_mods -> Bool,
        instance_id -> Uuid,
        srv_sign -> Nullable<Text>,
        tx -> Nullable<Text>,
        //extras -> Nullable<Jsonb>,
    }
}

table! {
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
    }
}

table! {
    community_follower (id) {
        id -> Uuid,
        community_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
        pending -> Bool,
    }
}

table! {
    community_moderator (id) {
        id -> Uuid,
        community_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

table! {
    community_person_ban (id) {
        id -> Uuid,
        community_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
        expires -> Nullable<Timestamp>,
    }
}

table! {
    local_user (id) {
        id -> Uuid,
        person_id -> Uuid,
        password_encrypted -> Text,
        email -> Nullable<Text>,
        show_nsfw -> Bool,
        theme -> Varchar,
        default_sort_type -> Int2,
        default_listing_type -> Int2,
        interface_language -> Varchar,
        show_avatars -> Bool,
        send_notifications_to_email -> Bool,
        validator_time -> Timestamp,
        show_bot_accounts -> Bool,
        show_scores -> Bool,
        show_read_posts -> Bool,
        show_new_post_notifs -> Bool,
        email_verified -> Bool,
        accepted_application -> Bool,
        //private_seeds -> Nullable<Text>,
        //signing_data -> Bool,
        //extras -> Nullable<Jsonb>,
    }
}

table! {
    mod_add (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        removed -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    mod_add_community (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        community_id -> Uuid,
        removed -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    mod_transfer_community (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        community_id -> Uuid,
        removed -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    mod_ban (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        reason -> Nullable<Text>,
        banned -> Nullable<Bool>,
        expires -> Nullable<Timestamp>,
        when_ -> Timestamp,
    }
}

table! {
    mod_ban_from_community (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        community_id -> Uuid,
        reason -> Nullable<Text>,
        banned -> Nullable<Bool>,
        expires -> Nullable<Timestamp>,
        when_ -> Timestamp,
    }
}

table! {
    mod_lock_post (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        post_id -> Uuid,
        locked -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    mod_remove_comment (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        comment_id -> Uuid,
        reason -> Nullable<Text>,
        removed -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    mod_remove_community (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        community_id -> Uuid,
        reason -> Nullable<Text>,
        removed -> Nullable<Bool>,
        expires -> Nullable<Timestamp>,
        when_ -> Timestamp,
    }
}

table! {
    mod_remove_post (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        post_id -> Uuid,
        reason -> Nullable<Text>,
        removed -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    mod_sticky_post (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        post_id -> Uuid,
        stickied -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    password_reset_request (id) {
        id -> Uuid,
        token_encrypted -> Text,
        published -> Timestamp,
        local_user_id -> Uuid,
    }
}

table! {
    person (id) {
        id -> Uuid,
        name -> Varchar,
        display_name -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        banned -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        actor_id -> Varchar,
        bio -> Nullable<Text>,
        local -> Bool,
        private_key -> Nullable<Text>,
        public_key -> Text,
        last_refreshed_at -> Timestamp,
        banner -> Nullable<Varchar>,
        deleted -> Bool,
        inbox_url -> Varchar,
        shared_inbox_url -> Nullable<Varchar>,
        matrix_user_id -> Nullable<Text>,
        admin -> Bool,
        bot_account -> Bool,
        ban_expires -> Nullable<Timestamp>,
        instance_id -> Uuid,	
        external_id -> Nullable<Text>,
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
        tx -> Nullable<Text>,
        //extras -> Nullable<Jsonb>,        
    }
}

table! {
    person_aggregates (id) {
        id -> Uuid,
        person_id -> Uuid,
        post_count -> Int8,
        post_score -> Int8,
        comment_count -> Int8,
        comment_score -> Int8,
    }
}

table! {
    person_ban (id) {
        id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

table! {
    person_mention (id) {
        id -> Uuid,
        recipient_id -> Uuid,
        comment_id -> Uuid,
        read -> Bool,
        published -> Timestamp,
    }
}

table! {
    comment_reply (id) {
        id -> Uuid,
        recipient_id -> Uuid,
        comment_id -> Uuid,
        read -> Bool,
        published -> Timestamp,
    }
}

table! {
    post (id) {
        id -> Uuid,
        name -> Varchar,
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
        stickied -> Bool,
        embed_title -> Nullable<Text>,
        embed_description -> Nullable<Text>,
        embed_video_url -> Nullable<Text>,
        thumbnail_url -> Nullable<Text>,
        ap_id -> Varchar,
        local -> Bool,
        language_id -> Int4,
        auth_sign -> Nullable<Text>,
        srv_sign -> Nullable<Text>,
        tx -> Nullable<Text>,
        //extras -> Nullable<Jsonb>,
    }
}

table! {
    person_post_aggregates (id) {
        id -> Uuid,
        person_id -> Uuid,
        post_id -> Uuid,
        read_comments -> Int8,
        published -> Timestamp,
    }
}

table! {
    post_aggregates (id) {
        id -> Uuid,
        post_id -> Uuid,
        comments -> Int8,
        score -> Int8,
        upvotes -> Int8,
        downvotes -> Int8,
        stickied -> Bool,
        published -> Timestamp,
        newest_comment_time_necro -> Timestamp,
        newest_comment_time -> Timestamp,
    }
}

table! {
    post_like (id) {
        id -> Uuid,
        post_id -> Uuid,
        person_id -> Uuid,
        score -> Int2,
        published -> Timestamp,
    }
}

table! {
    post_read (id) {
        id -> Uuid,
        post_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

table! {
    post_report (id) {
        id -> Uuid,
        creator_id -> Uuid,
        post_id -> Uuid,
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

table! {
    post_saved (id) {
        id -> Uuid,
        post_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

table! {
    private_message (id) {
        id -> Uuid,
        creator_id -> Uuid,
        recipient_id -> Uuid,
        content -> Text,
        deleted -> Bool,
        read -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        ap_id -> Varchar,
        local -> Bool,
        secured -> Nullable<Text>,
        auth_sign -> Nullable<Text>,
        srv_sign -> Nullable<Text>,
        tx -> Nullable<Text>,
        //extras -> Nullable<Jsonb>,
    }
}

table! {
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

table! {
    site (id) {
        id -> Uuid,
        name -> Varchar,
        sidebar -> Nullable<Text>,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        icon -> Nullable<Varchar>,
        banner -> Nullable<Varchar>,
        description -> Nullable<Text>,
        actor_id -> Text,
        last_refreshed_at -> Timestamp,
        inbox_url -> Text,
        private_key -> Nullable<Text>,
        public_key -> Text,
        instance_id -> Uuid,
        srv_sign -> Nullable<Text>,
        tx -> Nullable<Text>,
        //extras -> Nullable<Jsonb>,
    }
}

table! {
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

table! {
    person_block (id) {
        id -> Uuid,
        person_id -> Uuid,
        target_id -> Uuid,
        published -> Timestamp,
    }
}

table! {
    community_block (id) {
        id -> Uuid,
        person_id -> Uuid,
        community_id -> Uuid,
        published -> Timestamp,
    }
}

table! {
  secret(id) {
    id -> Uuid,
    jwt_secret -> Varchar,
  }
}

table! {
  admin_purge_comment (id) {
    id -> Uuid,
    admin_person_id -> Uuid,
    post_id -> Uuid,
    reason -> Nullable<Text>,
    when_ -> Timestamp,
  }
}

table! {
  email_verification (id) {
    id -> Uuid,
    local_user_id -> Uuid,
    email -> Text,
    verification_token -> Varchar,
    published -> Timestamp,
  }
}

table! {
  admin_purge_community (id) {
    id -> Uuid,
    admin_person_id -> Uuid,
    reason -> Nullable<Text>,
    when_ -> Timestamp,
  }
}

table! {
  admin_purge_person (id) {
    id -> Uuid,
    admin_person_id -> Uuid,
    reason -> Nullable<Text>,
    when_ -> Timestamp,
  }
}

table! {
  admin_purge_post (id) {
    id -> Uuid,
    admin_person_id -> Uuid,
    community_id -> Uuid,
    reason -> Nullable<Text>,
    when_ -> Timestamp,
  }
}

table! {
    registration_application (id) {
        id -> Uuid,
        local_user_id -> Uuid,
        answer -> Text,
        admin_id -> Nullable<Uuid>,
        deny_reason -> Nullable<Text>,
        published -> Timestamp,
    }
}

table! {
    mod_hide_community (id) {
        id -> Uuid,
        community_id -> Uuid,
        mod_person_id -> Uuid,
        reason -> Nullable<Text>,
        hidden -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    language (id) {
        id -> Int4,
        code -> Text,
        name -> Text,
    }
}


table! {
    local_user_language(id) {
        id -> Uuid,
        local_user_id -> Uuid,
        language_id -> Int4,
    }
}

table! {
    site_language(id) {
        id -> Uuid,
        site_id -> Uuid,
        language_id -> Int4,
    }
}

table! {
    community_language(id) {
        id -> Uuid,
        community_id -> Uuid,
        language_id -> Int4,
    }
}

table! {
  instance(id) {
    id -> Uuid,
    domain -> Text,
    published -> Timestamp,
    updated -> Nullable<Timestamp>,
  }
}

table! {
  federation_allowlist(id) {
    id -> Uuid,
    instance_id -> Uuid,
    published -> Timestamp,
    updated -> Nullable<Timestamp>,
  }
}

table! {
  federation_blocklist(id) {
    id -> Uuid,
    instance_id -> Uuid,
    published -> Timestamp,
    updated -> Nullable<Timestamp>,
  }
}

table! {
  local_site(id) {
    id -> Uuid,
    site_id -> Uuid,
    site_setup -> Bool,
    enable_downvotes -> Bool,
    open_registration -> Bool,
    enable_nsfw -> Bool,
    community_creation_admin_only -> Bool,
    require_email_verification -> Bool,
    require_application -> Bool,
    application_question -> Nullable<Text>,
    private_instance -> Bool,
    default_theme -> Text,
    default_post_listing_type -> Text,
    legal_information -> Nullable<Text>,
    hide_modlog_mod_names -> Bool,
    application_email_admins -> Bool,
    slur_filter_regex -> Nullable<Text>,
    actor_name_max_length -> Int4,
    federation_enabled -> Bool,
    federation_debug -> Bool,
    federation_worker_count -> Int4,
    captcha_enabled -> Bool,
    captcha_difficulty -> Text,
    published -> Timestamp,
    updated -> Nullable<Timestamp>,
  }
}

table! {
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

table! {
  tagline(id) {
    id -> Uuid,
    local_site_id -> Uuid,
    content -> Text,
    published -> Timestamp,
    updated -> Nullable<Timestamp>,
  }
}

table! {
    person_follower (id) {
        id -> Uuid,
        person_id -> Uuid,
        follower_id -> Uuid,
        published -> Timestamp,
        pending -> Bool,
    }
}

table! {
    pipayment (id) {
        id -> Uuid,
        domain -> Nullable<Text>, 
        instance_id -> Nullable<Uuid>, // WePi user's id
        person_id -> Nullable<Uuid>, // WePi user's id
        testnet -> Bool,
        published -> Timestamp,
        object_cat -> Nullable<Text>,  // register - page - note - message - person - instance - group
        object_id -> Nullable<Uuid>,    // Post id - comment id, chat message id, site id, instance id, person id, community id 
        
        pi_username -> Text,        // UserDTO - username
        pi_uid -> Nullable<Uuid>,   // UserDTO - uid
        finished -> Bool,
        updated -> Nullable<Timestamp>,
        other_id -> Nullable<Uuid>,    // Captchar id

        identifier -> Text,         // PaymentDto - identifier
        user_uid -> Text,           // PaymentDto - user_uid
        amount -> Double,
        memo -> Text,
        to_address -> Text,
        created_at -> Nullable<Timestamp>,
        approved -> Bool,
        verified -> Bool,
        completed -> Bool,
        cancelled -> Bool,
        user_cancelled -> Bool,
        tx_verified -> Bool,
        tx_link -> Text,
        tx_id -> Text,
        metadata -> Nullable<Jsonb>,
        extras -> Nullable<Jsonb>,
        notes -> Nullable<Text>, 
       
    }
}

// table! {
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

joinable!(person_block -> person (person_id));

joinable!(comment -> person (creator_id));
joinable!(comment -> post (post_id));
joinable!(comment_aggregates -> comment (comment_id));
joinable!(comment_like -> comment (comment_id));
joinable!(comment_like -> person (person_id));
joinable!(comment_like -> post (post_id));
joinable!(comment_report -> comment (comment_id));
joinable!(comment_saved -> comment (comment_id));
joinable!(comment_saved -> person (person_id));
joinable!(community_aggregates -> community (community_id));
joinable!(community_block -> community (community_id));
joinable!(community_block -> person (person_id));
joinable!(community_follower -> community (community_id));
joinable!(community_follower -> person (person_id));
joinable!(community_moderator -> community (community_id));
joinable!(community_moderator -> person (person_id));
joinable!(community_person_ban -> community (community_id));
joinable!(community_person_ban -> person (person_id));
joinable!(local_user -> person (person_id));
joinable!(mod_add_community -> community (community_id));
joinable!(mod_transfer_community -> community (community_id));
joinable!(mod_ban_from_community -> community (community_id));
joinable!(mod_lock_post -> person (mod_person_id));
joinable!(mod_lock_post -> post (post_id));
joinable!(mod_remove_comment -> comment (comment_id));
joinable!(mod_remove_comment -> person (mod_person_id));
joinable!(mod_remove_community -> community (community_id));
joinable!(mod_remove_community -> person (mod_person_id));
joinable!(mod_remove_post -> person (mod_person_id));
joinable!(mod_remove_post -> post (post_id));
joinable!(mod_sticky_post -> person (mod_person_id));
joinable!(mod_sticky_post -> post (post_id));
joinable!(password_reset_request -> local_user (local_user_id));
joinable!(person_aggregates -> person (person_id));
joinable!(person_ban -> person (person_id));
joinable!(person_mention -> comment (comment_id));
joinable!(person_mention -> person (recipient_id));
joinable!(comment_reply -> comment (comment_id));
joinable!(comment_reply -> person (recipient_id));
joinable!(post -> community (community_id));
joinable!(post -> person (creator_id));
joinable!(person_post_aggregates -> post (post_id));
joinable!(person_post_aggregates -> person (person_id));
joinable!(post_aggregates -> post (post_id));
joinable!(post_like -> person (person_id));
joinable!(post_like -> post (post_id));
joinable!(post_read -> person (person_id));
joinable!(post_read -> post (post_id));
joinable!(post_report -> post (post_id));
joinable!(post_saved -> person (person_id));
joinable!(post_saved -> post (post_id));
joinable!(site_aggregates -> site (site_id));
joinable!(email_verification -> local_user (local_user_id));
joinable!(registration_application -> local_user (local_user_id));
joinable!(registration_application -> person (admin_id));
joinable!(mod_hide_community -> person (mod_person_id));
joinable!(mod_hide_community -> community (community_id));
joinable!(post -> language (language_id));
joinable!(comment -> language (language_id));
joinable!(local_user_language -> language (language_id));
joinable!(local_user_language -> local_user (local_user_id));
joinable!(private_message_report -> private_message (private_message_id));
joinable!(site_language -> language (language_id));
joinable!(site_language -> site (site_id));
joinable!(community_language -> language (language_id));
joinable!(community_language -> community (community_id));
joinable!(person_follower -> person (follower_id));

joinable!(admin_purge_comment -> person (admin_person_id));
joinable!(admin_purge_comment -> post (post_id));
joinable!(admin_purge_community -> person (admin_person_id));
joinable!(admin_purge_person -> person (admin_person_id));
joinable!(admin_purge_post -> community (community_id));
joinable!(admin_purge_post -> person (admin_person_id));

joinable!(site -> instance (instance_id));
joinable!(person -> instance (instance_id));
joinable!(community -> instance (instance_id));
joinable!(federation_allowlist -> instance (instance_id));
joinable!(federation_blocklist -> instance (instance_id));
joinable!(local_site -> site (site_id));
joinable!(local_site_rate_limit -> local_site (local_site_id));
joinable!(tagline -> local_site (local_site_id));

allow_tables_to_appear_in_same_query!(
  activity,
  comment,
  comment_aggregates,
  community_block,
  comment_like,
  comment_report,
  comment_saved,
  community,
  community_aggregates,
  community_follower,
  community_moderator,
  community_person_ban,
  local_user,
  mod_add,
  mod_add_community,
  mod_transfer_community,
  mod_ban,
  mod_ban_from_community,
  mod_lock_post,
  mod_remove_comment,
  mod_remove_community,
  mod_remove_post,
  mod_sticky_post,
  mod_hide_community,
  password_reset_request,
  person,
  person_aggregates,
  person_ban,
  person_block,
  person_mention,
  person_post_aggregates,
  comment_reply,
  post,
  post_aggregates,
  post_like,
  post_read,
  post_report,
  post_saved,
  private_message,
  private_message_report,
  site,
  site_aggregates,
  admin_purge_comment,
  admin_purge_community,
  admin_purge_person,
  admin_purge_post,
  email_verification,
  registration_application,
  language,
  tagline,
  local_user_language,
  site_language,
  community_language,
  instance,
  federation_allowlist,
  federation_blocklist,
  local_site,
  local_site_rate_limit,
  person_follower,
  pipayment,
);
