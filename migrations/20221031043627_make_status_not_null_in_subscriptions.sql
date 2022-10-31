-- Turn the existing user's status to confirmed where it is null
BEGIN;

UPDATE
  subscriptions
SET
  status = 'confirmed'
WHERE
  status IS NULL;

-- Make `status` mandatory
ALTER TABLE
  subscriptions
ALTER COLUMN
  status
SET
  NOT NULL;

COMMIT;