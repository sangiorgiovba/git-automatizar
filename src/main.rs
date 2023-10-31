use std::process::{Command, exit};
use names::Generator;

fn upadate_commit_push(){
    let add_command = Command::new("git")
    .arg("add")
    .arg("-A")
    .output()
    .expect("OCORREU UM ERROR AO EXECUTAR O COMMANDO ADD");

    if !add_command.status.success(){
        eprintln!("error: ocorreu um error em adicionar estes arquivos no git");
        exit(1);
    }

    let commit_command = Command::new("git")
    .arg("commit")
    .arg("-m")
    .arg(name_generator())
    .output()
    .expect("OCORREU UM ERROR AO FAZER O COMMIT DO ARQUIVO");

    if !commit_command.status.success(){
        eprintln!("error: ocorreu um error MODIFICAR O arquivos no git");
        exit(1);
    }

    let push_command = Command::new("git")
    .arg("push")
    .arg("origin")
    .arg("master")
    .output()
    .expect("OCORREU UM ERROR AO ENVIAR O ARQUIVO");

    if !commit_command.status.success(){
        eprintln!("error: ocorreu um error ENVIAR O arquivos no git");
        exit(1);
    }

    eprintln!("SUCESSO AO ENVIAR TODOS OS ARQUIVOS PARA O GIT");    
}

fn name_generator()->String{
    let mut generator = Generator::default();
    generator.next().unwrap()
}

fn main(){
    upadate_commit_push();
}