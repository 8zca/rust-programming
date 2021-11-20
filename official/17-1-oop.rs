pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let res = self.list.pop();
        match res {
            // matchしたものがvalueに入る
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let sum: i32 = self.list.iter().sum();
        self.average = sum as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut avg = AveragedCollection { list: Vec::new(), average: 0.0 };
    avg.add(5);
    avg.add(7);
    println!("{}", avg.average());
}
