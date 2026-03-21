use sndv_scalpel::transaction::Transaction;

#[tokio::test]
async fn rollback_after_write_failure_critical_path() {
    let temp = tempfile::tempdir().expect("tempdir");
    let file_path = temp.path().join("target.txt");
    std::fs::write(&file_path, "sum = 1\n").expect("write baseline");

    let tx = Transaction::begin(std::slice::from_ref(&file_path)).await.expect("begin transaction");

    let write_err = Transaction::atomic_write(temp.path(), "bad write").await;
    assert!(write_err.is_err());

    tx.rollback().await.expect("rollback");
    tx.cleanup().await.expect("cleanup");

    let restored = std::fs::read_to_string(file_path).expect("read restored");
    assert_eq!(restored, "sum = 1\n");
}
