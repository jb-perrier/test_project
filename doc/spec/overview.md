# Test Project Specification

## Overview

The test project manages a small in-memory task list and produces text reports.

## Current behavior

- Tasks have an id, title, status, priority, and optional due date.
- The report groups tasks by status.
- Closed tasks are still included in the report.
- There is no archive concept.

## Constraints

- Keep the project lightweight and dependency free.
- Preserve the simple in-memory design.
- Prefer small, explicit functions over framework-style abstractions.