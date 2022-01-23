use std::fmt::Display;
use std::time::{Duration, Instant};

use aes::Aes128;
use block_modes::block_padding::ZeroPadding;
use block_modes::{BlockMode, BlockModeError, Cbc};
use rayon::prelude::*;

use crate::key_space::Key;

type Aes128Cbc = Cbc<Aes128, ZeroPadding>;

#[derive(Debug)]
pub struct SearchResult {
    valid_keys: Vec<Key>,
    took: Duration,
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Key space search took {:?} and found {} candidate keys: {:#?}",
            self.took,
            self.valid_keys.len(),
            self.valid_keys
                .iter()
                .map(hex::encode_upper)
                .collect::<Vec<String>>(),
        )
    }
}

pub fn search_key_space<K>(key_space: &K) -> SearchResult
where
    K: IntoParallelIterator<Item = Key> + Clone,
{
    let start = Instant::now();

    let valid_keys = key_space
        .clone()
        .into_par_iter()
        .filter(is_key_valid)
        .collect();

    let took = Instant::now().saturating_duration_since(start);

    SearchResult { valid_keys, took }
}

const TEST_TELEGRAMS: [[u8; 63]; 3] = [
    hex_literal::hex!("3E44A5112655687276077A39003005EBAEEB906AE817B45D3CC6B46A955BAD34DEA47B00F860ACBB6D280069A227B334CEE23006878125BBAD10EDADAD9635"),
    hex_literal::hex!("3E44A5112655687276077A8DD33005C20A86D323D5AD1A25E3BA4E64427E7A1612693F8CFDAC4B79745E68F41BDC99B936F1A8122449061BB9B1D4BECD9406"),
    hex_literal::hex!("3E44A5112655687276077A6ED330055C1B9AB21431F33C04BBE4741DCE827E6849F8407FFCDFB7EFB0262CC350CE8AD13A7B2DE5BE281C5896B6D4E06FDC3A"),
];

const DECRYPTED_PAYLOAD_INDICATOR: [u8; 2] = [0x2f, 0x2f];

fn is_key_valid(key: &Key) -> bool {
    TEST_TELEGRAMS.iter().all(|telegram| {
        decrypt_telegram(telegram, key)
            .as_ref()
            .map(|payload| is_decrypted_payload(payload))
            .unwrap_or(false)
    })
}

fn decrypt_telegram(telegram: &[u8], key: &Key) -> Result<Vec<u8>, BlockModeError> {
    let manufacturer = &telegram[2..4];
    let address = &telegram[4..10];
    let access_number = &telegram[11..12];
    let encrypted_payload = &telegram[15..];
    let iv = [manufacturer, address, access_number.repeat(8).as_slice()].concat();

    let cipher = Aes128Cbc::new_from_slices(key, &iv).unwrap();

    cipher.decrypt_vec(encrypted_payload)
}

fn is_decrypted_payload(payload: &[u8]) -> bool {
    payload.starts_with(&DECRYPTED_PAYLOAD_INDICATOR)
}
