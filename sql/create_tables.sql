CREATE TABLE regions (
    id SERIAL PRIMARY KEY,
    name character varying(100) NOT NULL,
    translations text,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    flag boolean DEFAULT true NOT NULL,
    wikidataid character varying(255)
);
--||
CREATE TABLE subregions (
    id SERIAL PRIMARY KEY,
    name character varying(100) NOT NULL,
    translations text,
    region_id integer NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    flag boolean DEFAULT true NOT NULL,
    wikidataid character varying(255)
);
--||
CREATE TABLE countries (
    id SERIAL PRIMARY KEY,
    name character varying(100) NOT NULL,
    iso3 character(3),
    numeric_code character(3),
    iso2 character(2),
    phonecode character varying(255),
    capital character varying(255),
    currency character varying(255),
    currency_name character varying(255),
    currency_symbol character varying(255),
    tld character varying(255),
    native character varying(255),
    region character varying(255),
    region_id integer,
    subregion character varying(255),
    subregion_id integer,
    nationality character varying(255),
    timezones text,
    translations text,
    latitude numeric(10,8),
    longitude numeric(11,8),
    emoji character varying(191),
    emojiu character varying(191),
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    flag boolean DEFAULT true NOT NULL,
    wikidataid character varying(255)
);
--||
CREATE TABLE states (
    id SERIAL PRIMARY KEY,
    name character varying(255) NOT NULL,
    country_id integer NOT NULL,
    country_code character(2) NOT NULL,
    fips_code character varying(255),
    iso2 character varying(255),
    type character varying(191),
    latitude numeric(10,8),
    longitude numeric(11,8),
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    flag boolean DEFAULT true NOT NULL,
    wikidataid character varying(255)
);
--||
CREATE TABLE cities (
    id SERIAL PRIMARY KEY,
    name character varying(255) NOT NULL,
    state_id integer NOT NULL,
    state_code character varying(255) NOT NULL,
    country_id integer NOT NULL,
    country_code character(2) NOT NULL,
    latitude numeric(10,8) NOT NULL,
    longitude numeric(11,8) NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    flag boolean DEFAULT true NOT NULL,
    wikidataid character varying(255)
);