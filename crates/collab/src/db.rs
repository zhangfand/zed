use crate::{Error, Result};
use anyhow::{anyhow, Context};
use axum::http::StatusCode;
use collections::HashMap;
use futures::{future::BoxFuture, FutureExt, StreamExt};
use rpc::{proto, ConnectionId};
use serde::{Deserialize, Serialize};
pub use sqlx::postgres::PgPoolOptions as DbOptions;
use sqlx::{
    migrate::{Migrate as _, Migration, MigrationSource},
    types::Uuid,
    FromRow, Postgres, Transaction,
};
use std::{path::Path, time::Duration};
use time::OffsetDateTime;

pub trait Db: Send + Sync {
    fn create_user<'a>(
        &'a self,
        email_address: &'a str,
        admin: bool,
        params: NewUserParams,
    ) -> BoxFuture<'a, Result<NewUserResult>>;
    fn get_all_users<'a>(&'a self, page: u32, limit: u32) -> BoxFuture<'a, Result<Vec<User>>>;
    fn fuzzy_search_users<'a>(
        &'a self,
        query: &'a str,
        limit: u32,
    ) -> BoxFuture<'a, Result<Vec<User>>>;
    fn get_user_by_id<'a>(&'a self, id: UserId) -> BoxFuture<'a, Result<Option<User>>>;
    fn get_user_metrics_id<'a>(&'a self, id: UserId) -> BoxFuture<'a, Result<String>>;
    fn get_users_by_ids<'a>(&'a self, ids: Vec<UserId>) -> BoxFuture<'a, Result<Vec<User>>>;
    fn get_users_with_no_invites<'a>(
        &'a self,
        invited_by_another_user: bool,
    ) -> BoxFuture<'a, Result<Vec<User>>>;
    fn get_user_by_github_account<'a>(
        &'a self,
        github_login: &'a str,
        github_user_id: Option<i32>,
    ) -> BoxFuture<'a, Result<Option<User>>>;
    fn set_user_is_admin<'a>(&'a self, id: UserId, is_admin: bool) -> BoxFuture<'a, Result<()>>;
    fn set_user_connected_once<'a>(
        &'a self,
        id: UserId,
        connected_once: bool,
    ) -> BoxFuture<'a, Result<()>>;
    fn destroy_user<'a>(&'a self, id: UserId) -> BoxFuture<'a, Result<()>>;

    fn set_invite_count_for_user<'a>(&'a self, id: UserId, count: u32)
        -> BoxFuture<'a, Result<()>>;
    fn get_invite_code_for_user<'a>(
        &'a self,
        id: UserId,
    ) -> BoxFuture<'a, Result<Option<(String, u32)>>>;
    fn get_user_for_invite_code<'a>(&'a self, code: &'a str) -> BoxFuture<'a, Result<User>>;
    fn create_invite_from_code<'a>(
        &'a self,
        code: &'a str,
        email_address: &'a str,
        device_id: Option<&'a str>,
    ) -> BoxFuture<'a, Result<Invite>>;

    fn create_signup<'a>(&'a self, signup: Signup) -> BoxFuture<'a, Result<()>>;
    fn get_waitlist_summary<'a>(&'a self) -> BoxFuture<'a, Result<WaitlistSummary>>;
    fn get_unsent_invites<'a>(&'a self, count: usize) -> BoxFuture<'a, Result<Vec<Invite>>>;
    fn record_sent_invites<'a>(&'a self, invites: &'a [Invite]) -> BoxFuture<'a, Result<()>>;
    fn create_user_from_invite<'a>(
        &'a self,
        invite: &'a Invite,
        user: NewUserParams,
    ) -> BoxFuture<'a, Result<Option<NewUserResult>>>;

    fn create_room<'a>(
        &'a self,
        user_id: UserId,
        connection_id: ConnectionId,
    ) -> BoxFuture<'a, Result<proto::Room>>;

    fn call<'a>(
        &'a self,
        room_id: RoomId,
        calling_user_id: UserId,
        called_user_id: UserId,
        initial_project_id: Option<ProjectId>,
    ) -> BoxFuture<'a, Result<proto::Room>>;

    /// Registers a new project for the given user.
    fn register_project<'a>(&'a self, host_user_id: UserId) -> BoxFuture<'a, Result<ProjectId>>;

    /// Unregisters a project for the given project id.
    fn unregister_project<'a>(&'a self, project_id: ProjectId) -> BoxFuture<'a, Result<()>>;

    fn get_contacts<'a>(&'a self, id: UserId) -> BoxFuture<'a, Result<Vec<Contact>>>;
    fn has_contact<'a>(
        &'a self,
        user_id_a: UserId,
        user_id_b: UserId,
    ) -> BoxFuture<'a, Result<bool>>;
    fn send_contact_request<'a>(
        &'a self,
        requester_id: UserId,
        responder_id: UserId,
    ) -> BoxFuture<'a, Result<()>>;
    fn remove_contact<'a>(
        &'a self,
        requester_id: UserId,
        responder_id: UserId,
    ) -> BoxFuture<'a, Result<()>>;
    fn dismiss_contact_notification<'a>(
        &'a self,
        responder_id: UserId,
        requester_id: UserId,
    ) -> BoxFuture<'a, Result<()>>;
    fn respond_to_contact_request<'a>(
        &'a self,
        responder_id: UserId,
        requester_id: UserId,
        accept: bool,
    ) -> BoxFuture<'a, Result<()>>;

    fn create_access_token_hash<'a>(
        &'a self,
        user_id: UserId,
        access_token_hash: &'a str,
        max_access_token_count: usize,
    ) -> BoxFuture<'a, Result<()>>;
    fn get_access_token_hashes<'a>(&'a self, user_id: UserId)
        -> BoxFuture<'a, Result<Vec<String>>>;

    #[cfg(any(test, feature = "seed-support"))]
    fn find_org_by_slug<'a>(&'a self, slug: &'a str) -> BoxFuture<'a, Result<Option<Org>>>;
    #[cfg(any(test, feature = "seed-support"))]
    fn create_org<'a>(&'a self, name: &'a str, slug: &'a str) -> BoxFuture<'a, Result<OrgId>>;
    #[cfg(any(test, feature = "seed-support"))]
    fn add_org_member<'a>(
        &'a self,
        org_id: OrgId,
        user_id: UserId,
        is_admin: bool,
    ) -> BoxFuture<'a, Result<()>>;
    #[cfg(any(test, feature = "seed-support"))]
    fn create_org_channel<'a>(
        &'a self,
        org_id: OrgId,
        name: &'a str,
    ) -> BoxFuture<'a, Result<ChannelId>>;
    #[cfg(any(test, feature = "seed-support"))]

    fn get_org_channels<'a>(&'a self, org_id: OrgId) -> BoxFuture<'a, Result<Vec<Channel>>>;
    fn get_accessible_channels<'a>(
        &'a self,
        user_id: UserId,
    ) -> BoxFuture<'a, Result<Vec<Channel>>>;
    fn can_user_access_channel<'a>(
        &'a self,
        user_id: UserId,
        channel_id: ChannelId,
    ) -> BoxFuture<'a, Result<bool>>;

    #[cfg(any(test, feature = "seed-support"))]
    fn add_channel_member<'a>(
        &'a self,
        channel_id: ChannelId,
        user_id: UserId,
        is_admin: bool,
    ) -> BoxFuture<'a, Result<()>>;
    fn create_channel_message<'a>(
        &'a self,
        channel_id: ChannelId,
        sender_id: UserId,
        body: &'a str,
        timestamp: OffsetDateTime,
        nonce: u128,
    ) -> BoxFuture<'a, Result<MessageId>>;
    fn get_channel_messages<'a>(
        &'a self,
        channel_id: ChannelId,
        count: usize,
        before_id: Option<MessageId>,
    ) -> BoxFuture<'a, Result<Vec<ChannelMessage>>>;

    #[cfg(test)]
    fn teardown<'a>(&'a self, url: &'a str) -> futures::future::BoxFuture<'a, ()>;

    #[cfg(test)]
    fn as_fake(&self) -> Option<&FakeDb>;
}

#[cfg(any(test, debug_assertions))]
pub const DEFAULT_MIGRATIONS_PATH: Option<&'static str> =
    Some(concat!(env!("CARGO_MANIFEST_DIR"), "/migrations"));

#[cfg(not(any(test, debug_assertions)))]
pub const DEFAULT_MIGRATIONS_PATH: Option<&'static str> = None;

pub struct PostgresDb {
    pool: sqlx::PgPool,
}

impl PostgresDb {
    pub async fn new(url: &str, max_connections: u32) -> Result<Self> {
        let pool = DbOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await
            .context("failed to connect to postgres database")?;
        Ok(Self { pool })
    }

    pub async fn migrate(
        &self,
        migrations_path: &Path,
        ignore_checksum_mismatch: bool,
    ) -> anyhow::Result<Vec<(Migration, Duration)>> {
        let migrations = MigrationSource::resolve(migrations_path)
            .await
            .map_err(|err| anyhow!("failed to load migrations: {err:?}"))?;

        let mut conn = self.pool.acquire().await?;

        conn.ensure_migrations_table().await?;
        let applied_migrations: HashMap<_, _> = conn
            .list_applied_migrations()
            .await?
            .into_iter()
            .map(|m| (m.version, m))
            .collect();

        let mut new_migrations = Vec::new();
        for migration in migrations {
            match applied_migrations.get(&migration.version) {
                Some(applied_migration) => {
                    if migration.checksum != applied_migration.checksum && !ignore_checksum_mismatch
                    {
                        Err(anyhow!(
                            "checksum mismatch for applied migration {}",
                            migration.description
                        ))?;
                    }
                }
                None => {
                    let elapsed = conn.apply(&migration).await?;
                    new_migrations.push((migration, elapsed));
                }
            }
        }

        Ok(new_migrations)
    }

