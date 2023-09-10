CREATE TABLE Bucket(
    "bucket_id" UUID DEFAULT gen_random_uuid() NOT NULL,
    "user_id" UUID NOT NULL,
    "bucket_name" VARCHAR(255) NOT NULL,
    "bucket_size" BIGINT DEFAULT 0 NOT NULL,
    "max_bucket_size" BIGINT DEFAULT 1250000000 NOT NULL,
    "is_shared" BOOLEAN DEFAULT FALSE NOT NULL,
    "created_date" TIMESTAMP WITHOUT TIME ZONE DEFAULT now() NOT NULL
);
CREATE INDEX "bucket_bucket_name_index" ON
    Bucket("bucket_name");
ALTER TABLE
    Bucket ADD PRIMARY KEY("bucket_id");
ALTER TABLE
    Bucket ADD CONSTRAINT "bucket_bucket_name_unique" UNIQUE("bucket_name");
CREATE TABLE UserInfo(
    "user_id" UUID DEFAULT gen_random_uuid() NOT NULL,
    "user_name" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL,
    "passcode" VARCHAR(255) NOT NULL,
    "created_date" TIMESTAMP WITHOUT TIME ZONE DEFAULT now() NOT NULL
);
CREATE INDEX "userinfo_user_name_email_index" ON
    UserInfo("user_name", "email");
ALTER TABLE
    UserInfo ADD PRIMARY KEY("user_id");
ALTER TABLE
    UserInfo ADD CONSTRAINT "userinfo_user_name_unique" UNIQUE("user_name");
CREATE INDEX "userinfo_email_index" ON
    UserInfo("email");
CREATE TABLE BucketUsers(
    "bucket_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "Permissions" INTEGER DEFAULT 3 NOT NULL
);
ALTER TABLE
    BucketUsers ADD PRIMARY KEY("bucket_id", "user_id");
CREATE TABLE "FileUsers"(
    "file_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "Permissions" INTEGER DEFAULT 3 NOT NULL
);
ALTER TABLE
    "FileUsers" ADD PRIMARY KEY("file_id", "user_id");
CREATE TABLE FileMetadata(
    "file_id" UUID DEFAULT gen_random_uuid() NOT NULL,
    "file_name" VARCHAR(255) NOT NULL,
    "file_size" BIGINT NOT NULL,
    "file_type" VARCHAR(50) NOT NULL,
    "file_hash" VARCHAR(255) NOT NULL,
    "created_date" TIMESTAMP WITHOUT TIME ZONE DEFAULT now() NOT NULL
);
CREATE INDEX "filemetadata_file_name_index" ON
    FileMetadata("file_name");
ALTER TABLE
    FileMetadata ADD PRIMARY KEY("file_id");
CREATE TABLE UserFile(
    "file_id" UUID DEFAULT gen_random_uuid() NOT NULL,
    "user_id" UUID NOT NULL,
    "bucket_id" UUID NOT NULL,
    "is_shared" BOOLEAN DEFAULT FALSE NOT NULL,
    "is_public" BOOLEAN DEFAULT FALSE NOT NULL
);
ALTER TABLE
    UserFile ADD PRIMARY KEY("file_id");
ALTER TABLE
    FileMetadata ADD CONSTRAINT "filemetadata_file_id_foreign" FOREIGN KEY("file_id") REFERENCES UserFile("file_id");
ALTER TABLE
    UserFile ADD CONSTRAINT "userfile_user_id_foreign" FOREIGN KEY("user_id") REFERENCES UserInfo("user_id");
ALTER TABLE
    "FileUsers" ADD CONSTRAINT "fileusers_user_id_foreign" FOREIGN KEY("user_id") REFERENCES UserInfo("user_id");
ALTER TABLE
    UserFile ADD CONSTRAINT "userfile_bucket_id_foreign" FOREIGN KEY("bucket_id") REFERENCES Bucket("bucket_id");
ALTER TABLE
    BucketUsers ADD CONSTRAINT "bucketusers_user_id_foreign" FOREIGN KEY("user_id") REFERENCES UserInfo("user_id");
ALTER TABLE
    "FileUsers" ADD CONSTRAINT "fileusers_file_id_foreign" FOREIGN KEY("file_id") REFERENCES UserFile("file_id");
ALTER TABLE
    Bucket ADD CONSTRAINT "bucket_user_id_foreign" FOREIGN KEY("user_id") REFERENCES UserInfo("user_id");
ALTER TABLE
    BucketUsers ADD CONSTRAINT "bucketusers_bucket_id_foreign" FOREIGN KEY("bucket_id") REFERENCES Bucket("bucket_id");
