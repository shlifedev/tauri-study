use std::process::Output;

struct Git;

impl Git {
    async fn run_command(work_dir: &str, command: &str) -> (bool, Output) {
        println!("{work_dir}에서 {command} 명령어를 실행합니다.");

        let program = "git";
        let args = command.split_whitespace().collect::<Vec<&str>>();

        let output = tokio::process::Command::new(program)
            .args(&args)
            .current_dir(work_dir)
            .output()
            .await
            .unwrap();

        let has_error = !output.status.success();

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            println!("명령어 실행 결과: {}", result);
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            println!("명령어 실행 오류: {}", error);
        }

        (has_error, output)
    }

    async fn reset_hard(work_dir: &str) {
        println!("{work_dir}의 변경사항을 모두 버립니다.");
        Self::run_command(work_dir, "reset --hard").await;
    }

    async fn check_out(work_dir: &str, branch: &str, discard_all: bool) {
        println!("{work_dir}의 {branch}를 체크아웃합니다.");

        if discard_all {
            Self::reset_hard(work_dir).await;
        }

        Self::run_command(work_dir, &format!("checkout {}", branch)).await;
    }
}