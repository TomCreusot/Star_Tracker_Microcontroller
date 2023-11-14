(cd ../star_tracker_nix; cargo run --bin gen_database ../star_tracker_embed/src/flash/database.rs ../star_tracker_embed/database.json)
(cd ../star_tracker_nix; cargo run --bin gen_image ../star_tracker_embed/src/flash/img.rs 400 400 samples/16mm_checker_2/Mula/mula\ 700000e\ 11.3g.png)
cargo embed