    pub fn fuzzy_like_string(string: &str) -> String {
        let mut result = String::with_capacity(string.len() * 2 + 1);
        for c in string.chars() {
            if c.is_alphanumeric() {
                result.push('%');
                result.push(c);
            }
        }
        result.push('%');
        result
    }

    async fn commit_room_transaction(
        &self,
        room_id: RoomId,
        mut tx: Transaction<'static, Postgres>,
    ) -> Result<proto::Room> {
        sqlx::query(
            "
            UPDATE rooms
            SET version = version + 1
            WHERE id = $1
            ",
        )
        .bind(room_id)
        .execute(&mut tx)
        .await?;

        let room: Room = sqlx::query_as(
            "
            SELECT *
            FROM rooms
            WHERE id = $1
            ",
        )
        .bind(room_id)
        .fetch_one(&mut tx)
        .await?;

        let mut db_participants =
            sqlx::query_as::<_, (UserId, Option<i32>, Option<i32>, Option<i32>)>(
                "
                SELECT user_id, connection_id, location_kind, location_project_id,
                FROM room_participants
                WHERE room_id = $1
                ",
            )
            .bind(room_id)
            .fetch(&mut tx);

        let mut participants = Vec::new();
        let mut pending_participant_user_ids = Vec::new();
        while let Some(participant) = db_participants.next().await {
            let (user_id, connection_id, _location_kind, _location_project_id) = participant?;
            if let Some(connection_id) = connection_id {
                participants.push(proto::Participant {
                    user_id: user_id.to_proto(),
                    peer_id: connection_id as u32,
                    projects: Default::default(),
                    location: Some(proto::ParticipantLocation {
                        variant: Some(proto::participant_location::Variant::External(
                            Default::default(),
                        )),
                    }),
                });
            } else {
                pending_participant_user_ids.push(user_id.to_proto());
            }
        }
        drop(db_participants);

        tx.commit().await?;

        Ok(proto::Room {
            id: room.id.to_proto(),
            version: room.version as u64,
            live_kit_room: room.live_kit_room,
            participants,
            pending_participant_user_ids,
        })
    }
}

impl Db for PostgresDb {
    // users

