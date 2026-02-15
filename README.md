# AEGIS — The Unified Digital Citadel

Project repository for AEGIS: a unified campus platform implementing Identity & Governance, Grievance Management, Academic Mastery and Opportunity pillars with additional plugin features.

By - Taneshq Gupta & Uditya Yadav

---

## Project overview and problem context

AEGIS is built to unify campus services into a single web platform. It addresses fragmented communication, scattered academic resources, opaque grievance handling, and poor discovery of research/internship opportunities. The project provides role-based access for Students, Faculty, Authorities and Admins; a grievance tracking system; course management, enrollments and attendance; a searchable resource vault; and an opportunities/task system.

This repository contains a Rust/Axum backend and a Svelte frontend. It is deployed live at: https://aegis.taneshq.iitmandi.in.net (mobile-first and PWA-ready).

---

## Complete feature list (implemented)

- Pillar I — Identity & Governance
  - Role-based authentication via Google OAuth restricted to institute emails
  - Session handling (cookie session name: `aegis_session`) and RBAC in handlers
  - Admin endpoints for user management, audit logs and system stats

- Pillar II — Voice (Grievance Management)
  - Create grievances (anonymous or identified) with category, priority, location and photo upload
  - Status workflow (submitted → under_review → in_progress → resolved), comments, history and upvotes
  - Admin/Authority APIs to assign, update status, resolve and fetch analytics

- Pillar III — Fate (Academic Mastery)
  - Courses table and endpoints: create, list, filter, course details
  - Student enrollment and `my-enrollments` API
  - Attendance: student-accessible personal attendance logs and a `mark` endpoint to insert/update attendance
  - Academic resources (Vault): upload, list per-course resources
  - Calendar/events: create and personalized calendar view

- Pillar IV — Opportunity (Internships & Tasks)
  - Basic opportunity posting, application endpoints, and personal tasks APIs implemented in backend

- Additional/bonus items found in codebase
  - Cloudinary integration helper for image uploads (profile/resources)
  - Audit logging on login/logout and admin actions

---

## Technology stack

- Backend: Rust with Axum framework, sqlx (Postgres), tower-sessions for session handling.
  - Justification: Rust + Axum gives a small, fast, type-safe HTTP API server with strong compile-time guarantees.
- Database: PostgreSQL (migrations present in `/backend/migrations`).
  - Uses UUID primary keys and ENUM types for domain constraints (roles, statuses, course types, attendance, etc.).
- Frontend: Svelte + Vite, with a PWA-capable frontend and Svelte stores for auth.
  - Justification: lightweight reactive UI, fast dev feedback and small bundle sizes suitable for mobile/PWA.
- Hosting: Live site at https://aegis.taneshq.iitmandi.in.net (as provided).

---

## Repository layout (high-level)

- `backend/` — Rust Axum server
  - `src/main.rs` — router and middleware wiring (see API routes below)
  - `src/academic.rs` — course, enrollment, attendance, resources, events handlers
  - `src/auth.rs` — Google OAuth flow (`google_login_initiate`, `google_callback`), session setup
  - `src/admin.rs` — admin utilities (user list, role/status update, stats)
  - `src/structs.rs` — shared DTOs, enums and DB entities used across backend
  - `src/cloudinary.rs` — Cloudinary upload helper
  - `backend/migrations/` — SQL migration files (create users, grievances, courses/attendance/resources/events, opportunities, tasks)

- `frontend/` — Svelte app
  - `src/lib/api.ts` — generic `api` helper that uses `PUBLIC_BACKEND_URL` and `credentials: 'include'`
  - `src/lib/types.ts` — TypeScript types mirroring backend DTOs
  - `src/lib/auth.ts` — auth store using `checkAuth` and populating `user` writable store
  - `src/routes/...` — Svelte pages for dashboard, grievances, courses, attendance, auth flows and admin pages

---

## Database schema (summary)

The SQL migrations and `structs.rs` define the schema. Key tables and enums:

- Enums (short list)
  - `user_role` — ('student', 'faculty', 'authority', 'admin')
  - `user_status` — ('active', 'inactive', 'suspended')
  - `grievance_category`, `grievance_priority`, `grievance_status`
  - `course_type` — ('core', 'elective', 'major', 'minor')
  - `resource_type` — ('pyq', 'notes', 'lecture', 'assignment')
  - `event_type` — ('exam', 'deadline', 'holiday', 'class')
  - `attendance_status` — ('present', 'absent', 'cancelled')

- Primary tables (from migrations)
  - `users` — identity, role, profile, department, roll, timestamps (`backend/migrations/20260213192811_create_users_and_auth_tables.sql`)
  - `grievances`, `grievance_comments`, `grievance_status_history`, `grievance_upvotes` (`../20260214000000_create_grievances_tables.sql`)
  - `courses`, `course_enrollments` (`backend/migrations/20260214193222_third_pillar.sql`)
  - `attendance_logs` — linked to `course_enrollments`
  - `academic_resources` — vault entries with `tags` and `is_verified`
  - `academic_events` — course/global events calendar
  - `opportunities`, `applications`, `personal_tasks` (Pillar IV migrations)

Schema approach notes:
  - `UUID` primary keys used across tables with `gen_random_uuid()` defaults.
  - Timestamps stored as `TIMESTAMPTZ` and `update_updated_at()` trigger used for `updated_at`.
  - Referential integrity via `REFERENCES users(id)` and cascade rules for deletions.

---

## Backend API (important endpoints)

Router wiring is in `backend/src/main.rs`. Notable endpoints:

- Auth
  - `GET /auth/google` — begin Google OAuth
  - `GET /auth/google/callback` — OAuth callback (creates user if new)
  - `GET /auth/logout` — clear session
  - `GET /auth/me` — current user (used by frontend `checkAuth`)

- Grievances
  - `POST /api/grievances` — create grievance
  - `GET /api/grievances` — list grievances with filters
  - `GET /api/grievances/{id}` — get grievance by id
  - `DELETE /api/grievances/{id}` — delete (admin/owner rules applied)
  - `PUT /api/grievances/{id}/status` — update status
  - `PUT /api/grievances/{id}/assign` — assign to user/department
  - `POST /api/grievances/{id}/comments` — add comment
  - `POST /api/grievances/{id}/upvote` — toggle upvote

- Courses & Academic
  - `POST /api/courses` — create course (faculty/admin)
  - `GET /api/courses` — list/filter courses
  - `POST /api/courses/enroll` — enroll current student in course
  - `GET /api/courses/my-enrollments` — list current student's courses
  - `GET /api/courses/{id}` — course details and enrollment stats

- Attendance
  - `POST /api/attendance/mark` — mark attendance (student can mark own; faculty may mark for student)
  - `GET /api/attendance/{id}` — get the current student's attendance summary for course `{id}`

- Resources and Events
  - `POST /api/courses/{id}/resources` and `GET /api/courses/{id}/resources`
  - `POST /api/events` and `GET /api/events` (personalized calendar)

- Admin & Dashboard
  - `GET /api/admin/users` — list users (admin-only) and more admin endpoints in `admin.rs`
  - Dev helper: `PUT /api/user/role` and `POST /api/dev/seed` (dev/testing only)

Implementation notes:
  - Handlers use `tower_sessions::Session` to get `user_id` from session and then load `User` from DB.
  - API responses wrapped in `ApiResponse<T>` with fields `success`, `data`, `message`.
  - `sqlx` is used with `FromRow` types defined in `structs.rs`.

---

## Frontend notes and wiring

- API helper
  - `src/lib/api.ts` constructs requests using `PUBLIC_BACKEND_URL` env var and `credentials: 'include'` so the server session cookie is sent.

- Auth
  - `src/lib/auth.ts` checks `auth/me` on load and sets a Svelte `user` writable store.

- Types
  - `src/lib/types.ts` mirrors backend DTOs: `Course`, `AttendanceLog`, `AttendanceSummary`, `UserResponse`, etc.

