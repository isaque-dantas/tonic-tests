-- This file should undo anything in `up.sql`
CREATE TABLE "equations"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"content" VARCHAR(16) NOT NULL,
	"answer" FLOAT4 NOT NULL
);

DROP TABLE IF EXISTS "temporaldata";
