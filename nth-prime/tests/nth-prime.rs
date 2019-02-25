use nth_prime as np;

#[test]
fn test_is_prime() {
    assert!(np::is_prime(0) == false);
    assert!(np::is_prime(1) == false);
    assert!(np::is_prime(2));
    assert!(np::is_prime(3));
    assert!(np::is_prime(4) == false);
    assert!(np::is_prime(5));
    assert!(np::is_prime(6) == false);
    assert!(np::is_prime(7));
    assert!(np::is_prime(8) == false);
    assert!(np::is_prime(9) == false);
    assert!(np::is_prime(10) == false);
    assert!(np::is_prime(11));
    assert!(np::is_prime(12) == false);
    assert!(np::is_prime(13));
    assert!(np::is_prime(14) == false);
}

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
fn test_big_prime() {
    assert_eq!(np::nth(10000), 104743);
}
