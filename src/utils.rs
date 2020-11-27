use ring::hmac::{self};
use data_encoding::HEXLOWER;

pub fn verify_webhook_signature(data: &str, signature: &str, secret: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let expected_signature = hmac::sign(&key, data.as_bytes());
    let hex_expected_signature = HEXLOWER.encode(expected_signature.as_ref());

    if hex_expected_signature == signature {
        Ok(())
    } else {
        Err(From::from("Signature do not match with expected value"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_hmac_signature() {
        setup();
        let secret = "supersecret";
        let signature = "e279c1f4cbe963b535a4fd0d54332f2d4aa0b76bb9abdb61dd1391c4804817df";
        let data = "asdf";
        let result = verify_webhook_signature(data, signature, &secret).ok();
        assert_eq!(result, Some(()));
    }

    fn setup() {
        dotenv::dotenv().ok();
        let _ = env_logger::builder().is_test(true).try_init();
    }
}
