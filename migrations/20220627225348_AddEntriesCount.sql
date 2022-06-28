-- View: public.uploaded_citibank_transfer_initiation_files_index

-- DROP VIEW public.uploaded_citibank_transfer_initiation_files_index;

CREATE OR REPLACE VIEW public.uploaded_citibank_transfer_initiation_files_index
 AS
 SELECT uploaded_citibank_transfer_initiation_files.ulid,
    uploaded_citibank_transfer_initiation_files.title_identifier,
    uploaded_citibank_transfer_initiation_files.status,
    uploaded_citibank_transfer_initiation_files.created_at,
    uploaded_citibank_transfer_initiation_files.client_ulid,
    ( SELECT count(*) AS count
           FROM uploaded_citibank_transfer_initiation_files_records
          WHERE uploaded_citibank_transfer_initiation_files_records.file_ulid = uploaded_citibank_transfer_initiation_files.ulid) AS entries
   FROM uploaded_citibank_transfer_initiation_files;

ALTER TABLE public.uploaded_citibank_transfer_initiation_files_index
    OWNER TO postgres;

