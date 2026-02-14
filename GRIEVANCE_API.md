# Grievance System API Documentation

## Overview
The grievance system backend is fully implemented with comprehensive features for The Silent Scroll (Pillar II: Voice).

## Database Schema

### Tables Created
1. **grievances** - Main table storing all grievance submissions
2. **grievance_status_history** - Tracks all status changes with timestamps
3. **grievance_comments** - Comments and remarks on grievances
4. **grievance_upvotes** - User upvotes/support for grievances
5. **departments** - Department information for assignments

### Enums
- `grievance_category`: infrastructure, academics, hostel, food, other
- `grievance_priority`: low, medium, high, urgent
- `grievance_status`: submitted, under_review, in_progress, resolved, closed

## API Endpoints

### 1. Create Grievance
**POST** `/api/grievances`

**Body:**
```json
{
  "title": "string",
  "description": "string",
  "category": "infrastructure|academics|hostel|food|other",
  "priority": "low|medium|high|urgent",
  "location_type": "string (optional)",
  "location_details": "string (optional)",
  "is_anonymous": boolean
}
```

**Response:** Created grievance with anonymous identifier if anonymous

---

### 2. Get All Grievances (with Filters)
**GET** `/api/grievances?status=submitted&category=infrastructure&page=1&limit=20`

**Query Parameters:**
- `status`: Filter by status
- `category`: Filter by category
- `priority`: Filter by priority
- `assigned_to`: Filter by assigned user UUID
- `assigned_department`: Filter by department
- `submitted_by`: Filter by submitter UUID
- `search`: Search in title and description
- `page`: Page number (default: 1)
- `limit`: Items per page (default: 20, max: 100)

**Role-based visibility:**
- Students: See their own + all non-anonymous grievances
- Faculty: See assigned + all non-anonymous grievances
- Authority/Admin: See all grievances

---

### 3. Get Single Grievance
**GET** `/api/grievances/:id`

Returns full grievance details including submitter info (if not anonymous), assigned user, upvote status, etc.

---

### 4. Update Grievance Status (Authority/Admin only)
**PUT** `/api/grievances/:id/status`

**Body:**
```json
{
  "status": "submitted|under_review|in_progress|resolved|closed",
  "remarks": "string (optional)"
}
```

Automatically logs status change in history table.

---

### 5. Assign Grievance (Authority/Admin only)
**PUT** `/api/grievances/:id/assign`

**Body:**
```json
{
  "assigned_to": "uuid (optional)",
  "assigned_department": "string (optional)"
}
```

---

### 6. Resolve Grievance (Authority/Admin only)
**PUT** `/api/grievances/:id/resolve`

**Body:**
```json
{
  "resolution_notes": "string"
}
```

Sets status to 'resolved', adds resolution notes, and records resolver and timestamp.

---

### 7. Toggle Upvote
**POST** `/api/grievances/:id/upvote`

Adds upvote if not already upvoted, removes if already upvoted. Updates upvote count automatically.

---

### 8. Upload Photos
**POST** `/api/grievances/:id/photos`

**Content-Type:** `multipart/form-data`

**Form Fields:**
- `photos`: Multiple files allowed

Uploads to Cloudinary in `grievances/{grievance_id}/` folder. Appends new URLs to existing photo_urls array.

---

### 9. Get Status History
**GET** `/api/grievances/:id/history`

Returns chronological list of all status changes with remarks, updated_by user info, and timestamps.

---

### 10. Add Comment
**POST** `/api/grievances/:id/comments`

**Body:**
```json
{
  "comment": "string",
  "is_internal": boolean
}
```

**Note:** Only Authority/Admin can mark comments as internal. Internal comments are hidden from students.

---

### 11. Get Comments
**GET** `/api/grievances/:id/comments`

Returns all comments. Students only see public comments; Authority/Admin see all including internal notes.

---

### 12. Get Departments
**GET** `/api/departments`

Returns list of all departments for assignment dropdown.

---

### 13. Delete Grievance
**DELETE** `/api/grievances/:id`

Students can delete their own grievances. Admins can delete any grievance.

---

## Features Implemented

### Core Features ✅
- ✅ Anonymous and identified submissions
- ✅ Smart categorization (5 categories)
- ✅ Priority tagging (4 levels)
- ✅ Location tagging
- ✅ Photo upload with Cloudinary integration
- ✅ Submission history tracking
- ✅ Real-time status tracking with 5 states
- ✅ Authority remarks and resolution notes
- ✅ Complete timeline view (status history)
- ✅ Advanced filtering and search
- ✅ Assignment to authorities
- ✅ Department-based organization
- ✅ Upvote/support system
- ✅ Comment system with internal notes
- ✅ Role-based access control (RBAC)
- ✅ Audit logging for sensitive actions

### Security Features ✅
- ✅ Session-based authentication required
- ✅ Role-based permissions enforced
- ✅ Anonymous submissions protected
- ✅ Proper ownership validation
- ✅ SQL injection prevention (parameterized queries)

### Analytics Ready 📊
The database schema supports analytics queries for:
- Resolution time patterns
- Common issue categories
- Departmental response rates
- Priority distribution
- Upvote trends

## Next Steps (Optional Enhancements)

### Bonus Features (from requirements):
1. **Automated Intelligence**
   - Daily email summaries to department secretaries
   - High-priority alerts (72+ hours unresolved)
   - Analytics dashboard

2. **Notifications**
   - Email notifications on status changes
   - Push notifications for assigned authorities

3. **Advanced Analytics**
   - Dashboard showing pending/in-progress/resolved by department
   - Average resolution time by category
   - Most common issue types
   - Response rate tracking

## Database Migration

To apply the migration:
```bash
cd backend
sqlx migrate run
```

Or using your migration tool of choice with:
`backend/migrations/20260214000000_create_grievances_tables.sql`

## Environment Variables Required

```env
DATABASE_URL=postgresql://user:password@host/database
CLOUDINARY_CLOUD_NAME=your_cloud_name
CLOUDINARY_API_KEY=your_api_key
CLOUDINARY_API_SECRET=your_api_secret
```

## Testing Credentials

Create test users with different roles:
- **Student**: Can submit and view grievances
- **Faculty**: Can be assigned grievances
- **Authority**: Can update status, assign, and resolve
- **Admin**: Full access to all features

---

Built for **AEGIS Platform** - IIT Mandi Campus Management System
