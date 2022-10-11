#[allow(unused_imports)]
use serde_json::{Deserializer, Serializer, Value};
use std::{fs::{File, OpenOptions}, io::{Write, Read, stdin}};

fn main() {
    let mut escolha: i8;
    let mut input: String = String::new();

    loop {
        println!("[ 1 ] Ver dados\n[ 2 ] Adicionar dados\n[ 3 ] Editar dados\n[ 4 ] Excluir dados\n[ 5 ] Sair");

        Input(&mut input);

        escolha = input.parse::<i8>().unwrap();

        match escolha {
            1 => show(),
            2 => add(None),
            3 => edit(),
            4 => delete(),
            5 => break,
            _ => println!("Escolha uma opção válida")
        }
        input.clear();
    }
}

#[allow(non_snake_case)]
fn Input(input: &mut String){
    stdin().read_line(input).unwrap();
    input.pop();
}

fn show() {
    let mut dados: String = String::new();
    let mut f: File = File::open("src/data.json").unwrap();

    f.read_to_string(&mut dados).unwrap();
    
    let json: Value = serde_json::from_str(dados.as_str()).expect("O .json precisa ser inicializado com {}");
    
    for (key, _value) in json.as_object().unwrap(){
        println!("======================================");
        println!("Nome: {}\nIdade: {}\nCor favorita: {}", key, json[key]["idade"], json[key]["cor"]);
        println!("======================================");
    }

}

fn add(valor: Option<String>) {
    let mut f: File = OpenOptions::new().create(true).read(true).write(true).open("src/data.json").unwrap();
    let mut dados: String = String::new();
    let mut input: String = String::new();
    let nome: String;

    f.read_to_string(&mut dados).unwrap();
    let mut json: Value = serde_json::from_str(dados.as_str()).unwrap();

    if valor.is_none() {
        println!("Nome: ");
        Input(&mut input);

        nome = String::from(&input);
        input.clear();
    }
    else {
        nome = valor.unwrap();
    }

    println!("Idade: ");
    Input(&mut input);
    json[&nome]["idade"] = serde_json::Value::String(String::from(&input));
    input.clear();

    println!("Cor favorita: ");
    Input(&mut input);
    json[nome]["cor"] = serde_json::Value::String(String::from(&input));
    input.clear();

    let mut f: File = OpenOptions::new().write(true).truncate(true).open("src/data.json").unwrap();
    f.write_all(json.to_string().as_bytes()).unwrap();
}

fn edit(){
    let mut nome: String = String::new();

    show();
    println!("Quem queres editar?");

    Input(&mut nome);
    add(Some(nome));
}

fn delete(){
    let mut f: File = OpenOptions::new().create(true).read(true).write(true).open("src/data.json").unwrap();
    let mut dados: String = String::new();
    let mut nome: String = String::new();

    f.read_to_string(&mut dados).unwrap();
    let mut map: serde_json::Map<String, Value> = serde_json::from_str(&dados).unwrap();
    
    println!("Quem queres excluir? ");
    Input(&mut nome);

    map.remove(&nome);
    let json: Value = map.into();

    print!("{}", json);

    let mut f: File = OpenOptions::new().write(true).truncate(true).open("src/data.json").unwrap();
    f.write_all(json.to_string().as_bytes()).unwrap();
}