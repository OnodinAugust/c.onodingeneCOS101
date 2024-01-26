--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text NOT NULL,
    dno integer NOT NULL,
    age integer NOT NULL,
    staff_sal real NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, age, staff_sal, mobile) FROM stdin;
100	Mustapha_Ali	3	32	175000	8063285831
107	Alokwe_Martin	7	48	380000	7090082812
97	Dankade_Aminat	5	40	550000	9023688832
108	Josiah_Joshua	1	30	120000	8053189131
102	Mankinde_Mary	2	55	450000	90234878830
120	Adeleke_Jane	4	38	200000	7061045862
122	Osahon_Mark	6	44	320000	8022289842
104	Kuti_Lawal	1	35	750000	9145689842
117	Suleman_Ajayi	3	50	800000	7030089981
\.


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

