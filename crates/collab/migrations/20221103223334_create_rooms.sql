CREATE TABLE IF NOT EXISTS "rooms" (
    "id" SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS "room_authorizations" (
    "id" SERIAL PRIMARY KEY,
    "room_id" INTEGER REFERENCES rooms (id) ON DELETE CASCADE,
    "user_id" INTEGER REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX "index_room_authorizations_room_id" ON "room_authorizations" ("room_id");
CREATE UNIQUE INDEX "index_room_authorizations_room_id_user_id" ON "room_authorizations" ("room_id", "user_id");
