delete from uploaded_citibank_transfer_initiation_files;

ALTER TABLE uploaded_citibank_transfer_initiation_files
DROP COLUMN created_at;

ALTER TABLE IF EXISTS public.uploaded_citibank_transfer_initiation_files
ADD COLUMN created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP;
  
  