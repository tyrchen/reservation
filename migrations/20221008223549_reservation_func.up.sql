-- if user_id is null, find all reservations within during for the resource
-- if resource_id is null, find all reservations within during for the user
-- if both are null, find all reservations within during
-- if both set, find all reservations within during for the resource and user
CREATE OR REPLACE FUNCTION rsvp.query(uid text, rid text, during TSTZRANGE) RETURNS TABLE (LIKE rsvp.reservations) AS $$
BEGIN
    -- if both are null, find all reservations within during
    IF uid IS NULL AND rid IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE timespan && during;
    ELSIF uid IS NULL THEN
        -- if user_id is null, find all reservations within during for the resource
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE resource_id = rid AND during @> timespan;
    ELSIF rid IS NULL THEN
        -- if resource_id is null, find all reservations within during for the user
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE user_id = uid AND during @> timespan;
    ELSE
        -- if both set, find all reservations within during for the resource and user
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE resource_id = rid AND user_id = uid AND during @> timespan;
    END IF;
END;
$$ LANGUAGE plpgsql;
