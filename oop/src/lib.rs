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
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_averaged_collection() {
        let mut collection = AveragedCollection {
            list: vec![],
            average: 0.0,
        };
        collection.add(5);
        collection.add(10);
        assert_eq!(collection.average(), 7.5);
    }
}
