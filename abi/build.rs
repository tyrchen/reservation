use std::{fs, process::Command};

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .type_attribute(
            "reservation.ReservationStatus",
            "#[derive(sqlx::Type)]
        ",
        )
        .compile(&["protos/reservation.proto"], &["protos"])
        .unwrap();

    fs::remove_file("src/pb/google.protobuf.rs").unwrap();

    Command::new("cargo").args(&["fmt"]).output().unwrap();

    println!("cargo:rerun-if-changed=protos/reservation.proto");
}
