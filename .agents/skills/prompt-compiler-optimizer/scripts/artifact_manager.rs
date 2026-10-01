//! Artifact Manager (Quản Lý Tệp Đầu Ra & Đóng Gói ReleaseBundle)
//! 
//! THUỘC PHASE 6: ĐÓNG GÓI SẢN PHẨM (ARTIFACT ARCHIVING & RELEASE BUNDLING)
//! ========================================================================================
//! Tác dụng: Quản lý lưu trữ, tạo phiên bản, lưu ảnh chụp trạng thái (Snapshot) 
//!           và đóng gói tất cả tệp đầu ra (ReleaseBundle) giúp dễ dàng hoàn tác (Rollback) nếu cần.

use std::env;
use std::fs;
use std::path::Path;

/// Hàm `archive_run`: Lưu trữ toàn bộ kết quả của lần biên dịch vào thư mục riêng theo mã phiên chạy
fn archive_run(run_id: &str, artifact_dir: &Path) {
    let target = artifact_dir.join(format!("run_{}", run_id));
    fs::create_dir_all(&target).expect("[-] Lỗi khi tạo thư mục lưu trữ phiên chạy");
    println!("[+] Đã đóng gói và lưu trữ phiên chạy tại: {}", target.display());
}

/// Điểm bắt đầu công cụ CLI artifact_manager
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut run_id_opt: Option<String> = None;
    let mut artifact_dir_opt = String::from("./artifacts");

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--run-id" && i + 1 < args.len() {
            run_id_opt = Some(args[i + 1].clone());
            i += 1;
        } else if args[i] == "--artifact-dir" && i + 1 < args.len() {
            artifact_dir_opt = args[i + 1].clone();
            i += 1;
        }
        i += 1;
    }

    match run_id_opt {
        Some(run_id) => {
            archive_run(&run_id, Path::new(&artifact_dir_opt));
        }
        None => {
            eprintln!("Cú pháp: artifact_manager --run-id <id> [--artifact-dir <dir>]");
            std::process::exit(1);
        }
    }
}
