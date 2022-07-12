use crate::PerformCrud;
use activitypub_federation::core::signatures::generate_actor_keypair;
use actix_web::web::Data;
use lemmy_api_common::{
  person::{LoginResponse, Register},
  utils::{blocking, honeypot_check, password_length_check, send_verification_email},
};
use lemmy_apub::{
  generate_inbox_url,
  generate_local_apub_endpoint,
  generate_shared_inbox_url,
  EndpointType,
};
use lemmy_db_schema::{
  aggregates::structs::PersonAggregates,
  source::{
    local_user::{LocalUser, LocalUserForm},
    person::{Person, PersonForm},
    registration_application::{RegistrationApplication, RegistrationApplicationForm},
    site::Site,
    community::*,
  },
  traits::{Crud, ApubActor, },  
};
use lemmy_db_views::structs::LocalUserView;
use lemmy_db_views_actor::structs::PersonViewSafe;
use lemmy_utils::{
  claims::Claims,
  error::LemmyError,
  utils::{check_slurs, is_valid_actor_name},
  settings::SETTINGS,
  ConnectionId,
};
use lemmy_websocket::{messages::CheckCaptcha, LemmyContext};
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for Register {
  type Response = LoginResponse;

  #[tracing::instrument(skip(self, context, _websocket_id))]
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<LoginResponse, LemmyError> {
    let data: &Register = self;

    // no email verification, or applications if the site is not setup yet
    let (mut email_verification, mut require_application) = (false, false);

    // Make sure site has open registration
    if let Ok(site) = blocking(context.pool(), Site::read_local_site).await? {
      if !site.open_registration {
        return Err(LemmyError::from_message("registration_closed"));
      }
      email_verification = site.require_email_verification;
      require_application = site.require_application;
    }

    password_length_check(&data.password)?;
    honeypot_check(&data.honeypot)?;

    if email_verification && data.email.is_none() {
      return Err(LemmyError::from_message("email_required"));
    }

    if require_application && data.answer.is_none() {
      return Err(LemmyError::from_message(
        "registration_application_answer_required",
      ));
    }

    // Make sure passwords match
    if data.password != data.password_verify {
      return Err(LemmyError::from_message("passwords_dont_match"));
    }

    // Check if there are admins. False if admins exist
    let no_admins = blocking(context.pool(), move |conn| {
      PersonViewSafe::admins(conn).map(|a| a.is_empty())
    })
    .await??;

    if !no_admins {
      let settings = SETTINGS.to_owned();
      // Uncomment to disable registration
      if !settings.pinetwork.pi_allow_all {
        return Err(LemmyError::from_message("registration_disabled").into());
      }
    }

    // If its not the admin, check the captcha
    if !no_admins && context.settings().captcha.enabled {
      let check = context
        .chat_server()
        .send(CheckCaptcha {
          uuid: data
            .captcha_uuid
            .to_owned()
            .unwrap_or_else(|| "".to_string()),
          answer: data
            .captcha_answer
            .to_owned()
            .unwrap_or_else(|| "".to_string()),
        })
        .await?;
      if !check {
        return Err(LemmyError::from_message("captcha_incorrect"));
      }
    }

    check_slurs(&data.username, &context.settings().slur_regex())?;

    let actor_keypair = generate_actor_keypair()?;
    if !is_valid_actor_name(&data.username, context.settings().actor_name_max_length) {
      return Err(LemmyError::from_message("invalid_username"));
    }
    let actor_id = generate_local_apub_endpoint(
      EndpointType::Person,
      &data.username,
      &context.settings().get_protocol_and_hostname(),
    )?;

    // We have to create both a person, and local_user

    // Register the new person
    let person_form = PersonForm {
      name: data.username.to_owned(),
      actor_id: Some(actor_id.clone()),
      private_key: Some(Some(actor_keypair.private_key)),
      public_key: Some(actor_keypair.public_key),
      inbox_url: Some(generate_inbox_url(&actor_id)?),
      shared_inbox_url: Some(Some(generate_shared_inbox_url(&actor_id)?)),
      admin: Some(no_admins),
      //extra_user_id: None,
      ..PersonForm::default()
    };

    // insert the person
    let inserted_person = blocking(context.pool(), move |conn| {
      Person::create(conn, &person_form)
    })
    .await?
    .map_err(|e| LemmyError::from_error_message(e, "user_already_exists"))?;

    // Create the local user
    let local_user_form = LocalUserForm {
      person_id: Some(inserted_person.id),
      email: Some(data.email.as_deref().map(|s| s.to_owned())),
      password_encrypted: Some(data.password.to_string()),
      show_nsfw: Some(data.show_nsfw),
      email_verified: Some(false),
      ..LocalUserForm::default()
    };

    let inserted_local_user = match blocking(context.pool(), move |conn| {
      LocalUser::register(conn, &local_user_form)
    })
    .await?
    {
      Ok(lu) => lu,
      Err(e) => {
        let err_type = if e.to_string()
          == "duplicate key value violates unique constraint \"local_user_email_key\""
        {
          "email_already_exists"
        } else {
          "user_already_exists"
        };

        // If the local user creation errored, then delete that person
        blocking(context.pool(), move |conn| {
          Person::delete(conn, inserted_person.id)
        })
        .await??;

        return Err(LemmyError::from_error_message(e, err_type));
      }
    };

  // TODO: UUID check
  // Old: Create default main_community and auto join join
  /*
    let main_community_keypair = generate_actor_keypair()?;

    // Create the main community if it doesn't exist
    let main_community = match blocking(context.pool(), move |conn| {
      Community::read_from_name(conn, "main")
    })
    .await?
    {
      Ok(c) => c,
      Err(_e) => {
        let default_community_name = "main";
        let actor_id = generate_local_apub_endpoint(EndpointType::Community, default_community_name, &context.settings().get_protocol_and_hostname())?;
        let community_form = CommunityForm {
          name: default_community_name.to_string(),
          title: "The Default Community".to_string(),
          description: Some("The Default Community".to_string()),
          actor_id: Some(actor_id.to_owned()),
          private_key: Some(Some(main_community_keypair.private_key)),
          public_key: Some(main_community_keypair.public_key),
          followers_url: Some(generate_followers_url(&actor_id)?),
          inbox_url: Some(generate_inbox_url(&actor_id)?),
          shared_inbox_url: Some(Some(generate_shared_inbox_url(&actor_id)?)),
          ..CommunityForm::default()
        };
        blocking(context.pool(), move |conn| {
          Community::create(conn, &community_form)
        })
        .await??
      }
    };

    // Sign them up for main community no matter what
    let community_follower_form = CommunityFollowerForm {
      community_id: main_community.id,
      person_id: inserted_person.id,
      pending: false,
    };

    let follow = move |conn: &'_ _| CommunityFollower::follow(conn, &community_follower_form);
    if blocking(context.pool(), follow).await?.is_err() {
      return Err(LemmyError::from_message("community_follower_already_exists").into());
    };

    // If its an admin, add them as a mod and follower to main
    if no_admins {
      let community_moderator_form = CommunityModeratorForm {
        community_id: main_community.id,
        person_id: inserted_person.id,
    };
*/
    if require_application {
      // Create the registration application
      let form = RegistrationApplicationForm {
        local_user_id: Some(inserted_local_user.id),
        // We already made sure answer was not null above
        answer: data.answer.to_owned(),
        ..RegistrationApplicationForm::default()
      };

      blocking(context.pool(), move |conn| {
        RegistrationApplication::create(conn, &form)
      })
      .await??;
    }

    let mut login_response = LoginResponse {
      jwt: None,
      registration_created: false,
      verify_email_sent: false,
    };

    // Log the user in directly if email verification and application aren't required
    if !require_application && !email_verification {
      login_response.jwt = Some(
        Claims::jwt(
          inserted_local_user.id.0,
          &context.secret().jwt_secret,
          &context.settings().hostname,
        )?
        .into(),
      );
    } else {
      if email_verification {
        let local_user_view = LocalUserView {
          local_user: inserted_local_user,
          person: inserted_person,
          counts: PersonAggregates::default(),
        };
        // we check at the beginning of this method that email is set
        let email = local_user_view
          .local_user
          .email
          .clone()
          .expect("email was provided");
        send_verification_email(&local_user_view, &email, context.pool(), context.settings())
          .await?;
        login_response.verify_email_sent = true;
      }

      if require_application {
        login_response.registration_created = true;
      }
    }

    Ok(login_response)
  }
}
