#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    #[serde(rename = "student")]
    Student,
    #[serde(rename = "faculty")]
    Faculty,
    #[serde(rename = "authority")]
    Authority,
    #[serde(rename = "admin")]
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "suspended")]
    Suspended,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub google_id: String,
    pub role: UserRole,
    pub status: UserStatus,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: Option<String>,
    pub roll_number: Option<String>,
    pub batch_year: Option<i32>,
    pub program: Option<String>,
    pub department: Option<String>,
    pub employee_id: Option<String>,
    pub designation: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: Option<String>,
    pub department: Option<String>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            role: user.role,
            first_name: user.first_name,
            last_name: user.last_name,
            profile_picture: user.profile_picture,
            department: user.department,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: String,
    pub given_name: String,
    pub family_name: Option<String>,
    pub picture: Option<String>,
}

// ============================================================================
// GRIEVANCE SYSTEM STRUCTS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "grievance_category", rename_all = "lowercase")]
pub enum GrievanceCategory {
    #[serde(rename = "infrastructure")]
    Infrastructure,
    #[serde(rename = "academics")]
    Academics,
    #[serde(rename = "hostel")]
    Hostel,
    #[serde(rename = "food")]
    Food,
    #[serde(rename = "other")]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "grievance_priority", rename_all = "lowercase")]
pub enum GrievancePriority {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "urgent")]
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "grievance_status", rename_all = "snake_case")]
pub enum GrievanceStatus {
    #[serde(rename = "submitted")]
    Submitted,
    #[serde(rename = "under_review")]
    UnderReview,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "closed")]
    Closed,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Grievance {
    pub id: Uuid,
    pub submitted_by: Option<Uuid>,
    pub is_anonymous: bool,
    pub anonymous_identifier: Option<String>,
    pub title: String,
    pub description: String,
    pub category: GrievanceCategory,
    pub priority: GrievancePriority,
    pub status: GrievanceStatus,
    pub location_type: Option<String>,
    pub location_details: Option<String>,
    pub photo_urls: Option<Vec<String>>,
    pub assigned_to: Option<Uuid>,
    pub assigned_department: Option<String>,
    pub resolution_notes: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<Uuid>,
    pub view_count: i32,
    pub upvote_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct GrievanceResponse {
    pub id: Uuid,
    pub submitter: Option<UserResponse>, // Only included if not anonymous
    pub is_anonymous: bool,
    pub title: String,
    pub description: String,
    pub category: GrievanceCategory,
    pub priority: GrievancePriority,
    pub status: GrievanceStatus,
    pub location_type: Option<String>,
    pub location_details: Option<String>,
    pub photo_urls: Vec<String>,
    pub assigned_to: Option<UserResponse>,
    pub assigned_department: Option<String>,
    pub resolution_notes: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub view_count: i32,
    pub upvote_count: i32,
    pub user_has_upvoted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGrievanceRequest {
    pub title: String,
    pub description: String,
    pub category: GrievanceCategory,
    pub priority: GrievancePriority,
    pub location_type: Option<String>,
    pub location_details: Option<String>,
    pub is_anonymous: bool,
    // Photo uploads will be handled separately via multipart form
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UpdateGrievanceRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<GrievanceCategory>,
    pub priority: Option<GrievancePriority>,
    pub location_type: Option<String>,
    pub location_details: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGrievanceStatusRequest {
    pub status: GrievanceStatus,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AssignGrievanceRequest {
    pub assigned_to: Option<Uuid>,
    pub assigned_department: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResolveGrievanceRequest {
    pub resolution_notes: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct GrievanceStatusHistory {
    pub id: Uuid,
    pub grievance_id: Uuid,
    pub old_status: Option<GrievanceStatus>,
    pub new_status: GrievanceStatus,
    pub remarks: Option<String>,
    pub updated_by: Option<Uuid>,
    pub updated_by_role: Option<UserRole>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct GrievanceStatusHistoryResponse {
    pub id: Uuid,
    pub old_status: Option<GrievanceStatus>,
    pub new_status: GrievanceStatus,
    pub remarks: Option<String>,
    pub updated_by: Option<UserResponse>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct GrievanceComment {
    pub id: Uuid,
    pub grievance_id: Uuid,
    pub user_id: Uuid,
    pub comment: String,
    pub is_internal: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct GrievanceCommentResponse {
    pub id: Uuid,
    pub user: UserResponse,
    pub comment: String,
    pub is_internal: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub comment: String,
    pub is_internal: bool,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Department {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub head_user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct GrievanceFilters {
    pub status: Option<GrievanceStatus>,
    pub category: Option<GrievanceCategory>,
    pub priority: Option<GrievancePriority>,
    pub assigned_to: Option<Uuid>,
    pub assigned_department: Option<String>,
    #[allow(dead_code)]
    pub submitted_by: Option<Uuid>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

// ============================================================================
// ADMIN USER MANAGEMENT STRUCTS
// ============================================================================

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UpdateUserRoleRequest {
    pub role: UserRole,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UpdateUserStatusRequest {
    pub status: UserStatus,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub status: UserStatus,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: Option<String>,
    pub department: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserListResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            role: user.role,
            status: user.status,
            first_name: user.first_name,
            last_name: user.last_name,
            profile_picture: user.profile_picture,
            department: user.department,
            last_login_at: user.last_login_at,
            created_at: user.created_at,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct AuditLogResponse {
    pub id: Uuid,
    pub user: Option<UserResponse>,
    pub action: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UserFilters {
    pub role: Option<UserRole>,
    pub status: Option<UserStatus>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

// ============================================================================
// ACADEMIC SYSTEM STRUCTS (PILLAR III)
// ============================================================================

// --- Enums ---

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "course_type", rename_all = "lowercase")]
pub enum CourseType {
    #[serde(rename = "core")]
    Core,
    #[serde(rename = "elective")]
    Elective,
    #[serde(rename = "major")]
    Major,
    #[serde(rename = "minor")]
    Minor,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "resource_type", rename_all = "lowercase")]
pub enum ResourceType {
    #[serde(rename = "pyq")]
    Pyq,
    #[serde(rename = "notes")]
    Notes,
    #[serde(rename = "lecture")]
    Lecture,
    #[serde(rename = "assignment")]
    Assignment,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "event_type", rename_all = "lowercase")]
pub enum EventType {
    #[serde(rename = "exam")]
    Exam,
    #[serde(rename = "deadline")]
    Deadline,
    #[serde(rename = "holiday")]
    Holiday,
    #[serde(rename = "class")]
    Class,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "attendance_status", rename_all = "lowercase")]
pub enum AttendanceStatus {
    #[serde(rename = "present")]
    Present,
    #[serde(rename = "absent")]
    Absent,
    #[serde(rename = "cancelled")]
    Cancelled,
}

// --- Database Entities ---

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Course {
    pub id: Uuid,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub credits: i32,
    pub department: String,
    pub course_type: CourseType,
    pub instructor_id: Option<Uuid>,
    pub semester: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct CourseEnrollment {
    pub id: Uuid,
    pub student_id: Uuid,
    pub course_id: Uuid,
    pub enrolled_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AttendanceLog {
    pub id: Uuid,
    pub enrollment_id: Uuid,
    pub date: chrono::NaiveDate, // Uses NaiveDate for DATE types
    pub status: AttendanceStatus,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AcademicResource {
    pub id: Uuid,
    pub course_id: Uuid,
    pub uploaded_by: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub resource_type: ResourceType,
    pub file_url: String,
    pub year: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AcademicEvent {
    pub id: Uuid,
    pub course_id: Option<Uuid>, // Nullable for global events
    pub created_by: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub event_type: EventType,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- API Response DTOs ---

#[derive(Debug, Serialize)]
pub struct CourseResponse {
    pub id: Uuid,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub credits: i32,
    pub department: String,
    pub course_type: CourseType,
    pub instructor: Option<UserResponse>, // Expanded user object
    pub semester: String,
}

#[derive(Debug, Serialize)]
pub struct AcademicResourceResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub resource_type: ResourceType,
    pub file_url: String,
    pub uploaded_by: Option<UserResponse>, // Expanded user
    pub year: Option<i32>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AcademicEventResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub event_type: EventType,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub course_code: Option<String>, // Helpful context if it's a course event
    pub course_title: Option<String>,
}

// --- API Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateCourseRequest {
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub credits: i32,
    pub department: String,
    pub course_type: CourseType,
    pub semester: String,
    pub instructor_email: Option<String>, // Easier to bind by email than UUID from frontend
}

#[derive(Debug, Deserialize)]
pub struct CreateResourceRequest {
    pub title: String,
    pub description: Option<String>,
    pub resource_type: ResourceType,
    pub file_url: String, // Likely returned from an upload handler first
    pub year: Option<i32>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: Option<String>,
    pub event_type: EventType,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub course_id: Option<Uuid>, // Optional: if null, it's a global/personal event
}

#[derive(Debug, Deserialize)]
pub struct LogAttendanceRequest {
    pub date: chrono::NaiveDate,
    pub status: AttendanceStatus,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CourseFilter {
    pub semester: Option<String>,
    pub department: Option<String>,
    pub search: Option<String>,
}