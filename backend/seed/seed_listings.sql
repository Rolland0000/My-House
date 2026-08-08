-- MH-23: Seed data for the public read-only listings feed (EP-02.5).
--
-- Dev-only fixture — never part of the sqlx migration chain (migrations run
-- unconditionally on every boot, including staging/prod; seed data must not).
-- Inserts owner users + listings + listing_media directly via SQL, with
-- placeholder image URLs (no media upload path exists yet).
--
-- Usage (local dev, matches backend/.env):
--   psql "$DATABASE_URL" -f backend/seed/seed_listings.sql
--
-- Idempotent: fixed UUIDs + ON CONFLICT DO NOTHING, safe to re-run.

-- ============================================================
-- Owners
-- ============================================================
INSERT INTO users (id, email, role, first_name, last_name, phone, is_active)
VALUES
    ('11111111-1111-1111-1111-111111111111', 'moussa.diallo@myhouse.dev', 'owner', 'Moussa', 'Diallo', '+221771234567', TRUE),
    ('22222222-2222-2222-2222-222222222222', 'fatou.ndiaye@myhouse.dev',  'owner', 'Fatou',  'Ndiaye', '+221781234568', TRUE),
    ('33333333-3333-3333-3333-333333333333', 'amadou.ba@myhouse.dev',    'owner', 'Amadou', 'Ba',     '+221701234569', TRUE),
    ('44444444-4444-4444-4444-444444444444', 'claire.martin@myhouse.dev', 'owner', 'Claire', 'Martin', '+33612345670', TRUE)
ON CONFLICT (id) DO NOTHING;

-- ============================================================
-- Listings
-- ============================================================
INSERT INTO listings (id, owner_id, title, description, type, status, city, neighborhood, price, currency, surface_m2, rooms)
VALUES
    (
        'a1111111-0000-0000-0000-000000000001',
        '11111111-1111-1111-1111-111111111111',
        'Studio meublé Plateau',
        'Studio lumineux entièrement meublé au cœur du Plateau, à deux pas des commerces et des transports. Cuisine équipée, salle d''eau moderne.',
        'studio', 'available', 'Dakar', 'Plateau', 150000, 'XOF', 28, 1
    ),
    (
        'a1111111-0000-0000-0000-000000000002',
        '22222222-2222-2222-2222-222222222222',
        'Appartement 3 pièces Almadies',
        'Bel appartement de 3 pièces avec vue dégagée, quartier calme et résidentiel des Almadies. Parking sécurisé inclus.',
        'apartment', 'available', 'Dakar', 'Almadies', 420000, 'XOF', 85, 3
    ),
    (
        'a1111111-0000-0000-0000-000000000003',
        '33333333-3333-3333-3333-333333333333',
        'Villa avec jardin Cocody',
        'Villa spacieuse avec jardin privatif et piscine, idéale pour une famille. Quartier résidentiel sécurisé de Cocody.',
        'villa', 'available', 'Abidjan', 'Cocody', 950000, 'XOF', 220, 5
    ),
    (
        'a1111111-0000-0000-0000-000000000004',
        '33333333-3333-3333-3333-333333333333',
        'Chambre meublée Yopougon',
        'Chambre meublée dans une maison partagée, accès cuisine et salon communs. Quartier animé et bien desservi.',
        'room', 'unavailable', 'Abidjan', 'Yopougon', 65000, 'XOF', 14, 1
    ),
    (
        'a1111111-0000-0000-0000-000000000005',
        '44444444-4444-4444-4444-444444444444',
        'Deux pièces rénové Belleville',
        'Deux pièces entièrement rénové, cuisine ouverte, proche métro et commerces. Immeuble ancien avec ascenseur.',
        'apartment', 'available', 'Paris', 'Belleville', 1250, 'EUR', 38, 2
    ),
    (
        'a1111111-0000-0000-0000-000000000006',
        '44444444-4444-4444-4444-444444444444',
        'Maison de ville Montreuil',
        'Maison de ville avec petite cour extérieure, proche de Paris. Trois chambres, double séjour.',
        'house', 'available', 'Montreuil', NULL, 1850, 'EUR', 95, 4
    )
ON CONFLICT (id) DO NOTHING;

-- ============================================================
-- Listing media — placeholder image URLs (no upload path yet)
-- ============================================================
INSERT INTO listing_media (id, listing_id, storage_key, url, is_cover, position)
VALUES
    ('b1111111-0000-0000-0000-000000000001', 'a1111111-0000-0000-0000-000000000001', 'seed/1-cover.jpg', 'https://picsum.photos/seed/myhouse-listing-1/800/600', TRUE,  0),
    ('b1111111-0000-0000-0000-000000000002', 'a1111111-0000-0000-0000-000000000001', 'seed/1-alt.jpg',   'https://picsum.photos/seed/myhouse-listing-1b/800/600', FALSE, 1),

    ('b1111111-0000-0000-0000-000000000003', 'a1111111-0000-0000-0000-000000000002', 'seed/2-cover.jpg', 'https://picsum.photos/seed/myhouse-listing-2/800/600', TRUE,  0),

    ('b1111111-0000-0000-0000-000000000004', 'a1111111-0000-0000-0000-000000000003', 'seed/3-cover.jpg', 'https://picsum.photos/seed/myhouse-listing-3/800/600', TRUE,  0),
    ('b1111111-0000-0000-0000-000000000005', 'a1111111-0000-0000-0000-000000000003', 'seed/3-alt.jpg',   'https://picsum.photos/seed/myhouse-listing-3b/800/600', FALSE, 1),
    ('b1111111-0000-0000-0000-000000000006', 'a1111111-0000-0000-0000-000000000003', 'seed/3-alt2.jpg',  'https://picsum.photos/seed/myhouse-listing-3c/800/600', FALSE, 2),

    ('b1111111-0000-0000-0000-000000000007', 'a1111111-0000-0000-0000-000000000004', 'seed/4-cover.jpg', 'https://picsum.photos/seed/myhouse-listing-4/800/600', TRUE,  0),

    ('b1111111-0000-0000-0000-000000000008', 'a1111111-0000-0000-0000-000000000005', 'seed/5-cover.jpg', 'https://picsum.photos/seed/myhouse-listing-5/800/600', TRUE,  0),
    ('b1111111-0000-0000-0000-000000000009', 'a1111111-0000-0000-0000-000000000005', 'seed/5-alt.jpg',   'https://picsum.photos/seed/myhouse-listing-5b/800/600', FALSE, 1),

    ('b1111111-0000-0000-0000-00000000000a', 'a1111111-0000-0000-0000-000000000006', 'seed/6-cover.jpg', 'https://picsum.photos/seed/myhouse-listing-6/800/600', TRUE,  0)
ON CONFLICT (id) DO NOTHING;