    fn create_user<'a>(
        &'a self,
        email_address: &'a str,
        admin: bool,
        params: NewUserParams,
    ) -> BoxFuture<'a, Result<NewUserResult>> {
        async move {
            let query = "
                INSERT INTO users (email_address, github_login, github_user_id, admin)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (github_login) DO UPDATE SET github_login = excluded.github_login
                RETURNING id, metrics_id::text
            ";
            let (user_id, metrics_id): (UserId, String) = sqlx::query_as(query)
                .bind(email_address)
                .bind(params.github_login)
                .bind(params.github_user_id)
                .bind(admin)
                .fetch_one(&self.pool)
                .await?;
            Ok(NewUserResult {
                user_id,
                metrics_id,
                signup_device_id: None,
                inviting_user_id: None,
            })
        }
        .boxed()
    }

    fn get_all_users<'a>(&'a self, page: u32, limit: u32) -> BoxFuture<'a, Result<Vec<User>>> {
        async move {
            let query = "SELECT * FROM users ORDER BY github_login ASC LIMIT $1 OFFSET $2";
            Ok(sqlx::query_as(query)
                .bind(limit as i32)
                .bind((page * limit) as i32)
                .fetch_all(&self.pool)
                .await?)
        }
        .boxed()
    }

    fn fuzzy_search_users<'a>(
        &'a self,
        name_query: &'a str,
        limit: u32,
    ) -> BoxFuture<'a, Result<Vec<User>>> {
        async move {
            let like_string = Self::fuzzy_like_string(name_query);
            let query = "
                SELECT users.*
                FROM users
                WHERE github_login ILIKE $1
                ORDER BY github_login <-> $2
                LIMIT $3
            ";
            Ok(sqlx::query_as(query)
                .bind(like_string)
                .bind(name_query)
                .bind(limit as i32)
                .fetch_all(&self.pool)
                .await?)
        }
        .boxed()
    }

    fn get_user_by_id<'a>(&'a self, id: UserId) -> BoxFuture<'a, Result<Option<User>>> {
        async move {
            let users = self.get_users_by_ids(vec![id]).await?;
            Ok(users.into_iter().next())
        }
        .boxed()
    }

    fn get_user_metrics_id<'a>(&'a self, id: UserId) -> BoxFuture<'a, Result<String>> {
        async move {
            let query = "
                SELECT metrics_id::text
                FROM users
                WHERE id = $1
            ";
            Ok(sqlx::query_scalar(query)
                .bind(id)
                .fetch_one(&self.pool)
                .await?)
        }
        .boxed()
    }

    fn get_users_by_ids<'a>(&'a self, ids: Vec<UserId>) -> BoxFuture<'a, Result<Vec<User>>> {
        async move {
            let ids = ids.into_iter().map(|id| id.0).collect::<Vec<_>>();
            let query = "
                SELECT users.*
                FROM users
                WHERE users.id = ANY ($1)
            ";
            Ok(sqlx::query_as(query)
                .bind(&ids)
                .fetch_all(&self.pool)
                .await?)
        }
        .boxed()
    }

    fn get_users_with_no_invites<'a>(
        &'a self,
        invited_by_another_user: bool,
    ) -> BoxFuture<'a, Result<Vec<User>>> {
        async move {
            let query = format!(
                "
                SELECT users.*
                FROM users
                WHERE invite_count = 0
                AND inviter_id IS{} NULL
                ",
                if invited_by_another_user { " NOT" } else { "" }
            );

            Ok(sqlx::query_as(&query).fetch_all(&self.pool).await?)
        }
        .boxed()
    }

    fn get_user_by_github_account<'a>(
        &'a self,
        github_login: &'a str,
        github_user_id: Option<i32>,
    ) -> BoxFuture<'a, Result<Option<User>>> {
        async move {
            if let Some(github_user_id) = github_user_id {
                let mut user = sqlx::query_as::<_, User>(
                    "
                    UPDATE users
                    SET github_login = $1
                    WHERE github_user_id = $2
                    RETURNING *
                    ",
                )
                .bind(github_login)
                .bind(github_user_id)
                .fetch_optional(&self.pool)
                .await?;

                if user.is_none() {
                    user = sqlx::query_as::<_, User>(
                        "
                        UPDATE users
                        SET github_user_id = $1
                        WHERE github_login = $2
                        RETURNING *
                        ",
                    )
                    .bind(github_user_id)
                    .bind(github_login)
                    .fetch_optional(&self.pool)
                    .await?;
                }

                Ok(user)
            } else {
                Ok(sqlx::query_as(
                    "
                    SELECT * FROM users
                    WHERE github_login = $1
                    LIMIT 1
                    ",
                )
                .bind(github_login)
                .fetch_optional(&self.pool)
                .await?)
            }
        }
        .boxed()
    }

    fn set_user_is_admin<'a>(&'a self, id: UserId, is_admin: bool) -> BoxFuture<'a, Result<()>> {
        async move {
            let query = "UPDATE users SET admin = $1 WHERE id = $2";
            Ok(sqlx::query(query)
                .bind(is_admin)
                .bind(id.0)
                .execute(&self.pool)
                .await
                .map(drop)?)
        }
        .boxed()
    }

    fn set_user_connected_once<'a>(
        &'a self,
        id: UserId,
        connected_once: bool,
    ) -> BoxFuture<'a, Result<()>> {
        async move {
            let query = "UPDATE users SET connected_once = $1 WHERE id = $2";
            Ok(sqlx::query(query)
                .bind(connected_once)
                .bind(id.0)
                .execute(&self.pool)
                .await
                .map(drop)?)
        }
        .boxed()
    }

    fn destroy_user<'a>(&'a self, id: UserId) -> BoxFuture<'a, Result<()>> {
        async move {
            let query = "DELETE FROM access_tokens WHERE user_id = $1;";
            sqlx::query(query)
                .bind(id.0)
                .execute(&self.pool)
                .await
                .map(drop)?;
            let query = "DELETE FROM users WHERE id = $1;";
            Ok(sqlx::query(query)
                .bind(id.0)
                .execute(&self.pool)
                .await
                .map(drop)?)
        }
        .boxed()
    }

    // signups

    fn create_signup<'a>(&'a self, signup: Signup) -> BoxFuture<'a, Result<()>> {
        async move {
            sqlx::query(
                "
                INSERT INTO signups
                (
                    email_address,
                    email_confirmation_code,
                    email_confirmation_sent,
                    platform_linux,
                    platform_mac,
                    platform_windows,
                    platform_unknown,
                    editor_features,
                    programming_languages,
                    device_id
                )
                VALUES
                    ($1, $2, 'f', $3, $4, $5, 'f', $6, $7, $8)
                RETURNING id
                ",
            )
            .bind(&signup.email_address)
            .bind(&random_email_confirmation_code())
            .bind(&signup.platform_linux)
            .bind(&signup.platform_mac)
            .bind(&signup.platform_windows)
            .bind(&signup.editor_features)
            .bind(&signup.programming_languages)
            .bind(&signup.device_id)
            .execute(&self.pool)
            .await?;
            Ok(())
        }
        .boxed()
    }

    fn get_waitlist_summary<'a>(&'a self) -> BoxFuture<'a, Result<WaitlistSummary>> {
        async move {
            Ok(sqlx::query_as(
                "
                SELECT
                    COUNT(*) as count,
                    COALESCE(SUM(CASE WHEN platform_linux THEN 1 ELSE 0 END), 0) as linux_count,
                    COALESCE(SUM(CASE WHEN platform_mac THEN 1 ELSE 0 END), 0) as mac_count,
                    COALESCE(SUM(CASE WHEN platform_windows THEN 1 ELSE 0 END), 0) as windows_count,
                    COALESCE(SUM(CASE WHEN platform_unknown THEN 1 ELSE 0 END), 0) as unknown_count
                FROM (
                    SELECT *
                    FROM signups
                    WHERE
                        NOT email_confirmation_sent
                ) AS unsent
                ",
            )
            .fetch_one(&self.pool)
            .await?)
        }
        .boxed()
    }

    fn get_unsent_invites<'a>(&'a self, count: usize) -> BoxFuture<'a, Result<Vec<Invite>>> {
        async move {
            Ok(sqlx::query_as(
                "
                SELECT
                    email_address, email_confirmation_code
                FROM signups
                WHERE
                    NOT email_confirmation_sent AND
                    (platform_mac OR platform_unknown)
                LIMIT $1
                ",
            )
            .bind(count as i32)
            .fetch_all(&self.pool)
            .await?)
        }
        .boxed()
    }

    fn record_sent_invites<'a>(&'a self, invites: &'a [Invite]) -> BoxFuture<'a, Result<()>> {
        async move {
            sqlx::query(
                "
                UPDATE signups
                SET email_confirmation_sent = 't'
                WHERE email_address = ANY ($1)
                ",
            )
            .bind(
                &invites
                    .iter()
                    .map(|s| s.email_address.as_str())
                    .collect::<Vec<_>>(),
            )
            .execute(&self.pool)
            .await?;
            Ok(())
        }
        .boxed()
    }

    fn create_user_from_invite<'a>(
        &'a self,
        invite: &'a Invite,
        user: NewUserParams,
    ) -> BoxFuture<'a, Result<Option<NewUserResult>>> {
        async move {
            let mut tx = self.pool.begin().await?;

            let (signup_id, existing_user_id, inviting_user_id, signup_device_id): (
                i32,
                Option<UserId>,
                Option<UserId>,
                Option<String>,
            ) = sqlx::query_as(
                "
                SELECT id, user_id, inviting_user_id, device_id
                FROM signups
                WHERE
                    email_address = $1 AND
                    email_confirmation_code = $2
                ",
            )
            .bind(&invite.email_address)
            .bind(&invite.email_confirmation_code)
            .fetch_optional(&mut tx)
            .await?
            .ok_or_else(|| Error::Http(StatusCode::NOT_FOUND, "no such invite".to_string()))?;

            if existing_user_id.is_some() {
                return Ok(None);
            }

            let (user_id, metrics_id): (UserId, String) = sqlx::query_as(
                "
                INSERT INTO users
                (email_address, github_login, github_user_id, admin, invite_count, invite_code)
                VALUES
                ($1, $2, $3, 'f', $4, $5)
                ON CONFLICT (github_login) DO UPDATE SET
                    email_address = excluded.email_address,
                    github_user_id = excluded.github_user_id,
                    admin = excluded.admin
                RETURNING id, metrics_id::text
                ",
            )
            .bind(&invite.email_address)
            .bind(&user.github_login)
            .bind(&user.github_user_id)
            .bind(&user.invite_count)
            .bind(random_invite_code())
            .fetch_one(&mut tx)
            .await?;

            sqlx::query(
                "
                UPDATE signups
                SET user_id = $1
                WHERE id = $2
                ",
            )
            .bind(&user_id)
            .bind(&signup_id)
            .execute(&mut tx)
            .await?;

            if let Some(inviting_user_id) = inviting_user_id {
                let id: Option<UserId> = sqlx::query_scalar(
                    "
                    UPDATE users
                    SET invite_count = invite_count - 1
                    WHERE id = $1 AND invite_count > 0
                    RETURNING id
                    ",
                )
                .bind(&inviting_user_id)
                .fetch_optional(&mut tx)
                .await?;

                if id.is_none() {
                    Err(Error::Http(
                        StatusCode::UNAUTHORIZED,
                        "no invites remaining".to_string(),
                    ))?;
                }

                sqlx::query(
                    "
                    INSERT INTO contacts
                        (user_id_a, user_id_b, a_to_b, should_notify, accepted)
                    VALUES
                        ($1, $2, 't', 't', 't')
                    ON CONFLICT DO NOTHING
                    ",
                )
                .bind(inviting_user_id)
                .bind(user_id)
                .execute(&mut tx)
                .await?;
            }

            tx.commit().await?;
            Ok(Some(NewUserResult {
                user_id,
                metrics_id,
                inviting_user_id,
                signup_device_id,
            }))
        }
        .boxed()
    }

    // invite codes

    fn set_invite_count_for_user<'a>(
        &'a self,
        id: UserId,
        count: u32,
    ) -> BoxFuture<'a, Result<()>> {
        async move {
            let mut tx = self.pool.begin().await?;
            if count > 0 {
                sqlx::query(
                    "
                    UPDATE users
                    SET invite_code = $1
                    WHERE id = $2 AND invite_code IS NULL
                    ",
                )
                .bind(random_invite_code())
                .bind(id)
                .execute(&mut tx)
                .await?;
            }

            sqlx::query(
                "
                UPDATE users
                SET invite_count = $1
                WHERE id = $2
                ",
            )
            .bind(count as i32)
            .bind(id)
            .execute(&mut tx)
            .await?;
            tx.commit().await?;
            Ok(())
        }
        .boxed()
    }

    fn get_invite_code_for_user<'a>(
        &'a self,
        id: UserId,
    ) -> BoxFuture<'a, Result<Option<(String, u32)>>> {
        async move {
            let result: Option<(String, i32)> = sqlx::query_as(
                "
                SELECT invite_code, invite_count
                FROM users
                WHERE id = $1 AND invite_code IS NOT NULL 
                ",
            )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
            if let Some((code, count)) = result {
                Ok(Some((code, count.try_into().map_err(anyhow::Error::new)?)))
            } else {
                Ok(None)
            }
        }
        .boxed()
    }

    fn get_user_for_invite_code<'a>(&'a self, code: &'a str) -> BoxFuture<'a, Result<User>> {
        async move {
            sqlx::query_as(
                "
                SELECT *
                FROM users
                WHERE invite_code = $1
                ",
            )
            .bind(code)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| {
                Error::Http(
                    StatusCode::NOT_FOUND,
                    "that invite code does not exist".to_string(),
                )
            })
        }
        .boxed()
    }

    fn create_invite_from_code<'a>(
        &'a self,
        code: &'a str,
        email_address: &'a str,
        device_id: Option<&'a str>,
    ) -> BoxFuture<'a, Result<Invite>> {
        async move {
            let mut tx = self.pool.begin().await?;

            let existing_user: Option<UserId> = sqlx::query_scalar(
                "
                SELECT id
                FROM users
                WHERE email_address = $1
                ",
            )
            .bind(email_address)
            .fetch_optional(&mut tx)
            .await?;
            if existing_user.is_some() {
                Err(anyhow!("email address is already in use"))?;
            }

            let row: Option<(UserId, i32)> = sqlx::query_as(
                "
                SELECT id, invite_count
                FROM users
                WHERE invite_code = $1
                ",
            )
            .bind(code)
            .fetch_optional(&mut tx)
            .await?;

            let (inviter_id, invite_count) = match row {
                Some(row) => row,
                None => Err(Error::Http(
                    StatusCode::NOT_FOUND,
                    "invite code not found".to_string(),
                ))?,
            };

            if invite_count == 0 {
                Err(Error::Http(
                    StatusCode::UNAUTHORIZED,
                    "no invites remaining".to_string(),
                ))?;
            }

            let email_confirmation_code: String = sqlx::query_scalar(
                "
                INSERT INTO signups
                (
                    email_address,
                    email_confirmation_code,
                    email_confirmation_sent,
                    inviting_user_id,
                    platform_linux,
                    platform_mac,
                    platform_windows,
                    platform_unknown,
                    device_id
                )
                VALUES
                    ($1, $2, 'f', $3, 'f', 'f', 'f', 't', $4)
                ON CONFLICT (email_address)
                DO UPDATE SET
                    inviting_user_id = excluded.inviting_user_id
                RETURNING email_confirmation_code
                ",
            )
            .bind(&email_address)
            .bind(&random_email_confirmation_code())
            .bind(&inviter_id)
            .bind(&device_id)
            .fetch_one(&mut tx)
            .await?;

            tx.commit().await?;

            Ok(Invite {
                email_address: email_address.into(),
                email_confirmation_code,
            })
        }
        .boxed()
    }

    // rooms

    fn create_room<'a>(
        &'a self,
        user_id: UserId,
        connection_id: ConnectionId,
    ) -> BoxFuture<'a, Result<proto::Room>> {
        async move {
            let mut tx = self.pool.begin().await?;
            let live_kit_room = nanoid::nanoid!(30);
            let room_id = sqlx::query_scalar(
                "
                INSERT INTO rooms (live_kit_room, version)
                VALUES ($1, 0)
                ",
            )
            .bind(&live_kit_room)
            .fetch_one(&mut tx)
            .await
            .map(RoomId)?;

            sqlx::query(
                "
                INSERT INTO room_participants (room_id, user_id, connection_id)
                VALUES ($1, $2, $3)
                ",
            )
            .bind(room_id)
            .bind(user_id)
            .bind(connection_id.0 as i32)
            .execute(&mut tx)
            .await?;

            sqlx::query(
                "
                INSERT INTO calls (room_id, calling_user_id, called_user_id, answering_connection_id)
                VALUES ($1, $2, $3, $4)
                ",
            )
            .bind(room_id)
            .bind(user_id)
            .bind(user_id)
            .bind(connection_id.0 as i32)
            .execute(&mut tx)
            .await?;

            self.commit_room_transaction(room_id, tx).await
        }
        .boxed()
    }

    fn call<'a>(
        &'a self,
        room_id: RoomId,
        calling_user_id: UserId,
        called_user_id: UserId,
        initial_project_id: Option<ProjectId>,
    ) -> BoxFuture<'a, Result<proto::Room>> {
        async move {
            let mut tx = self.pool.begin().await?;
            sqlx::query(
                "
                INSERT INTO calls (room_id, calling_user_id, called_user_id, initial_project_id)
                VALUES ($1, $2, $3, $4)
                ",
            )
            .bind(room_id)
            .bind(calling_user_id)
            .bind(called_user_id)
            .bind(initial_project_id)
            .execute(&mut tx)
            .await?;

            sqlx::query(
                "
                INSERT INTO room_participants (room_id, user_id)
                VALUES ($1, $2)
                ",
            )
            .bind(room_id)
            .bind(called_user_id)
            .execute(&mut tx)
            .await?;

            self.commit_room_transaction(room_id, tx).await
        }
        .boxed()
    }

    // projects

    fn register_project<'a>(&'a self, host_user_id: UserId) -> BoxFuture<'a, Result<ProjectId>> {
        async move {
            Ok(sqlx::query_scalar(
                "
                INSERT INTO projects(host_user_id)
                VALUES ($1)
                RETURNING id
                ",
            )
            .bind(host_user_id)
            .fetch_one(&self.pool)
            .await
            .map(ProjectId)?)
        }
        .boxed()
    }

    fn unregister_project<'a>(&'a self, project_id: ProjectId) -> BoxFuture<'a, Result<()>> {
        async move {
            sqlx::query(
                "
                UPDATE projects
                SET unregistered = 't'
                WHERE id = $1
                ",
            )
            .bind(project_id)
            .execute(&self.pool)
            .await?;
            Ok(())
        }
        .boxed()
    }

    // contacts

    fn get_contacts<'a>(&'a self, user_id: UserId) -> BoxFuture<'a, Result<Vec<Contact>>> {
        async move {
            let query = "
                SELECT user_id_a, user_id_b, a_to_b, accepted, should_notify
                FROM contacts
                WHERE user_id_a = $1 OR user_id_b = $1;
            ";

            let mut rows = sqlx::query_as::<_, (UserId, UserId, bool, bool, bool)>(query)
                .bind(user_id)
                .fetch(&self.pool);

            let mut contacts = Vec::new();
            while let Some(row) = rows.next().await {
                let (user_id_a, user_id_b, a_to_b, accepted, should_notify) = row?;

                if user_id_a == user_id {
                    if accepted {
                        contacts.push(Contact::Accepted {
                            user_id: user_id_b,
                            should_notify: should_notify && a_to_b,
                        });
                    } else if a_to_b {
                        contacts.push(Contact::Outgoing { user_id: user_id_b })
                    } else {
                        contacts.push(Contact::Incoming {
                            user_id: user_id_b,
                            should_notify,
                        });
                    }
                } else if accepted {
                    contacts.push(Contact::Accepted {
                        user_id: user_id_a,
                        should_notify: should_notify && !a_to_b,
                    });
                } else if a_to_b {
                    contacts.push(Contact::Incoming {
                        user_id: user_id_a,
                        should_notify,
                    });
                } else {
                    contacts.push(Contact::Outgoing { user_id: user_id_a });
                }
            }

            contacts.sort_unstable_by_key(|contact| contact.user_id());

            Ok(contacts)
        }
        .boxed()
    }

    fn has_contact<'a>(
        &'a self,
        user_id_1: UserId,
        user_id_2: UserId,
    ) -> BoxFuture<'a, Result<bool>> {
        async move {
            let (id_a, id_b) = if user_id_1 < user_id_2 {
                (user_id_1, user_id_2)
            } else {
                (user_id_2, user_id_1)
            };

            let query = "
                SELECT 1 FROM contacts
                WHERE user_id_a = $1 AND user_id_b = $2 AND accepted = 't'
                LIMIT 1
            ";
            Ok(sqlx::query_scalar::<_, i32>(query)
                .bind(id_a.0)
                .bind(id_b.0)
                .fetch_optional(&self.pool)
                .await?
                .is_some())
        }
        .boxed()
    }

    fn send_contact_request<'a>(
        &'a self,
        sender_id: UserId,
        receiver_id: UserId,
    ) -> BoxFuture<'a, Result<()>> {
        async move {
            let (id_a, id_b, a_to_b) = if sender_id < receiver_id {
                (sender_id, receiver_id, true)
            } else {
                (receiver_id, sender_id, false)
            };
            let query = "
                INSERT into contacts (user_id_a, user_id_b, a_to_b, accepted, should_notify)
                VALUES ($1, $2, $3, 'f', 't')
                ON CONFLICT (user_id_a, user_id_b) DO UPDATE
                SET
                    accepted = 't',
                    should_notify = 'f'
                WHERE
                    NOT contacts.accepted AND
                    ((contacts.a_to_b = excluded.a_to_b AND contacts.user_id_a = excluded.user_id_b) OR
                    (contacts.a_to_b != excluded.a_to_b AND contacts.user_id_a = excluded.user_id_a));
            ";
            let result = sqlx::query(query)
                .bind(id_a.0)
                .bind(id_b.0)
                .bind(a_to_b)
                .execute(&self.pool)
                .await?;

            if result.rows_affected() == 1 {
                Ok(())
            } else {
                Err(anyhow!("contact already requested"))?
            }
        }
        .boxed()
    }

    fn remove_contact<'a>(
        &'a self,
        requester_id: UserId,
        responder_id: UserId,
    ) -> BoxFuture<'a, Result<()>> {
        async move {
            let (id_a, id_b) = if responder_id < requester_id {
                (responder_id, requester_id)
            } else {
                (requester_id, responder_id)
            };
            let query = "
                DELETE FROM contacts
                WHERE user_id_a = $1 AND user_id_b = $2;
            ";
            let result = sqlx::query(query)
                .bind(id_a.0)
                .bind(id_b.0)
                .execute(&self.pool)
                .await?;

            if result.rows_affected() == 1 {
                Ok(())
            } else {
                Err(anyhow!("no such contact"))?
            }
        }
        .boxed()
    }

    fn dismiss_contact_notification<'a>(
        &'a self,
        user_id: UserId,
        contact_user_id: UserId,
    ) -> BoxFuture<'a, Result<()>> {
        async move {
            let (id_a, id_b, a_to_b) = if user_id < contact_user_id {
                (user_id, contact_user_id, true)
            } else {
                (contact_user_id, user_id, false)
            };

            let query = "
                UPDATE contacts
                SET should_notify = 'f'
                WHERE
                    user_id_a = $1 AND user_id_b = $2 AND
                    (
                        (a_to_b = $3 AND accepted) OR
                        (a_to_b != $3 AND NOT accepted)
                    );
            ";

            let result = sqlx::query(query)
                .bind(id_a.0)
                .bind(id_b.0)
                .bind(a_to_b)
                .execute(&self.pool)
                .await?;

            if result.rows_affected() == 0 {
                Err(anyhow!("no such contact request"))?;
            }

            Ok(())
        }
        .boxed()
    }

    fn respond_to_contact_request<'a>(
        &'a self,
        responder_id: UserId,
        requester_id: UserId,
        accept: bool,
    ) -> BoxFuture<'a, Result<()>> {
        async move {
            let (id_a, id_b, a_to_b) = if responder_id < requester_id {
                (responder_id, requester_id, false)
            } else {
                (requester_id, responder_id, true)
            };
            let result = if accept {
                let query = "
                    UPDATE contacts
                    SET accepted = 't', should_notify = 't'
                    WHERE user_id_a = $1 AND user_id_b = $2 AND a_to_b = $3;
                ";
                sqlx::query(query)
                    .bind(id_a.0)
                    .bind(id_b.0)
                    .bind(a_to_b)
                    .execute(&self.pool)
                    .await?
            } else {
                let query = "
                    DELETE FROM contacts
                    WHERE user_id_a = $1 AND user_id_b = $2 AND a_to_b = $3 AND NOT accepted;
                ";
                sqlx::query(query)
                    .bind(id_a.0)
                    .bind(id_b.0)
                    .bind(a_to_b)
                    .execute(&self.pool)
                    .await?
            };
            if result.rows_affected() == 1 {
                Ok(())
            } else {
                Err(anyhow!("no such contact request"))?
            }
        }
        .boxed()
    }

    // access tokens

    fn create_access_token_hash<'a>(
        &'a self,
        user_id: UserId,
        access_token_hash: &'a str,
        max_access_token_count: usize,
    ) -> BoxFuture<'a, Result<()>> {
        async move {
            let insert_query = "
                INSERT INTO access_tokens (user_id, hash)
                VALUES ($1, $2);
            ";
            let cleanup_query = "
                DELETE FROM access_tokens
                WHERE id IN (
                    SELECT id from access_tokens
                    WHERE user_id = $1
                    ORDER BY id DESC
                    OFFSET $3
                )
            ";

            let mut tx = self.pool.begin().await?;
            sqlx::query(insert_query)
                .bind(user_id.0)
                .bind(access_token_hash)
                .execute(&mut tx)
                .await?;
            sqlx::query(cleanup_query)
                .bind(user_id.0)
                .bind(access_token_hash)
                .bind(max_access_token_count as i32)
                .execute(&mut tx)
                .await?;
            Ok(tx.commit().await?)
        }
        .boxed()
    }

    fn get_access_token_hashes<'a>(
        &'a self,
        user_id: UserId,
    ) -> BoxFuture<'a, Result<Vec<String>>> {
        async move {
            let query = "
                SELECT hash
                FROM access_tokens
                WHERE user_id = $1
                ORDER BY id DESC
            ";
            Ok(sqlx::query_scalar(query)
                .bind(user_id.0)
                .fetch_all(&self.pool)
                .await?)
        }
        .boxed()
    }

    // orgs

    #[allow(unused)] // Help rust-analyzer
    #[cfg(any(test, feature = "seed-support"))]
    fn find_org_by_slug<'a>(&'a self, slug: &'a str) -> BoxFuture<'a, Result<Option<Org>>> {
        async move {
            let query = "
                SELECT *
                FROM orgs
                WHERE slug = $1
            ";
            Ok(sqlx::query_as(query)
                .bind(slug)
                .fetch_optional(&self.pool)
                .await?)
        }
        .boxed()
    }

    #[cfg(any(test, feature = "seed-support"))]
    fn create_org<'a>(&'a self, name: &'a str, slug: &'a str) -> BoxFuture<'a, Result<OrgId>> {
        async move {
            let query = "
                INSERT INTO orgs (name, slug)
                VALUES ($1, $2)
                RETURNING id
            ";
            Ok(sqlx::query_scalar(query)
                .bind(name)
                .bind(slug)
                .fetch_one(&self.pool)
                .await
                .map(OrgId)?)
        }
        .boxed()
    }

    #[cfg(any(test, feature = "seed-support"))]
    fn add_org_member<'a>(
        &'a self,
        org_id: OrgId,
        user_id: UserId,
        is_admin: bool,
    ) -> BoxFuture<'a, Result<()>> {
        async move {
            let query = "
                INSERT INTO org_memberships (org_id, user_id, admin)
                VALUES ($1, $2, $3)
                ON CONFLICT DO NOTHING
            ";
            Ok(sqlx::query(query)
                .bind(org_id.0)
                .bind(user_id.0)
                .bind(is_admin)
                .execute(&self.pool)
                .await
                .map(drop)?)
        }
        .boxed()
    }

    // channels

    #[cfg(any(test, feature = "seed-support"))]
    fn create_org_channel<'a>(
        &'a self,
        org_id: OrgId,
        name: &'a str,
    ) -> BoxFuture<'a, Result<ChannelId>> {
        async move {
            let query = "
                INSERT INTO channels (owner_id, owner_is_user, name)
                VALUES ($1, false, $2)
                RETURNING id
            ";
            Ok(sqlx::query_scalar(query)
                .bind(org_id.0)
                .bind(name)
                .fetch_one(&self.pool)
                .await
                .map(ChannelId)?)
        }
        .boxed()
    }

    #[allow(unused)] // Help rust-analyzer
    #[cfg(any(test, feature = "seed-support"))]
    fn get_org_channels<'a>(&'a self, org_id: OrgId) -> BoxFuture<'a, Result<Vec<Channel>>> {
        async move {
            let query = "
                SELECT *
                FROM channels
                WHERE
                    channels.owner_is_user = false AND
                    channels.owner_id = $1
            ";
            Ok(sqlx::query_as(query)
                .bind(org_id.0)
                .fetch_all(&self.pool)
                .await?)
        }
        .boxed()
    }

    fn get_accessible_channels<'a>(
        &'a self,
        user_id: UserId,
    ) -> BoxFuture<'a, Result<Vec<Channel>>> {
        async move {
            let query = "
                SELECT
                    channels.*
                FROM
                    channel_memberships, channels
                WHERE
                    channel_memberships.user_id = $1 AND
                    channel_memberships.channel_id = channels.id
            ";
            Ok(sqlx::query_as(query)
                .bind(user_id.0)
                .fetch_all(&self.pool)
                .await?)
        }
        .boxed()
    }

    fn can_user_access_channel<'a>(
        &'a self,
        user_id: UserId,
        channel_id: ChannelId,
    ) -> BoxFuture<'a, Result<bool>> {
        async move {
            let query = "
                SELECT id
                FROM channel_memberships
                WHERE user_id = $1 AND channel_id = $2
                LIMIT 1
            ";
            Ok(sqlx::query_scalar::<_, i32>(query)
                .bind(user_id.0)
                .bind(channel_id.0)
                .fetch_optional(&self.pool)
                .await
                .map(|e| e.is_some())?)
        }
        .boxed()
    }

    #[cfg(any(test, feature = "seed-support"))]
    fn add_channel_member<'a>(
        &'a self,
        channel_id: ChannelId,
        user_id: UserId,
        is_admin: bool,
    ) -> BoxFuture<'a, Result<()>> {
        async move {
            let query = "
                INSERT INTO channel_memberships (channel_id, user_id, admin)
                VALUES ($1, $2, $3)
                ON CONFLICT DO NOTHING
            ";
            Ok(sqlx::query(query)
                .bind(channel_id.0)
                .bind(user_id.0)
                .bind(is_admin)
                .execute(&self.pool)
                .await
                .map(drop)?)
        }
        .boxed()
    }

    // messages

    fn create_channel_message<'a>(
        &'a self,
        channel_id: ChannelId,
        sender_id: UserId,
        body: &'a str,
        timestamp: OffsetDateTime,
        nonce: u128,
    ) -> BoxFuture<'a, Result<MessageId>> {
        async move {
            let query = "
                INSERT INTO channel_messages (channel_id, sender_id, body, sent_at, nonce)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (nonce) DO UPDATE SET nonce = excluded.nonce
                RETURNING id
            ";
            Ok(sqlx::query_scalar(query)
                .bind(channel_id.0)
                .bind(sender_id.0)
                .bind(body)
                .bind(timestamp)
                .bind(Uuid::from_u128(nonce))
                .fetch_one(&self.pool)
                .await
                .map(MessageId)?)
        }
        .boxed()
    }

    fn get_channel_messages<'a>(
        &'a self,
        channel_id: ChannelId,
        count: usize,
        before_id: Option<MessageId>,
    ) -> BoxFuture<'a, Result<Vec<ChannelMessage>>> {
        async move {
            let query = r#"
                SELECT * FROM (
                    SELECT
                        id, channel_id, sender_id, body, sent_at AT TIME ZONE 'UTC' as sent_at, nonce
                    FROM
                        channel_messages
                    WHERE
                        channel_id = $1 AND
                        id < $2
                    ORDER BY id DESC
                    LIMIT $3
                ) as recent_messages
                ORDER BY id ASC
            "#;
            Ok(sqlx::query_as(query)
                .bind(channel_id.0)
                .bind(before_id.unwrap_or(MessageId::MAX))
                .bind(count as i64)
                .fetch_all(&self.pool)
                .await?)
        }
        .boxed()
    }

    #[cfg(test)]
    fn teardown<'a>(&'a self, url: &'a str) -> BoxFuture<'a, ()> {
        async move {
            use util::ResultExt;

            let query = "
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = current_database() AND pid <> pg_backend_pid();
            ";
            sqlx::query(query).execute(&self.pool).await.log_err();
            self.pool.close().await;
            <sqlx::Postgres as sqlx::migrate::MigrateDatabase>::drop_database(url)
                .await
                .log_err();
        }
        .boxed()
    }

    #[cfg(test)]
    fn as_fake(&self) -> Option<&FakeDb> {
        None
    }
}

