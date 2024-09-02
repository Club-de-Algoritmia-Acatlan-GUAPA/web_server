-- Add migration script here
CREATE OR REPLACE FUNCTION notify_change() RETURNS TRIGGER AS $$
BEGIN
  RAISE NOTICE 'Value: %', TG_OP;

  IF (TG_OP = 'UPDATE') THEN
    PERFORM pg_notify('submission_channel', '' || json_build_object( 'submission_id', NEW.submission_id, 'status', NEW.status)::text);
  END IF;
  RETURN NULL; -- Since this is an AFTER trigger, we can return NULL
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER submission_change_trigger
AFTER UPDATE ON submission
FOR EACH ROW EXECUTE FUNCTION notify_change();

