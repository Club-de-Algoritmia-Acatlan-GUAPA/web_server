-- Add migration script here
CREATE OR REPLACE FUNCTION notify_submission_change() RETURNS TRIGGER AS $$
BEGIN
  RAISE NOTICE 'Value: %', TG_OP;

  IF (TG_OP = 'INSERT') THEN
    PERFORM pg_notify('submission_channel', '' || json_build_object( 'msg_id', NEW.msg_id, 'status', NEW.message)::text);
  END IF;
  RETURN NULL; -- Since this is an AFTER trigger, we can return NULL
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER test_change_trigger
AFTER INSERT ON pgmq.q_submission_queue
FOR EACH ROW EXECUTE FUNCTION notify_submission_change();

