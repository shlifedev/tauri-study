use std::process::Output;

struct Git{

}

impl Git{
    async fn run_command(work_dir: String, command: String) -> (bool, Output) {
        println!("{work_dir}에서 {command} 명령어를 실행합니다.");
        let program = "git";
        let args = command.split_whitespace().collect::<Vec<&str>>();
        let output = std::process::Command::new(program).args(&args).output().unwrap();
        let mut has_error = false;
        if(output.status.success()){
            let result = String::from_utf8_lossy(&output.stdout);
            println!("명령어 실행 결과: {}", result);
            has_error = true;
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            println!("명령어 실행 오류: {}", error);
        }
        return (has_error, output);
    }

    fn resetHard(workDir: String){
        println!("{workDir}의 변경사항을 모두 버립니다.");
        Self::run_command(workDir, format!("reset --hard -f"));
    }
    fn checkOut(workDir: String, branch: String, discardAll: bool){
        println!("{workDir}의 {branch} 를 체크아웃합니다.");

        // 리셋부터 실행하는게 나을듯
        if(discardAll){
           Self::resetHard(workDir);
        }
    }
}
