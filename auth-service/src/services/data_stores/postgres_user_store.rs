// use std::error::Error;

// use argon2::{
//     password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
//     PasswordVerifier, Version,
// };

// use sqlx::PgPool;

// use crate::domain::{
//     data_stores::{UserStore, UserStoreError},
//     Email, Password, User,
// };

// pub struct PostgresUserStore {
//     pool: PgPool,
// }

// impl PostgresUserStore {
//     pub fn new(pool: PgPool) -> Self {
//         Self { pool }
//     }
// }

// #[async_trait::async_trait]
// impl UserStore for PostgresUserStore {
//     async fn add_user(&self, user: User) -> Result<(), UserStoreError> {
//         let query = "INSERT INTO users (email, password) VALUES ($1, $2)";
//         let email = user.email.to_string();
//         let password = compute_password_hash(&user.password)?;

//         sqlx::query(query)
//             .bind(email)
//             .bind(password)
//             .execute(&self.pool)
//             .await
//             .map_err(|e| UserStoreError::DatabaseError(e.to_string()))?;

//         Ok(())
//     }

//     async fn get_user(&self, email: &Email) -> Result<Option<User>, UserStoreError> {
//         let query = "SELECT email, password FROM users WHERE email = $1";
//         let email = email.to_string();

//         let row = sqlx::query(query)
//             .bind(email)
//             .fetch_optional(&self.pool)
//             .await
//             .map_err(|e| UserStoreError::DatabaseError(e.to_string()))?;

//         match row {
//             Some(row) => {
//                 let email: String = row.get(0);
//                 let password: String = row.get(1);

//                 let user = User {
//                     email: Email::new(email)?,
//                     password: password,
//                     requires_2fa: false,
//                 };

//                 Ok(Some(user))
//             }
//             None => Ok(None),
//         }
//     }

//     async fn validate_user(&self, email: &Email, password: &Password) -> Result<bool, UserStoreError> {
//         let query = "SELECT password FROM users WHERE email = $1";
//         let email = email.to_string();

//         let row = sqlx::query(query)
//             .bind(email)
//             .fetch_optional(&self.pool)
//             .await
//             .map_err(|e| UserStoreError::DatabaseError(e.to_string()))?;

//         match row {
//             Some(row) => {
//                 let password_hash: String = row.get(0);

//                 verify_password_hash(&password_hash, &password)
//                     .map_err(|e| UserStoreError::HashingError(e.to_string()))?;

//                 Ok(true)
//             }
//             None => Ok(false),
//         }
//     }
// }

// // Helper function to verify if a given password matches an expected hash
// // TODO: Hashing is a CPU-intensive operation. To avoid blocking
// // other async tasks, update this function to perform hashing on a
// // separate thread pool using tokio::task::spawn_blocking. Note that you
// // will need to update the input parameters to be String types instead of &str
// fn verify_password_hash(
//     expected_password_hash: &str,
//     password_candidate: &str,
// ) -> Result<(), Box<dyn Error>> {
//     let expected_password_hash: PasswordHash<'_> = PasswordHash::new(expected_password_hash)?;

//     Argon2::default()
//         .verify_password(password_candidate.as_bytes(), &expected_password_hash)
//         .map_err(|e| e.into())
// }

// // Helper function to hash passwords before persisting them in the database.
// // TODO: Hashing is a CPU-intensive operation. To avoid blocking
// // other async tasks, update this function to perform hashing on a
// // separate thread pool using tokio::task::spawn_blocking. Note that you
// // will need to update the input parameters to be String types instead of &str
// fn compute_password_hash(password: &str) -> Result<String, Box<dyn Error>> {
//     let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
//     let password_hash = Argon2::new(
//         Algorithm::Argon2id,
//         Version::V0x13,
//         Params::new(15000, 2, 1, None)?,
//     )
//     .hash_password(password.as_bytes(), &salt)?
//     .to_string();

//     Ok(password_hash)
// }