use pbkdf2::pbkdf2_hmac;
use sha2::Sha512;
use rand::RngCore;
use base64::{Engine as _, engine::general_purpose};

const SALT_LEN: usize = 16;
const ITERATIONS: u32 = 100_000;
const KEY_LEN: usize = 64;

pub fn hash_password(password: &str) -> String {
    // Generate random salt
    let mut salt = [0u8; SALT_LEN];
    rand::rng().fill_bytes(&mut salt);
    
    // Hash password using PBKDF2-HMAC-SHA512
    let mut hash = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha512>(
        password.as_bytes(),
        &salt,
        ITERATIONS,
        &mut hash,
    );
    
    // Combine salt and hash
    let mut combined = Vec::with_capacity(SALT_LEN + KEY_LEN);
    combined.extend_from_slice(&salt);
    combined.extend_from_slice(&hash);
    
    // Return base64 encoded result
    general_purpose::STANDARD.encode(combined)
}

pub fn compare_password(password: Option<&str>, hashed_password: Option<&str>) -> bool {
    let password = match password {
        Some(p) => p,
        None => return false,
    };
    
    let hashed_password = match hashed_password {
        Some(h) => h,
        None => return false,
    };
    
    // Decode base64
    let combined = match general_purpose::STANDARD.decode(hashed_password) {
        Ok(data) => data,
        Err(_) => return false,
    };
    
    // Check length
    if combined.len() != SALT_LEN + KEY_LEN {
        return false;
    }
    
    // Extract salt and stored hash
    let salt = &combined[0..SALT_LEN];
    let stored_hash = &combined[SALT_LEN..];
    
    // Hash the provided password with the extracted salt
    let mut computed_hash = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha512>(
        password.as_bytes(),
        salt,
        ITERATIONS,
        &mut computed_hash,
    );
    
    // Constant-time comparison
    use subtle::ConstantTimeEq;
    stored_hash.ct_eq(&computed_hash).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_and_compare() {
        let password = "test_password_123";
        let hashed = hash_password(password);
        
        // Should match with correct password
        assert!(compare_password(Some(password), Some(&hashed)));
        
        // Should not match with wrong password
        assert!(!compare_password(Some("wrong_password"), Some(&hashed)));
        
        // Should handle None inputs
        assert!(!compare_password(None, Some(&hashed)));
        assert!(!compare_password(Some(password), None));
        assert!(!compare_password(None, None));
    }
    
    #[test]
    fn test_invalid_hash_format() {
        let password = "test_password";
        
        // Invalid base64
        assert!(!compare_password(Some(password), Some("invalid_base64!")));
        
        // Wrong length
        let short_hash = general_purpose::STANDARD.encode(b"too_short");
        assert!(!compare_password(Some(password), Some(&short_hash)));
    }
    
    #[test]
    fn test_different_passwords_different_hashes() {
        let hash1 = hash_password("password1");
        let hash2 = hash_password("password1");
        
        // Same password should produce different hashes due to random salt
        assert_ne!(hash1, hash2);
        
        // But both should verify correctly
        assert!(compare_password(Some("password1"), Some(&hash1)));
        assert!(compare_password(Some("password1"), Some(&hash2)));
    }
}