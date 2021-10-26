use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // Result型が返る
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }    
    let f = File::open("hello.txt");

    match f {
        Ok(file) => file,
        // refは値にマッチし、その参照を返す(&は参照にマッチし、その値を返す)
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };

    // unwrapでResultのOkが返る. ファイルなければpanic!が起きる
    let f = File::open("hello2.txt").unwrap();

    // expectでpanic!のメッセージ指定
    let f = File::open("hello2.txt").expect("Failed to open hello.txt");

    let f = read_username_from_file();
}

// hello2.txtの中身を返す
// 成功時はString,失敗時はio*::Errorが返る
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello2.txt");

    // 成功時はファイルハンドラをfに格納、失敗時はErrが早期リターン
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello2.txt")?;
    let mut s = String::new();
    // fがあればread_to_stringを実行. なければ Err(e)がその時点で返る
    // Rubyでいう&.read_to_string的処理(厳密には違う)
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    // まとめるとこう書ける. 変数も省略できるからスマート
    File.open("hello2.txt")?.read_to_string(&mut s)?;
    OK(s)
}

