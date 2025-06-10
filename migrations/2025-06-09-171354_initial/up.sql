-- Your SQL goes here
DROP TABLE IF EXISTS "equations";
CREATE TABLE "temporaldata"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"timestamp" TIMESTAMP NOT NULL,
	"value" FLOAT4 NOT NULL
);

