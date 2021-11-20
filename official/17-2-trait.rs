trait Draw {
    fn draw(&self);
}

struct Screen {
    // Box<T>はヒープにメモリを確保する汎用型であることを思い出そう
    // Drawの実装は別途されるのでこの段階では正確なサイズがわからない(Draw I/Fをもつオブジェクトのベクタであればよい)
    // そのためBoxをかませる必要がある
    components: Vec<Box<Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/**
 * パーツその1
 */
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

/**
 * パーツその2
 */
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    // はい
                    String::from("Yes"),
                    // 多分
                    String::from("Maybe"),
                    // いいえ
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                // 了解
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
