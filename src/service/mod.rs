use crate::db::DbPool;
use crate::model::{PhoneVerification, CreateVerificationRequest};
use sqlx::Error as SqlxError;
use chrono::Utc;

pub struct VerificationService {
    pool: DbPool,
}

impl VerificationService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // 修改：判断用户名是否存在，存在则更新，不存在则新增
    pub async fn create_or_update_verification(
        &self,
        request: CreateVerificationRequest,
    ) -> Result<PhoneVerification, SqlxError> {
        // 先检查用户名是否存在
        let existing = sqlx::query_as::<_, PhoneVerification>(
            r#"
            SELECT id, phone, username, verification_code, created_at, updated_at
            FROM phone_verifications
            WHERE username = ?
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
            .bind(&request.username)
            .fetch_optional(&self.pool)
            .await?;

        match existing {
            Some(mut record) => {
                // 用户名存在，更新验证码和手机号
                sqlx::query(
                    r#"
                    UPDATE phone_verifications
                    SET verification_code = ?, phone = ?, updated_at = ?
                    WHERE id = ?
                    "#,
                )
                    .bind(&request.verification_code)
                    .bind(&request.phone)
                    .bind(Utc::now())
                    .bind(&record.id)
                    .execute(&self.pool)
                    .await?;

                // 更新返回的记录
                record.verification_code = request.verification_code;
                record.phone = request.phone;
                record.updated_at = Some(Utc::now());

                Ok(record)
            }
            None => {
                // 用户名不存在，创建新记录
                let verification = PhoneVerification::new(
                    request.phone,
                    request.username,
                    request.verification_code,
                );

                sqlx::query(
                    r#"
                    INSERT INTO phone_verifications (id, phone, username, verification_code)
                    VALUES (?, ?, ?, ?)
                    "#,
                )
                    .bind(&verification.id)
                    .bind(&verification.phone)
                    .bind(&verification.username)
                    .bind(&verification.verification_code)
                    .execute(&self.pool)
                    .await?;

                Ok(verification)
            }
        }
    }

    // 保留原有的创建方法（可选）
    #[allow(dead_code)]
    pub async fn create_verification(
        &self,
        request: CreateVerificationRequest,
    ) -> Result<PhoneVerification, SqlxError> {
        let verification = PhoneVerification::new(
            request.phone,
            request.username,
            request.verification_code,
        );

        sqlx::query(
            r#"
            INSERT INTO phone_verifications (id, phone, username, verification_code)
            VALUES (?, ?, ?, ?)
            "#,
        )
            .bind(&verification.id)
            .bind(&verification.phone)
            .bind(&verification.username)
            .bind(&verification.verification_code)
            .execute(&self.pool)
            .await?;

        Ok(verification)
    }

    pub async fn get_verifications(&self) -> Result<Vec<PhoneVerification>, SqlxError> {
        let verifications = sqlx::query_as::<_, PhoneVerification>(
            r#"
            SELECT id, phone, username, verification_code, created_at, updated_at
            FROM phone_verifications
            ORDER BY created_at DESC
            "#,
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(verifications)
    }

    pub async fn get_phone_by_username(&self, username: &str) -> Result<Option<String>, SqlxError> {
        let record = sqlx::query_as::<_, (String,)>(
            r#"
            SELECT phone
            FROM phone_verifications
            WHERE username = ?
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(record.map(|(phone,)| phone))
    }

    pub async fn get_verifications_by_username(
        &self,
        username: &str
    ) -> Result<Vec<PhoneVerification>, SqlxError> {
        let verifications = sqlx::query_as::<_, PhoneVerification>(
            r#"
            SELECT id, phone, username, verification_code, created_at, updated_at
            FROM phone_verifications
            WHERE username = ?
            ORDER BY created_at DESC
            "#,
        )
            .bind(username)
            .fetch_all(&self.pool)
            .await?;

        Ok(verifications)
    }
}