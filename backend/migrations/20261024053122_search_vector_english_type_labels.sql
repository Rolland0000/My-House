-- MH-66: index the English type label next to the French one, so the word shown
-- in the interface finds the listing. Same weight (B); every other field is unchanged.
CREATE OR REPLACE FUNCTION fn_update_listing_search_vector()
RETURNS TRIGGER AS $$
DECLARE
    v_owner_name TEXT;
    v_type_label TEXT;
BEGIN
    SELECT COALESCE(first_name, '') || ' ' || COALESCE(last_name, '')
        INTO v_owner_name
        FROM users WHERE id = NEW.owner_id;

    -- French then English; studio and villa are the same word in both.
    v_type_label := CASE NEW.type
        WHEN 'apartment' THEN 'appartement apartment'
        WHEN 'studio'    THEN 'studio'
        WHEN 'house'     THEN 'maison house'
        WHEN 'room'      THEN 'chambre room'
        WHEN 'villa'     THEN 'villa'
        WHEN 'other'     THEN 'autre other'
    END;

    NEW.search_vector :=
        setweight(to_tsvector('french', unaccent(COALESCE(NEW.title, ''))),        'A') ||
        setweight(to_tsvector('french', unaccent(COALESCE(NEW.city, ''))),          'B') ||
        setweight(to_tsvector('french', unaccent(COALESCE(NEW.neighborhood, ''))),  'B') ||
        setweight(to_tsvector('french', unaccent(COALESCE(v_owner_name, ''))),      'B') ||
        setweight(to_tsvector('french', unaccent(COALESCE(v_type_label, ''))),      'B') ||
        setweight(to_tsvector('french', unaccent(COALESCE(NEW.description, ''))),   'C');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Reindex existing listings: any UPDATE fires tg_listings_search_vector. The
-- updated_at trigger is paused so the reindex does not look like an owner edit.
ALTER TABLE listings DISABLE TRIGGER tg_listings_updated_at;
UPDATE listings SET type = type;
ALTER TABLE listings ENABLE TRIGGER tg_listings_updated_at;
