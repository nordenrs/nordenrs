-- Your SQL goes here
CREATE TABLE "tb_users"(
	"id" SERIAL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"password" TEXT NOT NULL,
	"email" TEXT NOT NULL,
	"role" TEXT NOT NULL
);

