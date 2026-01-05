# banking QR Code
Content:
```
BCD
002
2
SCT
$BIC
$EMPFÄNGERNAME
$IBAN
$CURRENCY$AMOUNT

$PURPOSE (max 35chars -> needs break)
$ADDITIONALPURPOSE (max 140chars)
```

* with typst: https://github.com/Midbin/cades
* wist Rust (how to integrate logo?):
  * https://docs.rs/qrcode/latest/qrcode/
  * https://docs.rs/rqr/latest/rqr/
  * https://github.com/houseme/qrcode-rs
  * https://github.com/erwanvivien/fast_qr
  * https://lib.rs/crates/qr_code
  * https://github.com/sebastienrousseau/qrc
  * https://crates.io/crates/qrcode-generator