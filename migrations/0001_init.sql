CREATE TABLE UserInfo(
  user_id uuid DEFAULT gen_random_uuid() NOT NULL PRIMARY KEY,
  user_name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  passcode VARCHAR(255) NOT NULL
);

CREATE TABLE File(
  file_id uuid DEFAULT gen_random_uuid() NOT NULL PRIMARY KEY,
  user_id uuid references UserInfo(user_id) NOT NULL,
  file_name VARCHAR(255) NOT NULL,
  created_date timestamp NOT NULL,
  is_shared BOOLEAN NOT NULL
);