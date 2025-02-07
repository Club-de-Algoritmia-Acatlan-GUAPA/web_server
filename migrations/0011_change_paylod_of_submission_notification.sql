-- Add migration script here
-- create pgmq extension
CREATE EXTENSION IF NOT EXISTS pgmq;

-- create queue
SELECT pgmq.create('submission_queue');

-- create notification function
CREATE OR REPLACE FUNCTION notify_queue_insert() RETURNS TRIGGER AS $$
BEGIN
  RAISE NOTICE 'Value: %', TG_OP;

  IF (TG_OP = 'INSERT') THEN
    PERFORM pg_notify('submission_channel', '' || json_build_object( 'msg_id', NEW.msg_id)::text);
  END IF;
  RETURN NULL; -- Since this is an AFTER trigger, we can return NULL
END;
$$ LANGUAGE plpgsql;

-- create trigger
CREATE OR REPLACE TRIGGER queue_insert_trigger
AFTER INSERT ON pgmq.q_submission_queue
FOR EACH ROW EXECUTE FUNCTION notify_queue_insert();

