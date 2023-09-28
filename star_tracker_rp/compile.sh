(cd ../star_tracker_nix; cargo run --bin gen_database ../star_tracker_rp/lib/flash/database.rs ../star_tracker_rp/database.json)
(cd ../star_tracker_nix; cargo run --bin gen_image ../star_tracker_rp/lib/flash/img.rs 400 400 samples/16mm_checker_2/Mula/mula\ 700000e\ 11.3g.png)
cargo build --release --target=thumbv6m-none-eabi
(cd build; make )