- Relevant student-facing pages found in `frontend/src/routes` (examples from code):
  - `routes/dashboard/student/courses/+page.svelte` — lists courses, enroll action and credit calculator
  - `routes/dashboard/student/courses/attendance/+page.svelte` — per-course attendance summary and logs

UX notes:
  - The UI is mobile-first and PWA-friendly (manifest/icons in `static/pwa-icons`).
  - Course enrollment flow: frontend calls `POST /api/courses/enroll` then refreshes `my-enrollments`.
  - Attendance flow: frontend fetches `GET /api/attendance/{course_id}` and posts marks to `POST /api/attendance/mark`.

---

## Environment variables (runtime)

The backend expects several env vars. Key ones found in the code:

- `DATABASE_URL` — Postgres connection string
- `FRONTEND_URL` — frontend origin (used in OAuth redirects)
- `PORT` — port for backend server (default 8000)
- `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`, `GOOGLE_REDIRECT_URL` — Google OAuth
- `CLOUDINARY_CLOUD_NAME`, `CLOUDINARY_API_KEY`, `CLOUDINARY_API_SECRET` — if Cloudinary uploads are used

Frontend build runtime expects `PUBLIC_BACKEND_URL` to point to the backend origin.

Session cookie: the server uses `aegis_session` (configured in `main.rs` with `tower_sessions`).

---

## How to run?

Visit the live link: https://aegis.taneshq.iitmandi.in.net

---

## Setup & installation (developer notes)

This section documents how to run and develop locally (for contributors). It is intentionally actionable and concise.

1. Prerequisites
   - Rust toolchain (recommended: stable compatible with `rust-toolchain.toml` in `backend/`)
   - Node.js (for the frontend)
   - PostgreSQL with `pgcrypto`/`pgcrypto` extension (for `gen_random_uuid()`)

2. Back-end (development)
   - Create a Postgres database and set `DATABASE_URL` accordingly.
   - Ensure migrations under `backend/migrations` are applied (use `psql` or a migration runner).
   - Set OAuth and Cloudinary env vars if testing those flows.
   - Run the server from `backend/`:

```bash
cd backend
export DATABASE_URL="postgres://..."
export FRONTEND_URL="http://localhost:4173"
export GOOGLE_CLIENT_ID=...
export GOOGLE_CLIENT_SECRET=...
export GOOGLE_REDIRECT_URL=http://localhost:8000/auth/google/callback
cargo run
```

3. Front-end (development)

```bash
cd frontend
# set PUBLIC_BACKEND_URL in your .env or environment
export PUBLIC_BACKEND_URL="http://localhost:8000/"
npm install
npm run dev
```

Notes:
- The front-end `api.ts` will call `PUBLIC_BACKEND_URL + path`. Using `credentials: 'include'` ensures the backend session cookie is sent.
- For OAuth testing, redirect URIs and `FRONTEND_URL` must match settings in Google Cloud Console.

---

## Known limitations and future scope

Known limitations (from code review):
- Some SQL string composition uses direct interpolation for filters (e.g., `format!(" AND semester = '{}'", semester)` in `get_courses`) — in production this should use bound parameters to avoid SQL injection risk.
- No explicit rate limiting or brute-force protections in current server code.
- Email notifications/alerts (bonus features like daily summaries and high-priority alerts) are not implemented (placeholders only).
- Some admin checks and role restrictions are permissive in places for developer convenience.

Future scope / suggested improvements:
- Harden SQL query parameterization across handlers.
- Add automated tests (unit + integration) for core endpoints.
- Add background job worker (for daily digests / alerts) and an actionable notification system.
- Enhance calendar notifications (push, email reminders) and implement user settings for reminders.
- Add an admin UI for verifying `academic_resources` uploaded by students.
- Improve file upload flow to handle large files, virus scanning and upload progress.

---

## Contribution guidelines

- Fork the repo, implement features on a feature branch and open PRs with clear descriptions.
- Keep commits small and focused; run `cargo fmt` and `npm run build` as applicable.

---

