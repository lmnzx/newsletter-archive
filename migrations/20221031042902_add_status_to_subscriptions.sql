-- Add status column to the subscriptions table
ALTER TABLE
  subscriptions
ADD
  COLUMN status TEXT NULL;