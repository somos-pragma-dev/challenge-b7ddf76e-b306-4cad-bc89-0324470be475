-- Migration: Create loan_requests table
-- Direction: up
-- Description: Creates the loan_requests table for storing loan applications in the loan management system.
-- This table stores applicant information, loan amount, term, and status with proper constraints.

-- Enable UUID extension for generating unique identifiers
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create the loan_requests table with all required fields and constraints
CREATE TABLE loan_requests (
    -- Primary key: UUID-based identifier for global uniqueness across distributed systems
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Request identifier: unique string for idempotency - prevents duplicate submissions
    -- This field is critical for the duplicate detection requirement in the domain
    request_id VARCHAR(64) NOT NULL UNIQUE,
    
    -- Applicant name: required field storing the full name of the loan applicant
    -- Stored as VARCHAR to support international names with varying lengths
    applicant_name VARCHAR(255) NOT NULL,
    
    -- Loan amount: decimal field with precision for currency values
    -- Constraint: amount must be between 100 and 10000 units per domain requirements
    amount NUMERIC(12, 2) NOT NULL CHECK (amount >= 100 AND amount <= 10000),
    
    -- Term in months: integer field storing the loan repayment period
    -- Constraint: term must be between 6 and 60 months per domain requirements
    term_months INTEGER NOT NULL CHECK (term_months >= 6 AND term_months <= 60),
    
    -- Status: current state of the loan application
    -- Values: pending, approved, rejected, cancelled, under_review
    -- Default: pending - new applications start in pending status
    status VARCHAR(20) NOT NULL DEFAULT 'pending'::character varying,
    
    -- Rejection reason: optional field storing why a loan was rejected
    -- Only populated when status is 'rejected'
    rejection_reason TEXT,
    
    -- Approval date: timestamp when the loan was approved
    -- Nullable - only populated when status changes to 'approved'
    approved_at TIMESTAMP WITH TIME ZONE,
    
    -- Timestamps: standard audit fields for tracking record lifecycle
    -- created_at: automatically set when the record is inserted
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- updated_at: automatically updated on every modification via trigger
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create index on request_id for fast duplicate lookups
-- This index is crucial for the idempotency feature - O(1) lookup performance
CREATE INDEX idx_loan_requests_request_id ON loan_requests(request_id);

-- Create index on status for filtering by application status
-- Supports queries like "get all pending applications" efficiently
CREATE INDEX idx_loan_requests_status ON loan_requests(status);

-- Create index on created_at for temporal queries and ordering
-- Supports "get most recent applications" type queries
CREATE INDEX idx_loan_requests_created_at ON loan_requests(created_at DESC);

-- Create index on applicant_name for searching by name
-- Supports partial match searches (requires additional setup with pg_trgm)
CREATE INDEX idx_loan_requests_applicant_name ON loan_requests(applicant_name);

-- Create composite index for common query patterns: status + created_at
-- Optimizes queries like "get pending applications ordered by date"
CREATE INDEX idx_loan_requests_status_created ON loan_requests(status, created_at DESC);

-- Create trigger function to automatically update the updated_at timestamp
-- This ensures audit trail accuracy without manual updates in application code
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Attach the trigger to the loan_requests table
-- Fires BEFORE each UPDATE operation on any column
CREATE TRIGGER update_loan_requests_updated_at
    BEFORE UPDATE ON loan_requests
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Add comment to document the table purpose for future developers
COMMENT ON TABLE loan_requests IS 'Stores loan applications from the loan management platform. Each record represents a single loan request with applicant details, amount, term, and current processing status.';

-- Add comments to document critical columns
COMMENT ON COLUMN loan_requests.request_id IS 'Unique identifier for idempotency - prevents duplicate loan submissions from the same request';
COMMENT ON COLUMN loan_requests.amount IS 'Loan amount in platform currency - constrained to 100-10000 range per business rules';
COMMENT ON COLUMN loan_requests.term_months IS 'Repayment term in months - constrained to 6-60 range per business rules';
COMMENT ON COLUMN loan_requests.status IS 'Current processing status: pending, approved, rejected, cancelled, under_review';