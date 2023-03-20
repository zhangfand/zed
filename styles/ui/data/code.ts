export const code = `
    pub async fn stale_room_ids(
            &self,
            environment: &str,
            new_server_id: ServerId,
        ) -> Result<Vec<RoomId>> {
            self.transaction(|tx| async move {
                #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
                enum QueryAs {
                    RoomId,
                }

                let stale_server_epochs = self
                    .stale_server_ids(environment, new_server_id, &tx)
                    .await?;
                Ok(room_participant::Entity::find()
                    .select_only()
                    .column(room_participant::Column::RoomId)
                    .distinct()
                    .filter(
                        room_participant::Column::AnsweringConnectionServerId
                            .is_in(stale_server_epochs),
                    )
                    .into_values::<_, QueryAs>()
                    .all(&*tx)
                    .await?)
            })
            .await
        }

        pub async fn refresh_room(
            &self,
            room_id: RoomId,
            new_server_id: ServerId,
        ) -> Result<RoomGuard<RefreshedRoom>> {
            self.room_transaction(room_id, |tx| async move {
                let stale_participant_filter = Condition::all()
                    .add(room_participant::Column::RoomId.eq(room_id))
                    .add(room_participant::Column::AnsweringConnectionId.is_not_null())
                    .add(room_participant::Column::AnsweringConnectionServerId.ne(new_server_id));

                let stale_participant_user_ids = room_participant::Entity::find()
                    .filter(stale_participant_filter.clone())
                    .all(&*tx)
                    .await?
                    .into_iter()
                    .map(|participant| participant.user_id)
                    .collect::<Vec<_>>();

                // Delete participants who failed to reconnect.
                room_participant::Entity::delete_many()
                    .filter(stale_participant_filter)
                    .exec(&*tx)
                    .await?;

                let room = self.get_room(room_id, &tx).await?;
                let mut canceled_calls_to_user_ids = Vec::new();
                // Delete the room if it becomes empty and cancel pending calls.
                if room.participants.is_empty() {
                    canceled_calls_to_user_ids.extend(
                        room.pending_participants
                            .iter()
                            .map(|pending_participant| UserId::from_proto(pending_participant.user_id)),
                    );
                    room_participant::Entity::delete_many()
                        .filter(room_participant::Column::RoomId.eq(room_id))
                        .exec(&*tx)
                        .await?;
                    project::Entity::delete_many()
                        .filter(project::Column::RoomId.eq(room_id))
                        .exec(&*tx)
                        .await?;
                    room::Entity::delete_by_id(room_id).exec(&*tx).await?;
                }

                Ok(RefreshedRoom {
                    room,
                    stale_participant_user_ids,
                    canceled_calls_to_user_ids,
                })
            })
            .await
        }

        pub async fn delete_stale_servers(
            &self,
            environment: &str,
            new_server_id: ServerId,
        ) -> Result<()> {
            self.transaction(|tx| async move {
                server::Entity::delete_many()
                    .filter(
                        Condition::all()
                            .add(server::Column::Environment.eq(environment))
                            .add(server::Column::Id.ne(new_server_id)),
                    )
                    .exec(&*tx)
                    .await?;
                Ok(())
            })
            .await
        }

        async fn stale_server_ids(
            &self,
            environment: &str,
            new_server_id: ServerId,
            tx: &DatabaseTransaction,
        ) -> Result<Vec<ServerId>> {
            let stale_servers = server::Entity::find()
                .filter(
                    Condition::all()
                        .add(server::Column::Environment.eq(environment))
                        .add(server::Column::Id.ne(new_server_id)),
                )
                .all(&*tx)
                .await?;
            Ok(stale_servers.into_iter().map(|server| server.id).collect())
        }
`
