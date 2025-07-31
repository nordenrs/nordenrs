CREATE TABLE "tb_users"(
	"id" SERIAL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"password" TEXT NOT NULL,
	"email" TEXT NOT NULL,
	"role" TEXT
);

