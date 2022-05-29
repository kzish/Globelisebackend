--
-- Name: currency_codes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.currency_codes (
    code text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.currency_codes OWNER TO postgres;

--
-- Data for Name: currency_codes; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO public.currency_codes (code) VALUES ('AED');
INSERT INTO public.currency_codes (code) VALUES ('AFN');
INSERT INTO public.currency_codes (code) VALUES ('ALL');
INSERT INTO public.currency_codes (code) VALUES ('AMD');
INSERT INTO public.currency_codes (code) VALUES ('ANG');
INSERT INTO public.currency_codes (code) VALUES ('AOA');
INSERT INTO public.currency_codes (code) VALUES ('ARS');
INSERT INTO public.currency_codes (code) VALUES ('AUD');
INSERT INTO public.currency_codes (code) VALUES ('AWG');
INSERT INTO public.currency_codes (code) VALUES ('AZN');
INSERT INTO public.currency_codes (code) VALUES ('BAM');
INSERT INTO public.currency_codes (code) VALUES ('BBD');
INSERT INTO public.currency_codes (code) VALUES ('BDT');
INSERT INTO public.currency_codes (code) VALUES ('BGN');
INSERT INTO public.currency_codes (code) VALUES ('BHD');
INSERT INTO public.currency_codes (code) VALUES ('BIF');
INSERT INTO public.currency_codes (code) VALUES ('BMD');
INSERT INTO public.currency_codes (code) VALUES ('BND');
INSERT INTO public.currency_codes (code) VALUES ('BOB');
INSERT INTO public.currency_codes (code) VALUES ('BOV');
INSERT INTO public.currency_codes (code) VALUES ('BRL');
INSERT INTO public.currency_codes (code) VALUES ('BSD');
INSERT INTO public.currency_codes (code) VALUES ('BTN');
INSERT INTO public.currency_codes (code) VALUES ('BWP');
INSERT INTO public.currency_codes (code) VALUES ('BYN');
INSERT INTO public.currency_codes (code) VALUES ('BZD');
INSERT INTO public.currency_codes (code) VALUES ('CAD');
INSERT INTO public.currency_codes (code) VALUES ('CDF');
INSERT INTO public.currency_codes (code) VALUES ('CHE');
INSERT INTO public.currency_codes (code) VALUES ('CHF');
INSERT INTO public.currency_codes (code) VALUES ('CHW');
INSERT INTO public.currency_codes (code) VALUES ('CLF');
INSERT INTO public.currency_codes (code) VALUES ('CLP');
INSERT INTO public.currency_codes (code) VALUES ('CNY');
INSERT INTO public.currency_codes (code) VALUES ('COP');
INSERT INTO public.currency_codes (code) VALUES ('COU');
INSERT INTO public.currency_codes (code) VALUES ('CRC');
INSERT INTO public.currency_codes (code) VALUES ('CUC');
INSERT INTO public.currency_codes (code) VALUES ('CUP');
INSERT INTO public.currency_codes (code) VALUES ('CVE');
INSERT INTO public.currency_codes (code) VALUES ('CZK');
INSERT INTO public.currency_codes (code) VALUES ('DJF');
INSERT INTO public.currency_codes (code) VALUES ('DKK');
INSERT INTO public.currency_codes (code) VALUES ('DOP');
INSERT INTO public.currency_codes (code) VALUES ('DZD');
INSERT INTO public.currency_codes (code) VALUES ('EGP');
INSERT INTO public.currency_codes (code) VALUES ('ERN');
INSERT INTO public.currency_codes (code) VALUES ('ETB');
INSERT INTO public.currency_codes (code) VALUES ('EUR');
INSERT INTO public.currency_codes (code) VALUES ('FJD');
INSERT INTO public.currency_codes (code) VALUES ('FKP');
INSERT INTO public.currency_codes (code) VALUES ('GBP');
INSERT INTO public.currency_codes (code) VALUES ('GEL');
INSERT INTO public.currency_codes (code) VALUES ('GHS');
INSERT INTO public.currency_codes (code) VALUES ('GIP');
INSERT INTO public.currency_codes (code) VALUES ('GMD');
INSERT INTO public.currency_codes (code) VALUES ('GNF');
INSERT INTO public.currency_codes (code) VALUES ('GTQ');
INSERT INTO public.currency_codes (code) VALUES ('GYD');
INSERT INTO public.currency_codes (code) VALUES ('HKD');
INSERT INTO public.currency_codes (code) VALUES ('HNL');
INSERT INTO public.currency_codes (code) VALUES ('HRK');
INSERT INTO public.currency_codes (code) VALUES ('HTG');
INSERT INTO public.currency_codes (code) VALUES ('HUF');
INSERT INTO public.currency_codes (code) VALUES ('IDR');
INSERT INTO public.currency_codes (code) VALUES ('ILS');
INSERT INTO public.currency_codes (code) VALUES ('INR');
INSERT INTO public.currency_codes (code) VALUES ('IQD');
INSERT INTO public.currency_codes (code) VALUES ('IRR');
INSERT INTO public.currency_codes (code) VALUES ('ISK');
INSERT INTO public.currency_codes (code) VALUES ('JMD');
INSERT INTO public.currency_codes (code) VALUES ('JOD');
INSERT INTO public.currency_codes (code) VALUES ('JPY');
INSERT INTO public.currency_codes (code) VALUES ('KES');
INSERT INTO public.currency_codes (code) VALUES ('KGS');
INSERT INTO public.currency_codes (code) VALUES ('KHR');
INSERT INTO public.currency_codes (code) VALUES ('KMF');
INSERT INTO public.currency_codes (code) VALUES ('KPW');
INSERT INTO public.currency_codes (code) VALUES ('KRW');
INSERT INTO public.currency_codes (code) VALUES ('KWD');
INSERT INTO public.currency_codes (code) VALUES ('KYD');
INSERT INTO public.currency_codes (code) VALUES ('KZT');
INSERT INTO public.currency_codes (code) VALUES ('LAK');
INSERT INTO public.currency_codes (code) VALUES ('LBP');
INSERT INTO public.currency_codes (code) VALUES ('LKR');
INSERT INTO public.currency_codes (code) VALUES ('LRD');
INSERT INTO public.currency_codes (code) VALUES ('LSL');
INSERT INTO public.currency_codes (code) VALUES ('LYD');
INSERT INTO public.currency_codes (code) VALUES ('MAD');
INSERT INTO public.currency_codes (code) VALUES ('MDL');
INSERT INTO public.currency_codes (code) VALUES ('MGA');
INSERT INTO public.currency_codes (code) VALUES ('MKD');
INSERT INTO public.currency_codes (code) VALUES ('MMK');
INSERT INTO public.currency_codes (code) VALUES ('MNT');
INSERT INTO public.currency_codes (code) VALUES ('MOP');
INSERT INTO public.currency_codes (code) VALUES ('MRU');
INSERT INTO public.currency_codes (code) VALUES ('MUR');
INSERT INTO public.currency_codes (code) VALUES ('MVR');
INSERT INTO public.currency_codes (code) VALUES ('MWK');
INSERT INTO public.currency_codes (code) VALUES ('MXN');
INSERT INTO public.currency_codes (code) VALUES ('MXV');
INSERT INTO public.currency_codes (code) VALUES ('MYR');
INSERT INTO public.currency_codes (code) VALUES ('MZN');
INSERT INTO public.currency_codes (code) VALUES ('NAD');
INSERT INTO public.currency_codes (code) VALUES ('NGN');
INSERT INTO public.currency_codes (code) VALUES ('NIO');
INSERT INTO public.currency_codes (code) VALUES ('NOK');
INSERT INTO public.currency_codes (code) VALUES ('NPR');
INSERT INTO public.currency_codes (code) VALUES ('NZD');
INSERT INTO public.currency_codes (code) VALUES ('OMR');
INSERT INTO public.currency_codes (code) VALUES ('PAB');
INSERT INTO public.currency_codes (code) VALUES ('PEN');
INSERT INTO public.currency_codes (code) VALUES ('PGK');
INSERT INTO public.currency_codes (code) VALUES ('PHP');
INSERT INTO public.currency_codes (code) VALUES ('PKR');
INSERT INTO public.currency_codes (code) VALUES ('PLN');
INSERT INTO public.currency_codes (code) VALUES ('PYG');
INSERT INTO public.currency_codes (code) VALUES ('QAR');
INSERT INTO public.currency_codes (code) VALUES ('RON');
INSERT INTO public.currency_codes (code) VALUES ('RSD');
INSERT INTO public.currency_codes (code) VALUES ('RUB');
INSERT INTO public.currency_codes (code) VALUES ('RWF');
INSERT INTO public.currency_codes (code) VALUES ('SAR');
INSERT INTO public.currency_codes (code) VALUES ('SBD');
INSERT INTO public.currency_codes (code) VALUES ('SCR');
INSERT INTO public.currency_codes (code) VALUES ('SDG');
INSERT INTO public.currency_codes (code) VALUES ('SEK');
INSERT INTO public.currency_codes (code) VALUES ('SGD');
INSERT INTO public.currency_codes (code) VALUES ('SHP');
INSERT INTO public.currency_codes (code) VALUES ('SLL');
INSERT INTO public.currency_codes (code) VALUES ('SOS');
INSERT INTO public.currency_codes (code) VALUES ('SRD');
INSERT INTO public.currency_codes (code) VALUES ('SSP');
INSERT INTO public.currency_codes (code) VALUES ('STN');
INSERT INTO public.currency_codes (code) VALUES ('SVC');
INSERT INTO public.currency_codes (code) VALUES ('SYP');
INSERT INTO public.currency_codes (code) VALUES ('SZL');
INSERT INTO public.currency_codes (code) VALUES ('THB');
INSERT INTO public.currency_codes (code) VALUES ('TJS');
INSERT INTO public.currency_codes (code) VALUES ('TMT');
INSERT INTO public.currency_codes (code) VALUES ('TND');
INSERT INTO public.currency_codes (code) VALUES ('TOP');
INSERT INTO public.currency_codes (code) VALUES ('TRY');
INSERT INTO public.currency_codes (code) VALUES ('TTD');
INSERT INTO public.currency_codes (code) VALUES ('TWD');
INSERT INTO public.currency_codes (code) VALUES ('TZS');
INSERT INTO public.currency_codes (code) VALUES ('UAH');
INSERT INTO public.currency_codes (code) VALUES ('UGX');
INSERT INTO public.currency_codes (code) VALUES ('USD');
INSERT INTO public.currency_codes (code) VALUES ('USN');
INSERT INTO public.currency_codes (code) VALUES ('UYI');
INSERT INTO public.currency_codes (code) VALUES ('UYU');
INSERT INTO public.currency_codes (code) VALUES ('UYW');
INSERT INTO public.currency_codes (code) VALUES ('UZS');
INSERT INTO public.currency_codes (code) VALUES ('VED');
INSERT INTO public.currency_codes (code) VALUES ('VES');
INSERT INTO public.currency_codes (code) VALUES ('VND');
INSERT INTO public.currency_codes (code) VALUES ('VUV');
INSERT INTO public.currency_codes (code) VALUES ('WST');
INSERT INTO public.currency_codes (code) VALUES ('XAF');
INSERT INTO public.currency_codes (code) VALUES ('XAG');
INSERT INTO public.currency_codes (code) VALUES ('XAU');
INSERT INTO public.currency_codes (code) VALUES ('XBA');
INSERT INTO public.currency_codes (code) VALUES ('XBB');
INSERT INTO public.currency_codes (code) VALUES ('XBC');
INSERT INTO public.currency_codes (code) VALUES ('XBD');
INSERT INTO public.currency_codes (code) VALUES ('XCD');
INSERT INTO public.currency_codes (code) VALUES ('XDR');
INSERT INTO public.currency_codes (code) VALUES ('XOF');
INSERT INTO public.currency_codes (code) VALUES ('XPD');
INSERT INTO public.currency_codes (code) VALUES ('XPF');
INSERT INTO public.currency_codes (code) VALUES ('XPT');
INSERT INTO public.currency_codes (code) VALUES ('XSU');
INSERT INTO public.currency_codes (code) VALUES ('XTS');
INSERT INTO public.currency_codes (code) VALUES ('XUA');
INSERT INTO public.currency_codes (code) VALUES ('XXX');
INSERT INTO public.currency_codes (code) VALUES ('YER');
INSERT INTO public.currency_codes (code) VALUES ('ZAR');
INSERT INTO public.currency_codes (code) VALUES ('ZMW');
INSERT INTO public.currency_codes (code) VALUES ('ZWL');


--
-- Name: currency_codes currency_codes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.currency_codes
    ADD CONSTRAINT currency_codes_pkey PRIMARY KEY (code);
