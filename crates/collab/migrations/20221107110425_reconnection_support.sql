CREATE TABLE IF NOT EXISTS "worktrees" (
    "id" INTEGER NOT NULL,
    "project_id" INTEGER NOT NULL REFERENCES projects (id),
    "root_name" VARCHAR NOT NULL,
    PRIMARY KEY(project_id, id)
);
CREATE INDEX "index_worktrees_on_project_id" ON "worktrees" ("project_id");

CREATE TABLE IF NOT EXISTS "rooms" (
    "id" SERIAL PRIMARY KEY,
    "live_kit_room" VARCHAR
);

CREATE TABLE IF NOT EXISTS "room_participants" (
    "id" SERIAL PRIMARY KEY,
    "room_id" INTEGER NOT NULL REFERENCES rooms (id) ON DELETE CASCADE,
    "user_id" INTEGER NOT NULL REFERENCES users (id),
    "peer_id" INTEGER,
    "location_kind" INTEGER,
    "location_project_id" INTEGER REFERENCES projects (id), 
    "server_epoch" UUID NOT NULL
);
CREATE UNIQUE INDEX "index_room_participants_on_user_id_and_room_id" ON "room_participants" ("user_id", "room_id");

CREATE TABLE IF NOT EXISTS "room_participant_projects" (
    "id" SERIAL PRIMARY KEY,
    "room_participant_id" INTEGER NOT NULL REFERENCES room_participants (id) ON DELETE CASCADE,
    "project_id" INTEGER NOT NULL REFERENCES projects (id)
);
CREATE INDEX "index_room_participant_projects_on_room_participant_id" ON "room_participant_projects" ("room_participant_id");
