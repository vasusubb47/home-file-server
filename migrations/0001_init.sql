CREATE TABLE UserInfo(
  user_id uuid DEFAULT gen_random_uuid() NOT NULL,
  user_name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  passcode VARCHAR(255) NOT NULL,
  created_date timestamp DEFAULT now() NOT NULL,
  CONSTRAINT unique_email UNIQUE(email),
  CONSTRAINT user_id_key PRIMARY KEY(user_id)
);

CREATE UNIQUE INDEX user_email_index ON UserInfo(email);

CREATE TABLE UserFile(
  file_id uuid DEFAULT gen_random_uuid() NOT NULL,
  user_id uuid references UserInfo(user_id) NOT NULL,
  file_name VARCHAR(255) NOT NULL,
  file_size int NOT NULL,
  file_hash VARCHAR(255) NOT NULL,
  created_date timestamp DEFAULT now() NOT NULL,
  is_shared BOOLEAN DEFAULT False NOT NULL,
  CONSTRAINT file_id_key PRIMARY KEY(file_id)
);