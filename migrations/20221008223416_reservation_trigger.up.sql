
-- resevation change queue
CREATE TABLE rsvp.reservation_changes (
    id SERIAL NOT NULL,
    reservation_id BIGSERIAL NOT NULL,
    old JSONB,
    new JSONB,
    op rsvp.reservation_update_type NOT NULL,
    CONSTRAINT reservation_changes_pkey PRIMARY KEY (id)
);
CREATE INDEX reservation_changes_reservation_id_op_idx ON rsvp.reservation_changes (reservation_id, op);

-- server read cursor
CREATE TABLE rsvp.server_read_cursor (
    server_id VARCHAR(64) NOT NULL,
    last_change_id BIGSERIAL NOT NULL,
    CONSTRAINT reservation_changes_cursor_pkey PRIMARY KEY (server_id)
);

-- trigger for add/update/delete a reservation
CREATE OR REPLACE FUNCTION rsvp.reservations_trigger() RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        -- update reservation_changes
        INSERT INTO rsvp.reservation_changes (reservation_id, old, new, op) VALUES (NEW.id, null, to_jsonb(NEW), 'create');
    ELSIF TG_OP = 'UPDATE' THEN
        -- if status changed, update reservation_changes
        IF OLD.status <> NEW.status THEN
            INSERT INTO rsvp.reservation_changes (reservation_id, old, new, op) VALUES (NEW.id, to_jsonb(OLD), to_jsonb(NEW), 'update');
        END IF;
    ELSIF TG_OP = 'DELETE' THEN
        -- update reservation_changes
        INSERT INTO rsvp.reservation_changes (reservation_id, old, new, op) VALUES (OLD.id, to_jsonb(OLD), null, 'delete');
    END IF;
    -- notify a channel called reservation_update
    NOTIFY reservation_update;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER reservations_trigger
    AFTER INSERT OR UPDATE OR DELETE ON rsvp.reservations
    FOR EACH ROW EXECUTE PROCEDURE rsvp.reservations_trigger();
