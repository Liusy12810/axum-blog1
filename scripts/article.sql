CREATE TABLE authblog.topics (
  id BIGSERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  category_id INT NOT NULL,
  summary VARCHAR(255) NOT NULL,
  markdown VARCHAR NOT NULL,
  html VARCHAR NOT NULL,
  hit INT NOT NULL DEFAULT 0,
  dateline TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  is_del BOOLEAN NOT NULL DEFAULT FALSE,
  FOREIGN KEY (category_id) REFERENCES authblog.categories (id)
);