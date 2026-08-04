-- MH-20: Database triggers & functions

-- ============================================================
-- Triggers
-- ============================================================

-- search_vector : mis à jour à chaque INSERT/UPDATE sur listings
-- Inclut le nom de l'owner et le libellé français du type de bien (pondérés B)
-- pour permettre la recherche par propriétaire et par label de bien
CREATE OR REPLACE FUNCTION fn_update_listing_search_vector()
RETURNS TRIGGER AS $$
DECLARE
    v_owner_name TEXT;
    v_type_label TEXT;
BEGIN
    SELECT COALESCE(first_name, '') || ' ' || COALESCE(last_name, '')
        INTO v_owner_name
        FROM users WHERE id = NEW.owner_id;

    v_type_label := CASE NEW.type
        WHEN 'apartment' THEN 'appartement'
        WHEN 'studio'    THEN 'studio'
        WHEN 'house'     THEN 'maison'
        WHEN 'room'      THEN 'chambre'
        WHEN 'villa'     THEN 'villa'
        WHEN 'other'     THEN 'autre'
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

CREATE TRIGGER tg_listings_search_vector
    BEFORE INSERT OR UPDATE ON listings
    FOR EACH ROW EXECUTE FUNCTION fn_update_listing_search_vector();

-- Cascade : si le nom de l'owner est modifié, invalide les search_vector
-- de ses listings (le trigger tg_listings_search_vector les recalcule au prochain UPDATE)
CREATE OR REPLACE FUNCTION fn_cascade_owner_name_to_listings()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.first_name IS DISTINCT FROM NEW.first_name
    OR OLD.last_name  IS DISTINCT FROM NEW.last_name THEN
        UPDATE listings SET updated_at = NOW()
        WHERE owner_id = NEW.id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tg_users_cascade_search
    AFTER UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION fn_cascade_owner_name_to_listings();

-- updated_at automatique
CREATE OR REPLACE FUNCTION fn_set_updated_at()
RETURNS TRIGGER AS $$
BEGIN NEW.updated_at = NOW(); RETURN NEW; END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tg_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION fn_set_updated_at();

CREATE TRIGGER tg_listings_updated_at
    BEFORE UPDATE ON listings
    FOR EACH ROW EXECUTE FUNCTION fn_set_updated_at();

CREATE TRIGGER tg_owner_requests_updated_at
    BEFORE UPDATE ON owner_requests
    FOR EACH ROW EXECUTE FUNCTION fn_set_updated_at();