macro_rules! id_type {
    ($name:ident) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            sqlx::Type,
            Serialize,
            Deserialize,
        )]
        #[sqlx(transparent)]
        #[serde(transparent)]
        pub struct $name(pub i32);

        impl $name {
            #[allow(unused)]
            pub const MAX: Self = Self(i32::MAX);

            #[allow(unused)]
            pub fn from_proto(value: u64) -> Self {
                Self(value as i32)
            }

            #[allow(unused)]
            pub fn to_proto(self) -> u64 {
                self.0 as u64
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

id_type!(UserId);
#[derive(Clone, Debug, Default, FromRow, Serialize, PartialEq)]
pub struct User {
    pub id: UserId,
    pub github_login: String,
    pub github_user_id: Option<i32>,
    pub email_address: Option<String>,
    pub admin: bool,
    pub invite_code: Option<String>,
    pub invite_count: i32,
    pub connected_once: bool,
}

id_type!(RoomId);
#[derive(Clone, Debug, Default, FromRow, Serialize, PartialEq)]
pub struct Room {
    pub id: RoomId,
    pub version: i32,
    pub live_kit_room: String,
}

#[derive(Clone, Debug, Default, FromRow, PartialEq)]
pub struct RoomParticipant {
    pub room_id: RoomId,
    pub user_id: UserId,
    pub location_kind: Option<i32>,
    pub location_project_id: Option<ProjectId>,
    pub connection_id: Option<i32>,
}

#[derive(Clone, Debug, Default, FromRow, PartialEq)]
pub struct Call {
    pub room_id: RoomId,
    pub calling_user_id: UserId,
    pub called_user_id: UserId,
    pub answering_connection_id: Option<i32>,
    pub initial_project_id: Option<ProjectId>,
}

id_type!(ProjectId);
#[derive(Clone, Debug, Default, FromRow, Serialize, PartialEq)]
pub struct Project {
    pub id: ProjectId,
    pub host_user_id: UserId,
    pub unregistered: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserActivitySummary {
    pub id: UserId,
    pub github_login: String,
    pub project_activity: Vec<ProjectActivitySummary>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProjectActivitySummary {
    pub id: ProjectId,
    pub duration: Duration,
    pub max_collaborators: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserActivityPeriod {
    pub project_id: ProjectId,
    #[serde(with = "time::serde::iso8601")]
    pub start: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub end: OffsetDateTime,
    pub extensions: HashMap<String, usize>,
}

id_type!(OrgId);
#[derive(FromRow)]
pub struct Org {
    pub id: OrgId,
    pub name: String,
    pub slug: String,
}

id_type!(ChannelId);
#[derive(Clone, Debug, FromRow, Serialize)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
    pub owner_id: i32,
    pub owner_is_user: bool,
}

id_type!(MessageId);
#[derive(Clone, Debug, FromRow)]
pub struct ChannelMessage {
    pub id: MessageId,
    pub channel_id: ChannelId,
    pub sender_id: UserId,
    pub body: String,
    pub sent_at: OffsetDateTime,
    pub nonce: Uuid,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Contact {
    Accepted {
        user_id: UserId,
        should_notify: bool,
    },
    Outgoing {
        user_id: UserId,
    },
    Incoming {
        user_id: UserId,
        should_notify: bool,
    },
}

impl Contact {
    pub fn user_id(&self) -> UserId {
        match self {
            Contact::Accepted { user_id, .. } => *user_id,
            Contact::Outgoing { user_id } => *user_id,
            Contact::Incoming { user_id, .. } => *user_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IncomingContactRequest {
    pub requester_id: UserId,
    pub should_notify: bool,
}

#[derive(Clone, Deserialize)]
pub struct Signup {
    pub email_address: String,
    pub platform_mac: bool,
    pub platform_windows: bool,
    pub platform_linux: bool,
    pub editor_features: Vec<String>,
    pub programming_languages: Vec<String>,
    pub device_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, FromRow)]
pub struct WaitlistSummary {
    #[sqlx(default)]
    pub count: i64,
    #[sqlx(default)]
    pub linux_count: i64,
    #[sqlx(default)]
    pub mac_count: i64,
    #[sqlx(default)]
    pub windows_count: i64,
    #[sqlx(default)]
    pub unknown_count: i64,
}

#[derive(FromRow, PartialEq, Debug, Serialize, Deserialize)]
pub struct Invite {
    pub email_address: String,
    pub email_confirmation_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserParams {
    pub github_login: String,
    pub github_user_id: i32,
    pub invite_count: i32,
}

#[derive(Debug)]
pub struct NewUserResult {
    pub user_id: UserId,
    pub metrics_id: String,
    pub inviting_user_id: Option<UserId>,
    pub signup_device_id: Option<String>,
}

fn random_invite_code() -> String {
    nanoid::nanoid!(16)
}

fn random_email_confirmation_code() -> String {
    nanoid::nanoid!(64)
}

#[cfg(test)]
pub use test::*;

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::anyhow;
    use collections::BTreeMap;
    use futures::FutureExt;
    use gpui::executor::Background;
    use lazy_static::lazy_static;
    use parking_lot::Mutex;
    use rand::prelude::*;
    use sqlx::{migrate::MigrateDatabase, Postgres};
    use std::sync::Arc;
    use util::post_inc;

    pub struct FakeDb {
        background: Arc<Background>,
        pub users: Mutex<BTreeMap<UserId, User>>,
        pub rooms: Mutex<BTreeMap<RoomId, Room>>,
        pub room_participants: Mutex<BTreeMap<RoomId, Vec<RoomParticipant>>>,
        pub calls: Mutex<BTreeMap<UserId, Call>>,
        pub projects: Mutex<BTreeMap<ProjectId, Project>>,
        pub orgs: Mutex<BTreeMap<OrgId, Org>>,
        pub org_memberships: Mutex<BTreeMap<(OrgId, UserId), bool>>,
        pub channels: Mutex<BTreeMap<ChannelId, Channel>>,
        pub channel_memberships: Mutex<BTreeMap<(ChannelId, UserId), bool>>,
        pub channel_messages: Mutex<BTreeMap<MessageId, ChannelMessage>>,
        pub contacts: Mutex<Vec<FakeContact>>,
        next_channel_message_id: Mutex<i32>,
        next_user_id: Mutex<i32>,
        next_org_id: Mutex<i32>,
        next_channel_id: Mutex<i32>,
        next_room_id: Mutex<i32>,
        next_project_id: Mutex<i32>,
    }

    #[derive(Debug)]
    pub struct FakeContact {
        pub requester_id: UserId,
        pub responder_id: UserId,
        pub accepted: bool,
        pub should_notify: bool,
    }

    impl FakeDb {
        pub fn new(background: Arc<Background>) -> Self {
            Self {
                background,
                users: Default::default(),
                next_user_id: Mutex::new(0),
                rooms: Default::default(),
                next_room_id: Mutex::new(0),
                room_participants: Default::default(),
                calls: Default::default(),
                projects: Default::default(),
                next_project_id: Mutex::new(1),
                orgs: Default::default(),
                next_org_id: Mutex::new(1),
                org_memberships: Default::default(),
                channels: Default::default(),
                next_channel_id: Mutex::new(1),
                channel_memberships: Default::default(),
                channel_messages: Default::default(),
                next_channel_message_id: Mutex::new(1),
                contacts: Default::default(),
            }
        }

        fn room_snapshot(&self, room_id: RoomId) -> Result<proto::Room> {
            let mut rooms = self.rooms.lock();
            let room = rooms
                .get_mut(&room_id)
                .ok_or_else(|| anyhow!("no such room"))?;
            room.version += 1;

            let mut participants = Vec::new();
            let mut pending_participant_user_ids = Vec::new();
            for participant in self.room_participants.lock().get(&room_id).unwrap() {
                if let Some(connection_id) = participant.connection_id {
                    participants.push(proto::Participant {
                        user_id: participant.user_id.to_proto(),
                        peer_id: connection_id as u32,
                        projects: Default::default(),
                        location: Some(proto::ParticipantLocation {
                            variant: Some(proto::participant_location::Variant::External(
                                Default::default(),
                            )),
                        }),
                    });
                } else {
                    pending_participant_user_ids.push(participant.user_id.to_proto());
                }
            }

            Ok(proto::Room {
                id: room_id.to_proto(),
                version: room.version as u64,
                live_kit_room: room.live_kit_room.clone(),
                participants,
                pending_participant_user_ids,
            })
        }
    }

    impl Db for FakeDb {
        fn create_user<'a>(
            &'a self,
            email_address: &'a str,
            admin: bool,
            params: NewUserParams,
        ) -> BoxFuture<'a, Result<NewUserResult>> {
            async move {
                self.background.simulate_random_delay().await;

                let mut users = self.users.lock();
                let user_id = if let Some(user) = users
                    .values()
                    .find(|user| user.github_login == params.github_login)
                {
                    user.id
                } else {
                    let id = post_inc(&mut *self.next_user_id.lock());
                    let user_id = UserId(id);
                    users.insert(
                        user_id,
                        User {
                            id: user_id,
                            github_login: params.github_login,
                            github_user_id: Some(params.github_user_id),
                            email_address: Some(email_address.to_string()),
                            admin,
                            invite_code: None,
                            invite_count: 0,
                            connected_once: false,
                        },
                    );
                    user_id
                };
                Ok(NewUserResult {
                    user_id,
                    metrics_id: "the-metrics-id".to_string(),
                    inviting_user_id: None,
                    signup_device_id: None,
                })
            }
            .boxed()
        }

        fn get_all_users<'a>(
            &'a self,
            _page: u32,
            _limit: u32,
        ) -> BoxFuture<'a, Result<Vec<User>>> {
            async move { unimplemented!() }.boxed()
        }

        fn fuzzy_search_users<'a>(
            &'a self,
            _: &'a str,
            _: u32,
        ) -> BoxFuture<'a, Result<Vec<User>>> {
            async move { unimplemented!() }.boxed()
        }

        fn get_user_by_id<'a>(&'a self, id: UserId) -> BoxFuture<'a, Result<Option<User>>> {
            async move {
                self.background.simulate_random_delay().await;
                Ok(self.get_users_by_ids(vec![id]).await?.into_iter().next())
            }
            .boxed()
        }

        fn get_user_metrics_id<'a>(&'a self, _id: UserId) -> BoxFuture<'a, Result<String>> {
            async move { Ok("the-metrics-id".to_string()) }.boxed()
        }

        fn get_users_by_ids<'a>(&'a self, ids: Vec<UserId>) -> BoxFuture<'a, Result<Vec<User>>> {
            async move {
                self.background.simulate_random_delay().await;
                let users = self.users.lock();
                Ok(ids.iter().filter_map(|id| users.get(id).cloned()).collect())
            }
            .boxed()
        }

        fn get_users_with_no_invites<'a>(&'a self, _: bool) -> BoxFuture<'a, Result<Vec<User>>> {
            async move { unimplemented!() }.boxed()
        }

        fn get_user_by_github_account<'a>(
            &'a self,
            github_login: &'a str,
            github_user_id: Option<i32>,
        ) -> BoxFuture<'a, Result<Option<User>>> {
            async move {
                self.background.simulate_random_delay().await;
                if let Some(github_user_id) = github_user_id {
                    for user in self.users.lock().values_mut() {
                        if user.github_user_id == Some(github_user_id) {
                            user.github_login = github_login.into();
                            return Ok(Some(user.clone()));
                        }
                        if user.github_login == github_login {
                            user.github_user_id = Some(github_user_id);
                            return Ok(Some(user.clone()));
                        }
                    }
                    Ok(None)
                } else {
                    Ok(self
                        .users
                        .lock()
                        .values()
                        .find(|user| user.github_login == github_login)
                        .cloned())
                }
            }
            .boxed()
        }

        fn set_user_is_admin<'a>(
            &'a self,
            _id: UserId,
            _is_admin: bool,
        ) -> BoxFuture<'a, Result<()>> {
            async move { unimplemented!() }.boxed()
        }

        fn set_user_connected_once<'a>(
            &'a self,
            id: UserId,
            connected_once: bool,
        ) -> BoxFuture<'a, Result<()>> {
            async move {
                self.background.simulate_random_delay().await;
                let mut users = self.users.lock();
                let mut user = users
                    .get_mut(&id)
                    .ok_or_else(|| anyhow!("user not found"))?;
                user.connected_once = connected_once;
                Ok(())
            }
            .boxed()
        }

        fn destroy_user<'a>(&'a self, _id: UserId) -> BoxFuture<'a, Result<()>> {
            async move { unimplemented!() }.boxed()
        }

        // signups

        fn create_signup<'a>(&'a self, _signup: Signup) -> BoxFuture<'a, Result<()>> {
            async move { unimplemented!() }.boxed()
        }

        fn get_waitlist_summary<'a>(&'a self) -> BoxFuture<'a, Result<WaitlistSummary>> {
            async move { unimplemented!() }.boxed()
        }

        fn get_unsent_invites<'a>(&'a self, _count: usize) -> BoxFuture<'a, Result<Vec<Invite>>> {
            async move { unimplemented!() }.boxed()
        }

        fn record_sent_invites<'a>(&'a self, _invites: &'a [Invite]) -> BoxFuture<'a, Result<()>> {
            unimplemented!()
        }

        fn create_user_from_invite<'a>(
            &'a self,
            _invite: &Invite,
            _user: NewUserParams,
        ) -> BoxFuture<'a, Result<Option<NewUserResult>>> {
            async move { unimplemented!() }.boxed()
        }

        // invite codes

        fn set_invite_count_for_user<'a>(
            &'a self,
            _id: UserId,
            _count: u32,
        ) -> BoxFuture<'a, Result<()>> {
            async move { unimplemented!() }.boxed()
        }

        fn get_invite_code_for_user<'a>(
            &'a self,
            _id: UserId,
        ) -> BoxFuture<'a, Result<Option<(String, u32)>>> {
            async move {
                self.background.simulate_random_delay().await;
                Ok(None)
            }
            .boxed()
        }

        fn get_user_for_invite_code<'a>(&'a self, _code: &'a str) -> BoxFuture<'a, Result<User>> {
            async move { unimplemented!() }.boxed()
        }

        fn create_invite_from_code<'a>(
            &'a self,
            _code: &str,
            _email_address: &str,
            _device_id: Option<&str>,
        ) -> BoxFuture<'a, Result<Invite>> {
            async move { unimplemented!() }.boxed()
        }

        // rooms

        fn create_room<'a>(
            &'a self,
            user_id: UserId,
            connection_id: ConnectionId,
        ) -> BoxFuture<'a, Result<proto::Room>> {
            async move {
                self.background.simulate_random_delay().await;

                if !self.users.lock().contains_key(&user_id) {
                    Err(anyhow!("no such user"))?;
                }

                if self.calls.lock().contains_key(&user_id) {
                    Err(anyhow!("can't create a room with an active call"))?;
                }

                let room_id = RoomId(post_inc(&mut *self.next_room_id.lock()));
                let room = Room {
                    id: room_id,
                    version: 0,
                    live_kit_room: nanoid::nanoid!(30),
                };
                self.rooms.lock().insert(room_id, room.clone());
                self.room_participants.lock().insert(
                    room_id,
                    vec![RoomParticipant {
                        room_id,
                        user_id,
                        location_kind: None,
                        location_project_id: None,
                        connection_id: Some(connection_id.0 as i32),
                    }],
                );
                self.calls.lock().insert(
                    user_id,
                    Call {
                        room_id,
                        calling_user_id: user_id,
                        called_user_id: user_id,
                        answering_connection_id: Some(connection_id.0 as i32),
                        initial_project_id: None,
                    },
                );

                self.room_snapshot(room_id)
            }
            .boxed()
        }

        fn call<'a>(
            &'a self,
            room_id: RoomId,
            calling_user_id: UserId,
            called_user_id: UserId,
            initial_project_id: Option<ProjectId>,
        ) -> BoxFuture<'a, Result<proto::Room>> {
            async move {
                self.background.simulate_random_delay().await;

                let mut calls = self.calls.lock();
                let mut room_participants = self.room_participants.lock();

                if calls.contains_key(&called_user_id) {
                    Err(anyhow!("called user is already on another call"))?
                }

                let room_participants = room_participants
                    .get_mut(&room_id)
                    .ok_or_else(|| anyhow!("room does not exist"))?;
                if room_participants
                    .iter()
                    .any(|p| p.user_id == called_user_id)
                {
                    Err(anyhow!("user is already in the room"))?;
                }

                calls.insert(
                    called_user_id,
                    Call {
                        room_id,
                        calling_user_id,
                        called_user_id,
                        answering_connection_id: None,
                        initial_project_id,
                    },
                );
                room_participants.push(RoomParticipant {
                    room_id,
                    user_id: called_user_id,
                    location_kind: None,
                    location_project_id: None,
                    connection_id: None,
                });

                self.room_snapshot(room_id)
            }
            .boxed()
        }

        // projects

        fn register_project<'a>(
            &'a self,
            host_user_id: UserId,
        ) -> BoxFuture<'a, Result<ProjectId>> {
            async move {
                self.background.simulate_random_delay().await;
                if !self.users.lock().contains_key(&host_user_id) {
                    Err(anyhow!("no such user"))?;
                }

                let project_id = ProjectId(post_inc(&mut *self.next_project_id.lock()));
                self.projects.lock().insert(
                    project_id,
                    Project {
                        id: project_id,
                        host_user_id,
                        unregistered: false,
                    },
                );
                Ok(project_id)
            }
            .boxed()
        }

        fn unregister_project<'a>(&'a self, project_id: ProjectId) -> BoxFuture<'a, Result<()>> {
            async move {
                self.background.simulate_random_delay().await;
                self.projects
                    .lock()
                    .get_mut(&project_id)
                    .ok_or_else(|| anyhow!("no such project"))?
                    .unregistered = true;
                Ok(())
            }
            .boxed()
        }

        // contacts

        fn get_contacts<'a>(&'a self, id: UserId) -> BoxFuture<'a, Result<Vec<Contact>>> {
            async move {
                self.background.simulate_random_delay().await;
                let mut contacts = Vec::new();

                for contact in self.contacts.lock().iter() {
                    if contact.requester_id == id {
                        if contact.accepted {
                            contacts.push(Contact::Accepted {
                                user_id: contact.responder_id,
                                should_notify: contact.should_notify,
                            });
                        } else {
                            contacts.push(Contact::Outgoing {
                                user_id: contact.responder_id,
                            });
                        }
                    } else if contact.responder_id == id {
                        if contact.accepted {
                            contacts.push(Contact::Accepted {
                                user_id: contact.requester_id,
                                should_notify: false,
                            });
                        } else {
                            contacts.push(Contact::Incoming {
                                user_id: contact.requester_id,
                                should_notify: contact.should_notify,
                            });
                        }
                    }
                }

                contacts.sort_unstable_by_key(|contact| contact.user_id());
                Ok(contacts)
            }
            .boxed()
        }

        fn has_contact<'a>(
            &'a self,
            user_id_a: UserId,
            user_id_b: UserId,
        ) -> BoxFuture<'a, Result<bool>> {
            async move {
                self.background.simulate_random_delay().await;
                Ok(self.contacts.lock().iter().any(|contact| {
                    contact.accepted
                        && ((contact.requester_id == user_id_a
                            && contact.responder_id == user_id_b)
                            || (contact.requester_id == user_id_b
                                && contact.responder_id == user_id_a))
                }))
            }
            .boxed()
        }

        fn send_contact_request<'a>(
            &'a self,
            requester_id: UserId,
            responder_id: UserId,
        ) -> BoxFuture<'a, Result<()>> {
            async move {
                self.background.simulate_random_delay().await;
                let mut contacts = self.contacts.lock();
                for contact in contacts.iter_mut() {
                    if contact.requester_id == requester_id && contact.responder_id == responder_id
                    {
                        if contact.accepted {
                            Err(anyhow!("contact already exists"))?;
                        } else {
                            Err(anyhow!("contact already requested"))?;
                        }
                    }
                    if contact.responder_id == requester_id && contact.requester_id == responder_id
                    {
                        if contact.accepted {
                            Err(anyhow!("contact already exists"))?;
                        } else {
                            contact.accepted = true;
                            contact.should_notify = false;
                            return Ok(());
                        }
                    }
                }
                contacts.push(FakeContact {
                    requester_id,
                    responder_id,
                    accepted: false,
                    should_notify: true,
                });
                Ok(())
            }
            .boxed()
        }

        fn remove_contact<'a>(
            &'a self,
            requester_id: UserId,
            responder_id: UserId,
        ) -> BoxFuture<'a, Result<()>> {
            async move {
                self.background.simulate_random_delay().await;
                self.contacts.lock().retain(|contact| {
                    !(contact.requester_id == requester_id && contact.responder_id == responder_id)
                });
                Ok(())
            }
            .boxed()
        }

        fn dismiss_contact_notification<'a>(
            &'a self,
            user_id: UserId,
            contact_user_id: UserId,
        ) -> BoxFuture<'a, Result<()>> {
            async move {
                self.background.simulate_random_delay().await;
                let mut contacts = self.contacts.lock();
                for contact in contacts.iter_mut() {
                    if contact.requester_id == contact_user_id
                        && contact.responder_id == user_id
                        && !contact.accepted
                    {
                        contact.should_notify = false;
                        return Ok(());
                    }
                    if contact.requester_id == user_id
                        && contact.responder_id == contact_user_id
                        && contact.accepted
                    {
                        contact.should_notify = false;
                        return Ok(());
                    }
                }
                Err(anyhow!("no such notification"))?
            }
            .boxed()
        }

        fn respond_to_contact_request<'a>(
            &'a self,
            responder_id: UserId,
            requester_id: UserId,
            accept: bool,
        ) -> BoxFuture<'a, Result<()>> {
            async move {
                self.background.simulate_random_delay().await;
                let mut contacts = self.contacts.lock();
                for (ix, contact) in contacts.iter_mut().enumerate() {
                    if contact.requester_id == requester_id && contact.responder_id == responder_id
                    {
                        if contact.accepted {
                            Err(anyhow!("contact already confirmed"))?;
                        }
                        if accept {
                            contact.accepted = true;
                            contact.should_notify = true;
                        } else {
                            contacts.remove(ix);
                        }
                        return Ok(());
                    }
                }
                Err(anyhow!("no such contact request"))?
            }
            .boxed()
        }

        fn create_access_token_hash<'a>(
            &'a self,
            _user_id: UserId,
            _access_token_hash: &'a str,
            _max_access_token_count: usize,
        ) -> BoxFuture<'a, Result<()>> {
            async move { unimplemented!() }.boxed()
        }

        fn get_access_token_hashes<'a>(
            &'a self,
            _user_id: UserId,
        ) -> BoxFuture<'a, Result<Vec<String>>> {
            async move { unimplemented!() }.boxed()
        }

        fn find_org_by_slug<'a>(&'a self, _slug: &str) -> BoxFuture<'a, Result<Option<Org>>> {
            async move { unimplemented!() }.boxed()
        }

        fn create_org<'a>(&'a self, name: &'a str, slug: &'a str) -> BoxFuture<'a, Result<OrgId>> {
            async move {
                self.background.simulate_random_delay().await;
                let mut orgs = self.orgs.lock();
                if orgs.values().any(|org| org.slug == slug) {
                    Err(anyhow!("org already exists"))?
                } else {
                    let org_id = OrgId(post_inc(&mut *self.next_org_id.lock()));
                    orgs.insert(
                        org_id,
                        Org {
                            id: org_id,
                            name: name.to_string(),
                            slug: slug.to_string(),
                        },
                    );
                    Ok(org_id)
                }
            }
            .boxed()
        }

        fn add_org_member<'a>(
            &'a self,
            org_id: OrgId,
            user_id: UserId,
            is_admin: bool,
        ) -> BoxFuture<'a, Result<()>> {
            async move {
                self.background.simulate_random_delay().await;
                if !self.orgs.lock().contains_key(&org_id) {
                    Err(anyhow!("org does not exist"))?;
                }
                if !self.users.lock().contains_key(&user_id) {
                    Err(anyhow!("user does not exist"))?;
                }

                self.org_memberships
                    .lock()
                    .entry((org_id, user_id))
                    .or_insert(is_admin);
                Ok(())
            }
            .boxed()
        }

        fn create_org_channel<'a>(
            &'a self,
            org_id: OrgId,
            name: &'a str,
        ) -> BoxFuture<'a, Result<ChannelId>> {
            async move {
                self.background.simulate_random_delay().await;
                if !self.orgs.lock().contains_key(&org_id) {
                    Err(anyhow!("org does not exist"))?;
                }

                let mut channels = self.channels.lock();
                let channel_id = ChannelId(post_inc(&mut *self.next_channel_id.lock()));
                channels.insert(
                    channel_id,
                    Channel {
                        id: channel_id,
                        name: name.to_string(),
                        owner_id: org_id.0,
                        owner_is_user: false,
                    },
                );
                Ok(channel_id)
            }
            .boxed()
        }

        fn get_org_channels<'a>(&'a self, org_id: OrgId) -> BoxFuture<'a, Result<Vec<Channel>>> {
            async move {
                self.background.simulate_random_delay().await;
                Ok(self
                    .channels
                    .lock()
                    .values()
                    .filter(|channel| !channel.owner_is_user && channel.owner_id == org_id.0)
                    .cloned()
                    .collect())
            }
            .boxed()
        }

        fn get_accessible_channels<'a>(
            &'a self,
            user_id: UserId,
        ) -> BoxFuture<'a, Result<Vec<Channel>>> {
            async move {
                self.background.simulate_random_delay().await;
                let channels = self.channels.lock();
                let memberships = self.channel_memberships.lock();
                Ok(channels
                    .values()
                    .filter(|channel| memberships.contains_key(&(channel.id, user_id)))
                    .cloned()
                    .collect())
            }
            .boxed()
        }

        fn can_user_access_channel<'a>(
            &'a self,
            user_id: UserId,
            channel_id: ChannelId,
        ) -> BoxFuture<'a, Result<bool>> {
            async move {
                self.background.simulate_random_delay().await;
                Ok(self
                    .channel_memberships
                    .lock()
                    .contains_key(&(channel_id, user_id)))
            }
            .boxed()
        }

        fn add_channel_member<'a>(
            &'a self,
            channel_id: ChannelId,
            user_id: UserId,
            is_admin: bool,
        ) -> BoxFuture<'a, Result<()>> {
            async move {
                self.background.simulate_random_delay().await;
                if !self.channels.lock().contains_key(&channel_id) {
                    Err(anyhow!("channel does not exist"))?;
                }
                if !self.users.lock().contains_key(&user_id) {
                    Err(anyhow!("user does not exist"))?;
                }

                self.channel_memberships
                    .lock()
                    .entry((channel_id, user_id))
                    .or_insert(is_admin);
                Ok(())
            }
            .boxed()
        }

        fn create_channel_message<'a>(
            &'a self,
            channel_id: ChannelId,
            sender_id: UserId,
            body: &'a str,
            timestamp: OffsetDateTime,
            nonce: u128,
        ) -> BoxFuture<'a, Result<MessageId>> {
            async move {
                self.background.simulate_random_delay().await;
                if !self.channels.lock().contains_key(&channel_id) {
                    Err(anyhow!("channel does not exist"))?;
                }
                if !self.users.lock().contains_key(&sender_id) {
                    Err(anyhow!("user does not exist"))?;
                }

                let mut messages = self.channel_messages.lock();
                if let Some(message) = messages
                    .values()
                    .find(|message| message.nonce.as_u128() == nonce)
                {
                    Ok(message.id)
                } else {
                    let message_id = MessageId(post_inc(&mut *self.next_channel_message_id.lock()));
                    messages.insert(
                        message_id,
                        ChannelMessage {
                            id: message_id,
                            channel_id,
                            sender_id,
                            body: body.to_string(),
                            sent_at: timestamp,
                            nonce: Uuid::from_u128(nonce),
                        },
                    );
                    Ok(message_id)
                }
            }
            .boxed()
        }

        fn get_channel_messages<'a>(
            &'a self,
            channel_id: ChannelId,
            count: usize,
            before_id: Option<MessageId>,
        ) -> BoxFuture<'a, Result<Vec<ChannelMessage>>> {
            async move {
                self.background.simulate_random_delay().await;
                let mut messages = self
                    .channel_messages
                    .lock()
                    .values()
                    .rev()
                    .filter(|message| {
                        message.channel_id == channel_id
                            && message.id < before_id.unwrap_or(MessageId::MAX)
                    })
                    .take(count)
                    .cloned()
                    .collect::<Vec<_>>();
                messages.sort_unstable_by_key(|message| message.id);
                Ok(messages)
            }
            .boxed()
        }

        fn teardown<'a>(&'a self, _: &'a str) -> BoxFuture<'a, ()> {
            std::future::ready(()).boxed()
        }

        #[cfg(test)]
        fn as_fake(&self) -> Option<&FakeDb> {
            Some(self)
        }
    }

    pub struct TestDb {
        pub db: Option<Arc<dyn Db>>,
        pub url: String,
    }

    impl TestDb {
        #[allow(clippy::await_holding_lock)]
        pub async fn postgres() -> Self {
            lazy_static! {
                static ref LOCK: Mutex<()> = Mutex::new(());
            }

            let _guard = LOCK.lock();
            let mut rng = StdRng::from_entropy();
            let name = format!("zed-test-{}", rng.gen::<u128>());
            let url = format!("postgres://postgres@localhost/{}", name);
            Postgres::create_database(&url)
                .await
                .expect("failed to create test db");
            let db = PostgresDb::new(&url, 5).await.unwrap();
            db.migrate(Path::new(DEFAULT_MIGRATIONS_PATH.unwrap()), false)
                .await
                .unwrap();
            Self {
                db: Some(Arc::new(db)),
                url,
            }
        }

        pub fn fake(background: Arc<Background>) -> Self {
            Self {
                db: Some(Arc::new(FakeDb::new(background))),
                url: Default::default(),
            }
        }

        pub fn db(&self) -> &Arc<dyn Db> {
            self.db.as_ref().unwrap()
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            if let Some(db) = self.db.take() {
                futures::executor::block_on(db.teardown(&self.url));
            }
        }
    }
}
