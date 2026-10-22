--
-- PostgreSQL database dump
--

\restrict cIv6NU1O0w1bxxMgUwrHwYkOmfH2sO9qoiDidx2jpG78icN5wo2BrhwZOc8NJ0I

-- Dumped from database version 18.6 (Homebrew)
-- Dumped by pg_dump version 18.6 (Homebrew)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: citext; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS citext WITH SCHEMA public;


--
-- Name: EXTENSION citext; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION citext IS 'data type for case-insensitive character strings';


--
-- Name: email; Type: DOMAIN; Schema: public; Owner: postgres
--

CREATE DOMAIN public.email AS public.citext
	CONSTRAINT email_check CHECK ((VALUE OPERATOR(public.~~) '%@%'::public.citext));


ALTER DOMAIN public.email OWNER TO postgres;

--
-- Name: role_kind; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.role_kind AS ENUM (
    'admin',
    'member'
);


ALTER TYPE public.role_kind OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: posts; Type: TABLE; Schema: public; Owner: app_owner
--

CREATE TABLE public.posts (
    id bigint NOT NULL,
    author_id integer NOT NULL,
    title text NOT NULL,
    tags text[]
);


ALTER TABLE public.posts OWNER TO app_owner;

--
-- Name: post_count(integer); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.post_count(uid integer) RETURNS bigint
    LANGUAGE sql STABLE
    BEGIN ATOMIC
 SELECT count(*) AS count
    FROM public.posts
   WHERE (posts.author_id = post_count.uid);
END;


ALTER FUNCTION public.post_count(uid integer) OWNER TO postgres;

--
-- Name: posts_id_seq; Type: SEQUENCE; Schema: public; Owner: app_owner
--

ALTER TABLE public.posts ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.posts_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: users; Type: TABLE; Schema: public; Owner: app_owner
--

CREATE TABLE public.users (
    id integer NOT NULL,
    email public.email NOT NULL,
    kind public.role_kind DEFAULT 'member'::public.role_kind NOT NULL,
    nick text
);


ALTER TABLE public.users OWNER TO app_owner;

--
-- Name: user_posts; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.user_posts AS
 SELECT u.id,
    u.email,
    p.title
   FROM (public.users u
     JOIN public.posts p ON ((p.author_id = u.id)));


ALTER VIEW public.user_posts OWNER TO postgres;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: app_owner
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.users_id_seq OWNER TO app_owner;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: app_owner
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: app_owner
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: posts posts_pkey; Type: CONSTRAINT; Schema: public; Owner: app_owner
--

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT posts_pkey PRIMARY KEY (id);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: app_owner
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: app_owner
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: posts posts_author_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: app_owner
--

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT posts_author_id_fkey FOREIGN KEY (author_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: TABLE posts; Type: ACL; Schema: public; Owner: app_owner
--

GRANT SELECT ON TABLE public.posts TO app_reader;


--
-- Name: TABLE users; Type: ACL; Schema: public; Owner: app_owner
--

GRANT SELECT ON TABLE public.users TO app_reader;


--
-- Name: TABLE user_posts; Type: ACL; Schema: public; Owner: postgres
--

GRANT SELECT ON TABLE public.user_posts TO app_reader;


--
-- PostgreSQL database dump complete
--

\unrestrict cIv6NU1O0w1bxxMgUwrHwYkOmfH2sO9qoiDidx2jpG78icN5wo2BrhwZOc8NJ0I

