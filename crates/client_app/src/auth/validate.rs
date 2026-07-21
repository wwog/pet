/// 表单校验(规则与后端 handler.rs 一致)

pub fn validate_account(s: &str) -> Result<(), String> {
    if s.len() < 6 {
        return Err("账号至少6位".into());
    }
    if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err("账号仅允许字母、数字、下划线".into());
    }
    Ok(())
}

pub fn validate_password(s: &str) -> Result<(), String> {
    if s.len() < 8 {
        return Err("密码至少8位".into());
    }
    let has_alpha = s.chars().any(|c| c.is_ascii_alphabetic());
    let has_digit = s.chars().any(|c| c.is_ascii_digit());
    if !has_alpha || !has_digit {
        return Err("密码需含字母和数字".into());
    }
    Ok(())
}

pub fn validate_nickname(s: &str) -> Result<(), String> {
    let len = s.chars().count();
    if len < 1 || len > 20 {
        return Err("昵称1-20字符".into());
    }
    Ok(())
}

pub fn validate_confirm(pw: &str, confirm: &str) -> Result<(), String> {
    if pw != confirm {
        return Err("两次密码不一致".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_too_short() {
        assert!(validate_account("abc").is_err());
    }

    #[test]
    fn account_valid() {
        assert!(validate_account("alice_01").is_ok());
    }

    #[test]
    fn account_invalid_char() {
        assert!(validate_account("alice-01").is_err());
    }

    #[test]
    fn password_too_short() {
        assert!(validate_password("ab1").is_err());
    }

    #[test]
    fn password_no_digit() {
        assert!(validate_password("abcdefgh").is_err());
    }

    #[test]
    fn password_valid() {
        assert!(validate_password("alice123").is_ok());
    }

    #[test]
    fn nickname_empty() {
        assert!(validate_nickname("").is_err());
    }

    #[test]
    fn nickname_too_long() {
        assert!(validate_nickname(&"a".repeat(21)).is_err());
    }

    #[test]
    fn nickname_valid() {
        assert!(validate_nickname("豆豆").is_ok());
    }

    #[test]
    fn confirm_mismatch() {
        assert!(validate_confirm("abc12345", "abc12346").is_err());
    }

    #[test]
    fn confirm_match() {
        assert!(validate_confirm("abc12345", "abc12345").is_ok());
    }
}
