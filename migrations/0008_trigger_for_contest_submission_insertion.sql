-- Add migration script here
-- Add migration script here
CREATE OR REPLACE FUNCTION notify_contest_change() RETURNS TRIGGER AS $$
BEGIN
  RAISE NOTICE 'Value: %', TG_OP;

  IF (TG_OP = 'UPDATE' OR TG_OP = 'INSERT') THEN
    PERFORM pg_notify('contest_channel', '' || json_build_object( 'contest_id', NEW.contest_id, 'status', NEW.status)::text);
  END IF;
  RETURN NULL; -- Since this is an AFTER trigger, we can return NULL
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER contest_change_trigger
AFTER UPDATE ON contest_submission
FOR EACH ROW EXECUTE FUNCTION notify_contest_change();

