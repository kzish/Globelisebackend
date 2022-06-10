--
-- Name: country_codes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.country_codes (
    code text NOT NULL,
    long_name text NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.country_codes OWNER TO postgres;

--
-- Data for Name: country_codes; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO public.country_codes (code, long_name) VALUES ('AF', 'Afghanistan');
INSERT INTO public.country_codes (code, long_name) VALUES ('AL', 'Albania');
INSERT INTO public.country_codes (code, long_name) VALUES ('DZ', 'Algeria');
INSERT INTO public.country_codes (code, long_name) VALUES ('AS', 'American Samoa');
INSERT INTO public.country_codes (code, long_name) VALUES ('AD', 'Andorra');
INSERT INTO public.country_codes (code, long_name) VALUES ('AO', 'Angola');
INSERT INTO public.country_codes (code, long_name) VALUES ('AI', 'Anguilla');
INSERT INTO public.country_codes (code, long_name) VALUES ('AQ', 'Antarctica');
INSERT INTO public.country_codes (code, long_name) VALUES ('AG', 'Antigua and Barbuda');
INSERT INTO public.country_codes (code, long_name) VALUES ('AR', 'Argentina');
INSERT INTO public.country_codes (code, long_name) VALUES ('AM', 'Armenia');
INSERT INTO public.country_codes (code, long_name) VALUES ('AW', 'Aruba');
INSERT INTO public.country_codes (code, long_name) VALUES ('AU', 'Australia');
INSERT INTO public.country_codes (code, long_name) VALUES ('AT', 'Austria');
INSERT INTO public.country_codes (code, long_name) VALUES ('AZ', 'Azerbaijan');
INSERT INTO public.country_codes (code, long_name) VALUES ('BS', 'Bahamas');
INSERT INTO public.country_codes (code, long_name) VALUES ('BH', 'Bahrain');
INSERT INTO public.country_codes (code, long_name) VALUES ('BD', 'Bangladesh');
INSERT INTO public.country_codes (code, long_name) VALUES ('BB', 'Barbados');
INSERT INTO public.country_codes (code, long_name) VALUES ('BY', 'Belarus');
INSERT INTO public.country_codes (code, long_name) VALUES ('BE', 'Belgium');
INSERT INTO public.country_codes (code, long_name) VALUES ('BZ', 'Belize');
INSERT INTO public.country_codes (code, long_name) VALUES ('BJ', 'Benin');
INSERT INTO public.country_codes (code, long_name) VALUES ('BM', 'Bermuda');
INSERT INTO public.country_codes (code, long_name) VALUES ('BT', 'Bhutan');
INSERT INTO public.country_codes (code, long_name) VALUES ('BO', 'Bolivia');
INSERT INTO public.country_codes (code, long_name) VALUES ('BA', 'Bosnia and Herzegovina');
INSERT INTO public.country_codes (code, long_name) VALUES ('BW', 'Botswana');
INSERT INTO public.country_codes (code, long_name) VALUES ('BV', 'Bouvet Island');
INSERT INTO public.country_codes (code, long_name) VALUES ('BR', 'Brazil');
INSERT INTO public.country_codes (code, long_name) VALUES ('IO', 'British Indian Ocean Territory');
INSERT INTO public.country_codes (code, long_name) VALUES ('BN', 'Brunei Darussalam');
INSERT INTO public.country_codes (code, long_name) VALUES ('BG', 'Bulgaria');
INSERT INTO public.country_codes (code, long_name) VALUES ('BF', 'Burkina Faso');
INSERT INTO public.country_codes (code, long_name) VALUES ('BI', 'Burundi');
INSERT INTO public.country_codes (code, long_name) VALUES ('KH', 'Cambodia');
INSERT INTO public.country_codes (code, long_name) VALUES ('CM', 'Cameroon');
INSERT INTO public.country_codes (code, long_name) VALUES ('CA', 'Canada');
INSERT INTO public.country_codes (code, long_name) VALUES ('CV', 'Cape Verde');
INSERT INTO public.country_codes (code, long_name) VALUES ('KY', 'Cayman Islands');
INSERT INTO public.country_codes (code, long_name) VALUES ('CF', 'Central African Republic');
INSERT INTO public.country_codes (code, long_name) VALUES ('TD', 'Chad');
INSERT INTO public.country_codes (code, long_name) VALUES ('CL', 'Chile');
INSERT INTO public.country_codes (code, long_name) VALUES ('CN', 'China');
INSERT INTO public.country_codes (code, long_name) VALUES ('CX', 'Christmas Island');
INSERT INTO public.country_codes (code, long_name) VALUES ('CC', 'Cocos (Keeling) Islands');
INSERT INTO public.country_codes (code, long_name) VALUES ('CO', 'Colombia');
INSERT INTO public.country_codes (code, long_name) VALUES ('KM', 'Comoros');
INSERT INTO public.country_codes (code, long_name) VALUES ('CG', 'Congo');
INSERT INTO public.country_codes (code, long_name) VALUES ('CD', 'Congo, the Democratic Republic of the');
INSERT INTO public.country_codes (code, long_name) VALUES ('CK', 'Cook Islands');
INSERT INTO public.country_codes (code, long_name) VALUES ('CR', 'Costa Rica');
INSERT INTO public.country_codes (code, long_name) VALUES ('CI', 'Cote D''Ivoire');
INSERT INTO public.country_codes (code, long_name) VALUES ('HR', 'Croatia');
INSERT INTO public.country_codes (code, long_name) VALUES ('CU', 'Cuba');
INSERT INTO public.country_codes (code, long_name) VALUES ('CY', 'Cyprus');
INSERT INTO public.country_codes (code, long_name) VALUES ('CZ', 'Czech Republic');
INSERT INTO public.country_codes (code, long_name) VALUES ('DK', 'Denmark');
INSERT INTO public.country_codes (code, long_name) VALUES ('DJ', 'Djibouti');
INSERT INTO public.country_codes (code, long_name) VALUES ('DM', 'Dominica');
INSERT INTO public.country_codes (code, long_name) VALUES ('DO', 'Dominican Republic');
INSERT INTO public.country_codes (code, long_name) VALUES ('EC', 'Ecuador');
INSERT INTO public.country_codes (code, long_name) VALUES ('EG', 'Egypt');
INSERT INTO public.country_codes (code, long_name) VALUES ('SV', 'El Salvador');
INSERT INTO public.country_codes (code, long_name) VALUES ('GQ', 'Equatorial Guinea');
INSERT INTO public.country_codes (code, long_name) VALUES ('ER', 'Eritrea');
INSERT INTO public.country_codes (code, long_name) VALUES ('EE', 'Estonia');
INSERT INTO public.country_codes (code, long_name) VALUES ('ET', 'Ethiopia');
INSERT INTO public.country_codes (code, long_name) VALUES ('FK', 'Falkland Islands (Malvinas)');
INSERT INTO public.country_codes (code, long_name) VALUES ('FO', 'Faroe Islands');
INSERT INTO public.country_codes (code, long_name) VALUES ('FJ', 'Fiji');
INSERT INTO public.country_codes (code, long_name) VALUES ('FI', 'Finland');
INSERT INTO public.country_codes (code, long_name) VALUES ('FR', 'France');
INSERT INTO public.country_codes (code, long_name) VALUES ('GF', 'French Guiana');
INSERT INTO public.country_codes (code, long_name) VALUES ('PF', 'French Polynesia');
INSERT INTO public.country_codes (code, long_name) VALUES ('TF', 'French Southern Territories');
INSERT INTO public.country_codes (code, long_name) VALUES ('GA', 'Gabon');
INSERT INTO public.country_codes (code, long_name) VALUES ('GM', 'Gambia');
INSERT INTO public.country_codes (code, long_name) VALUES ('GE', 'Georgia');
INSERT INTO public.country_codes (code, long_name) VALUES ('DE', 'Germany');
INSERT INTO public.country_codes (code, long_name) VALUES ('GH', 'Ghana');
INSERT INTO public.country_codes (code, long_name) VALUES ('GI', 'Gibraltar');
INSERT INTO public.country_codes (code, long_name) VALUES ('GR', 'Greece');
INSERT INTO public.country_codes (code, long_name) VALUES ('GL', 'Greenland');
INSERT INTO public.country_codes (code, long_name) VALUES ('GD', 'Grenada');
INSERT INTO public.country_codes (code, long_name) VALUES ('GP', 'Guadeloupe');
INSERT INTO public.country_codes (code, long_name) VALUES ('GU', 'Guam');
INSERT INTO public.country_codes (code, long_name) VALUES ('GT', 'Guatemala');
INSERT INTO public.country_codes (code, long_name) VALUES ('GN', 'Guinea');
INSERT INTO public.country_codes (code, long_name) VALUES ('GW', 'Guinea-Bissau');
INSERT INTO public.country_codes (code, long_name) VALUES ('GY', 'Guyana');
INSERT INTO public.country_codes (code, long_name) VALUES ('HT', 'Haiti');
INSERT INTO public.country_codes (code, long_name) VALUES ('HM', 'Heard Island and Mcdonald Islands');
INSERT INTO public.country_codes (code, long_name) VALUES ('VA', 'Holy See (Vatican City State)');
INSERT INTO public.country_codes (code, long_name) VALUES ('HN', 'Honduras');
INSERT INTO public.country_codes (code, long_name) VALUES ('HK', 'Hong Kong');
INSERT INTO public.country_codes (code, long_name) VALUES ('HU', 'Hungary');
INSERT INTO public.country_codes (code, long_name) VALUES ('IS', 'Iceland');
INSERT INTO public.country_codes (code, long_name) VALUES ('IN', 'India');
INSERT INTO public.country_codes (code, long_name) VALUES ('ID', 'Indonesia');
INSERT INTO public.country_codes (code, long_name) VALUES ('IR', 'Iran, Islamic Republic of');
INSERT INTO public.country_codes (code, long_name) VALUES ('IQ', 'Iraq');
INSERT INTO public.country_codes (code, long_name) VALUES ('IE', 'Ireland');
INSERT INTO public.country_codes (code, long_name) VALUES ('IL', 'Israel');
INSERT INTO public.country_codes (code, long_name) VALUES ('IT', 'Italy');
INSERT INTO public.country_codes (code, long_name) VALUES ('JM', 'Jamaica');
INSERT INTO public.country_codes (code, long_name) VALUES ('JP', 'Japan');
INSERT INTO public.country_codes (code, long_name) VALUES ('JO', 'Jordan');
INSERT INTO public.country_codes (code, long_name) VALUES ('KZ', 'Kazakhstan');
INSERT INTO public.country_codes (code, long_name) VALUES ('KE', 'Kenya');
INSERT INTO public.country_codes (code, long_name) VALUES ('KI', 'Kiribati');
INSERT INTO public.country_codes (code, long_name) VALUES ('KP', 'Korea, Democratic People''s Republic of');
INSERT INTO public.country_codes (code, long_name) VALUES ('KR', 'Korea, Republic of');
INSERT INTO public.country_codes (code, long_name) VALUES ('KW', 'Kuwait');
INSERT INTO public.country_codes (code, long_name) VALUES ('KG', 'Kyrgyzstan');
INSERT INTO public.country_codes (code, long_name) VALUES ('LA', 'Lao People''s Democratic Republic');
INSERT INTO public.country_codes (code, long_name) VALUES ('LV', 'Latvia');
INSERT INTO public.country_codes (code, long_name) VALUES ('LB', 'Lebanon');
INSERT INTO public.country_codes (code, long_name) VALUES ('LS', 'Lesotho');
INSERT INTO public.country_codes (code, long_name) VALUES ('LR', 'Liberia');
INSERT INTO public.country_codes (code, long_name) VALUES ('LY', 'Libyan Arab Jamahiriya');
INSERT INTO public.country_codes (code, long_name) VALUES ('LI', 'Liechtenstein');
INSERT INTO public.country_codes (code, long_name) VALUES ('LT', 'Lithuania');
INSERT INTO public.country_codes (code, long_name) VALUES ('LU', 'Luxembourg');
INSERT INTO public.country_codes (code, long_name) VALUES ('MO', 'Macao');
INSERT INTO public.country_codes (code, long_name) VALUES ('MK', 'Macedonia, the Former Yugoslav Republic of');
INSERT INTO public.country_codes (code, long_name) VALUES ('MG', 'Madagascar');
INSERT INTO public.country_codes (code, long_name) VALUES ('MW', 'Malawi');
INSERT INTO public.country_codes (code, long_name) VALUES ('MY', 'Malaysia');
INSERT INTO public.country_codes (code, long_name) VALUES ('MV', 'Maldives');
INSERT INTO public.country_codes (code, long_name) VALUES ('ML', 'Mali');
INSERT INTO public.country_codes (code, long_name) VALUES ('MT', 'Malta');
INSERT INTO public.country_codes (code, long_name) VALUES ('MH', 'Marshall Islands');
INSERT INTO public.country_codes (code, long_name) VALUES ('MQ', 'Martinique');
INSERT INTO public.country_codes (code, long_name) VALUES ('MR', 'Mauritania');
INSERT INTO public.country_codes (code, long_name) VALUES ('MU', 'Mauritius');
INSERT INTO public.country_codes (code, long_name) VALUES ('YT', 'Mayotte');
INSERT INTO public.country_codes (code, long_name) VALUES ('MX', 'Mexico');
INSERT INTO public.country_codes (code, long_name) VALUES ('FM', 'Micronesia, Federated States of');
INSERT INTO public.country_codes (code, long_name) VALUES ('MD', 'Moldova, Republic of');
INSERT INTO public.country_codes (code, long_name) VALUES ('MC', 'Monaco');
INSERT INTO public.country_codes (code, long_name) VALUES ('MN', 'Mongolia');
INSERT INTO public.country_codes (code, long_name) VALUES ('MS', 'Montserrat');
INSERT INTO public.country_codes (code, long_name) VALUES ('MA', 'Morocco');
INSERT INTO public.country_codes (code, long_name) VALUES ('MZ', 'Mozambique');
INSERT INTO public.country_codes (code, long_name) VALUES ('MM', 'Myanmar');
INSERT INTO public.country_codes (code, long_name) VALUES ('NA', 'Namibia');
INSERT INTO public.country_codes (code, long_name) VALUES ('NR', 'Nauru');
INSERT INTO public.country_codes (code, long_name) VALUES ('NP', 'Nepal');
INSERT INTO public.country_codes (code, long_name) VALUES ('NL', 'Netherlands');
INSERT INTO public.country_codes (code, long_name) VALUES ('AN', 'Netherlands Antilles');
INSERT INTO public.country_codes (code, long_name) VALUES ('NC', 'New Caledonia');
INSERT INTO public.country_codes (code, long_name) VALUES ('NZ', 'New Zealand');
INSERT INTO public.country_codes (code, long_name) VALUES ('NI', 'Nicaragua');
INSERT INTO public.country_codes (code, long_name) VALUES ('NE', 'Niger');
INSERT INTO public.country_codes (code, long_name) VALUES ('NG', 'Nigeria');
INSERT INTO public.country_codes (code, long_name) VALUES ('NU', 'Niue');
INSERT INTO public.country_codes (code, long_name) VALUES ('NF', 'Norfolk Island');
INSERT INTO public.country_codes (code, long_name) VALUES ('MP', 'Northern Mariana Islands');
INSERT INTO public.country_codes (code, long_name) VALUES ('NO', 'Norway');
INSERT INTO public.country_codes (code, long_name) VALUES ('OM', 'Oman');
INSERT INTO public.country_codes (code, long_name) VALUES ('PK', 'Pakistan');
INSERT INTO public.country_codes (code, long_name) VALUES ('PW', 'Palau');
INSERT INTO public.country_codes (code, long_name) VALUES ('PS', 'Palestinian Territory, Occupied');
INSERT INTO public.country_codes (code, long_name) VALUES ('PA', 'Panama');
INSERT INTO public.country_codes (code, long_name) VALUES ('PG', 'Papua New Guinea');
INSERT INTO public.country_codes (code, long_name) VALUES ('PY', 'Paraguay');
INSERT INTO public.country_codes (code, long_name) VALUES ('PE', 'Peru');
INSERT INTO public.country_codes (code, long_name) VALUES ('PH', 'Philippines');
INSERT INTO public.country_codes (code, long_name) VALUES ('PN', 'Pitcairn');
INSERT INTO public.country_codes (code, long_name) VALUES ('PL', 'Poland');
INSERT INTO public.country_codes (code, long_name) VALUES ('PT', 'Portugal');
INSERT INTO public.country_codes (code, long_name) VALUES ('PR', 'Puerto Rico');
INSERT INTO public.country_codes (code, long_name) VALUES ('QA', 'Qatar');
INSERT INTO public.country_codes (code, long_name) VALUES ('RE', 'Reunion');
INSERT INTO public.country_codes (code, long_name) VALUES ('RO', 'Romania');
INSERT INTO public.country_codes (code, long_name) VALUES ('RU', 'Russian Federation');
INSERT INTO public.country_codes (code, long_name) VALUES ('RW', 'Rwanda');
INSERT INTO public.country_codes (code, long_name) VALUES ('SH', 'Saint Helena');
INSERT INTO public.country_codes (code, long_name) VALUES ('KN', 'Saint Kitts and Nevis');
INSERT INTO public.country_codes (code, long_name) VALUES ('LC', 'Saint Lucia');
INSERT INTO public.country_codes (code, long_name) VALUES ('PM', 'Saint Pierre and Miquelon');
INSERT INTO public.country_codes (code, long_name) VALUES ('VC', 'Saint Vincent and the Grenadines');
INSERT INTO public.country_codes (code, long_name) VALUES ('WS', 'Samoa');
INSERT INTO public.country_codes (code, long_name) VALUES ('SM', 'San Marino');
INSERT INTO public.country_codes (code, long_name) VALUES ('ST', 'Sao Tome and Principe');
INSERT INTO public.country_codes (code, long_name) VALUES ('SA', 'Saudi Arabia');
INSERT INTO public.country_codes (code, long_name) VALUES ('SN', 'Senegal');
INSERT INTO public.country_codes (code, long_name) VALUES ('CS', 'Serbia and Montenegro');
INSERT INTO public.country_codes (code, long_name) VALUES ('SC', 'Seychelles');
INSERT INTO public.country_codes (code, long_name) VALUES ('SL', 'Sierra Leone');
INSERT INTO public.country_codes (code, long_name) VALUES ('SG', 'Singapore');
INSERT INTO public.country_codes (code, long_name) VALUES ('SK', 'Slovakia');
INSERT INTO public.country_codes (code, long_name) VALUES ('SI', 'Slovenia');
INSERT INTO public.country_codes (code, long_name) VALUES ('SB', 'Solomon Islands');
INSERT INTO public.country_codes (code, long_name) VALUES ('SO', 'Somalia');
INSERT INTO public.country_codes (code, long_name) VALUES ('ZA', 'South Africa');
INSERT INTO public.country_codes (code, long_name) VALUES ('GS', 'South Georgia and the South Sandwich Islands');
INSERT INTO public.country_codes (code, long_name) VALUES ('ES', 'Spain');
INSERT INTO public.country_codes (code, long_name) VALUES ('LK', 'Sri Lanka');
INSERT INTO public.country_codes (code, long_name) VALUES ('SD', 'Sudan');
INSERT INTO public.country_codes (code, long_name) VALUES ('SR', 'Suriname');
INSERT INTO public.country_codes (code, long_name) VALUES ('SJ', 'Svalbard and Jan Mayen');
INSERT INTO public.country_codes (code, long_name) VALUES ('SZ', 'Swaziland');
INSERT INTO public.country_codes (code, long_name) VALUES ('SE', 'Sweden');
INSERT INTO public.country_codes (code, long_name) VALUES ('CH', 'Switzerland');
INSERT INTO public.country_codes (code, long_name) VALUES ('SY', 'Syrian Arab Republic');
INSERT INTO public.country_codes (code, long_name) VALUES ('TW', 'Taiwan, Province of China');
INSERT INTO public.country_codes (code, long_name) VALUES ('TJ', 'Tajikistan');
INSERT INTO public.country_codes (code, long_name) VALUES ('TZ', 'Tanzania, United Republic of');
INSERT INTO public.country_codes (code, long_name) VALUES ('TH', 'Thailand');
INSERT INTO public.country_codes (code, long_name) VALUES ('TL', 'Timor-Leste');
INSERT INTO public.country_codes (code, long_name) VALUES ('TG', 'Togo');
INSERT INTO public.country_codes (code, long_name) VALUES ('TK', 'Tokelau');
INSERT INTO public.country_codes (code, long_name) VALUES ('TO', 'Tonga');
INSERT INTO public.country_codes (code, long_name) VALUES ('TT', 'Trinidad and Tobago');
INSERT INTO public.country_codes (code, long_name) VALUES ('TN', 'Tunisia');
INSERT INTO public.country_codes (code, long_name) VALUES ('TR', 'Turkey');
INSERT INTO public.country_codes (code, long_name) VALUES ('TM', 'Turkmenistan');
INSERT INTO public.country_codes (code, long_name) VALUES ('TC', 'Turks and Caicos Islands');
INSERT INTO public.country_codes (code, long_name) VALUES ('TV', 'Tuvalu');
INSERT INTO public.country_codes (code, long_name) VALUES ('UG', 'Uganda');
INSERT INTO public.country_codes (code, long_name) VALUES ('UA', 'Ukraine');
INSERT INTO public.country_codes (code, long_name) VALUES ('AE', 'United Arab Emirates');
INSERT INTO public.country_codes (code, long_name) VALUES ('GB', 'United Kingdom');
INSERT INTO public.country_codes (code, long_name) VALUES ('US', 'United States');
INSERT INTO public.country_codes (code, long_name) VALUES ('UM', 'United States Minor Outlying Islands');
INSERT INTO public.country_codes (code, long_name) VALUES ('UY', 'Uruguay');
INSERT INTO public.country_codes (code, long_name) VALUES ('UZ', 'Uzbekistan');
INSERT INTO public.country_codes (code, long_name) VALUES ('VU', 'Vanuatu');
INSERT INTO public.country_codes (code, long_name) VALUES ('VE', 'Venezuela');
INSERT INTO public.country_codes (code, long_name) VALUES ('VN', 'Viet Nam');
INSERT INTO public.country_codes (code, long_name) VALUES ('VG', 'Virgin Islands, British');
INSERT INTO public.country_codes (code, long_name) VALUES ('VI', 'Virgin Islands, U.s.');
INSERT INTO public.country_codes (code, long_name) VALUES ('WF', 'Wallis and Futuna');
INSERT INTO public.country_codes (code, long_name) VALUES ('EH', 'Western Sahara');
INSERT INTO public.country_codes (code, long_name) VALUES ('YE', 'Yemen');
INSERT INTO public.country_codes (code, long_name) VALUES ('ZM', 'Zambia');
INSERT INTO public.country_codes (code, long_name) VALUES ('ZW', 'Zimbabwe');


--
-- Name: country_codes country_codes_long_name_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.country_codes
    ADD CONSTRAINT country_codes_long_name_key UNIQUE (long_name);


--
-- Name: country_codes country_codes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.country_codes
    ADD CONSTRAINT country_codes_pkey PRIMARY KEY (code);