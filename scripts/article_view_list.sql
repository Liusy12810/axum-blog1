CREATE VIEW authblog.v_topic_cat_list AS
SELECT t.id,
  title,
  summary,
  hit,
  dateline,
  category_id,
  t.is_del,
  c.name AS category_name
FROM authblog.topics AS t
  INNER JOIN authblog.categories AS c ON t.category_id = c.id
WHERE c.is_del = false;