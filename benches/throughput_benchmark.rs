use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ed25519_dalek::{SigningKey};
use rand::{rngs::OsRng, RngCore};
use owami_network::Transaction;

fn transaction_signing_benchmark(c: &mut Criterion) {
    let mut rng = OsRng;
    let mut sender_seed = [0u8; 32];
    rng.fill_bytes(&mut sender_seed);
    let sender = SigningKey::from_bytes(&sender_seed);

    let mut recipient_seed = [0u8; 32];
    rng.fill_bytes(&mut recipient_seed);
    let recipient = SigningKey::from_bytes(&recipient_seed);

    c.bench_function("sign 1000 transactions", |b| b.iter(|| {
        for _ in 0..1000 {
            let mut tx = Transaction::new_transfer(
                hex::encode(sender.verifying_key().to_bytes()),
                hex::encode(recipient.verifying_key().to_bytes()),
                1,
                1,
                0
            );
            tx.sign(&sender);
            black_box(tx);
        }
    }));
}

criterion_group!(benches, transaction_signing_benchmark);
criterion_main!(benches);