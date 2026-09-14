-- Migration: Create loan_requests table
-- Direction: down
-- Description: Reverts the loan_requests table creation by removing all associated objects.
-- This migration is executed when rolling back this database change.

-- First, remove the trigger that updates timestamps
-- Must drop trigger before dropping the function it references
DROP TRIGGER IF EXISTS update_loan_requests_updated_at ON loan_requests;

-- Drop the trigger function that was created for auto-updating timestamps
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop all indexes created for query optimization
-- These indexes improve query performance but are not required for table existence
DROP INDEX IF EXISTS idx_loan_requests_status_created;
DROP INDEX IF EXISTS idx_loan_requests_applicant_name;
DROP INDEX IF EXISTS idx_loan_requests_created_at;
DROP INDEX IF EXISTS idx_loan_requests_status;
DROP INDEX IF EXISTS idx_loan_requests_request_id;

-- Drop the main table
-- This permanently deletes all loan request data - cannot be undone
DROP TABLE IF EXISTS loan_requests;

-- Note: The uuid-ossp extension is NOT dropped here because:
-- 1. It might be used by other tables in the database
-- 2. Extensions are database-level objects, not schema-level
-- 3. Dropping extensions can cause issues with other objects
-- If this is the only table using UUIDs, manually run: DROP EXTENSION IF EXISTS "uuid-ossp";

-- Verify cleanup: this query should return no rows if successful
-- SELECT * FROM information_schema.tables WHERE table_name = 'loan_requests';