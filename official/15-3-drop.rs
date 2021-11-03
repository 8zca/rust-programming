struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // デストラクタ的な処理
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };      // 俺のもの
    let d = CustomSmartPointer { data: String::from("other stuff") };   // 別のもの
    println!("CustomSmartPointers created.");                           // CustomSmartPointerが生成された
}
