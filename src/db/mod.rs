use crate::language::Translation;
use sqlx::SqlitePool;

const DB_URL: &str = "sqlite://sqlite.db";

#[derive(Debug, Clone)]
pub struct DB {
    pub pool: SqlitePool,
}

impl DB {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(DB_URL).await?;
        Ok(DB { pool })
    }
    pub async fn get_all_translations(&self) -> Result<Vec<Translation>, sqlx::Error> {
        let mut translations = vec![];
        let conn = &self.pool;
        let recs = sqlx::query!(
            r#"
            SELECT * FROM translations
            "#,
        )
        .fetch_all(conn)
        .await?;
        for rec in recs {
            translations.push(Translation {
                application: rec.application,
                page: rec.page,
                key: rec.key,
                lang: rec.lang,
                value: rec.value,
                context: rec.context,
            });
        }
        Ok(translations)
    }
    pub async fn get_translation_by_id(&self, id: i32) -> Result<Translation, sqlx::Error> {
        let conn = &self.pool;
        let rec = sqlx::query!(
            r#"
            SELECT * FROM translations WHERE id = $1
            "#,
            id
        )
        .fetch_one(conn)
        .await?;
        Ok(Translation {
            application: rec.application,
            page: rec.page,
            key: rec.key,
            lang: rec.lang,
            value: rec.value,
            context: rec.context,
        })
    }
}
